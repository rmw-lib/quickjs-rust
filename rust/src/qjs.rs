use crate::js;
use anyhow::Result;
use async_channel::{unbounded, Receiver, Sender};
use easy_parallel::Parallel;
use futures_lite::future::block_on;
use once_cell::sync::Lazy;
use parking_lot::{RwLock, RwLockReadGuard};
use quickjs_ffi::{
  JSContext, JSResolveReject, JSRuntime, JSValue, JS_FreeValue, JS_IsException, JS_NewPromise,
};
use smallvec::{smallvec, SmallVec};
use std::{
  collections::HashMap,
  future::Future,
  mem::drop,
  pin::Pin,
  sync::Arc,
  time::{Duration, Instant},
};

pub trait FutureResult = Future<Output = Result<js::Val>> + Sync + Send + 'static;

struct Null();

type Func = Pin<Box<dyn FutureResult>>;

struct Promise {
  pub _lock: RwLockReadGuard<'static, Null>,
  pub resolve_reject: JSResolveReject,
  pub task: Func,
}

unsafe impl Sync for Promise {}
unsafe impl Send for Promise {}

struct Done {
  pub _lock: RwLockReadGuard<'static, Null>,
  pub resolve_reject: JSResolveReject,
  pub val: Result<js::Val>,
}
unsafe impl Sync for Done {}
unsafe impl Send for Done {}

type CtxSender = HashMap<usize, (Sender<Promise>, Arc<RwLock<Null>>)>;
type Recv = Receiver<Done>;

static mut CTX_RECV: Lazy<HashMap<usize, Recv>> = Lazy::new(HashMap::new);
static mut CTX_SEND: Lazy<CtxSender> = Lazy::new(HashMap::new);
static mut RT_CTX: Lazy<HashMap<usize, SmallVec<[usize; 4]>>> = Lazy::new(HashMap::new);

#[no_mangle]
pub extern "C" fn rust_init(ctx: *mut JSContext, rt: *mut JSRuntime) {
  unsafe {
    let ctx = ctx as _;

    let rt = rt as _;
    match RT_CTX.get_mut(&rt) {
      Some(v) => {
        v.push(ctx);
      }
      None => {
        RT_CTX.insert(rt, smallvec![ctx]);
      }
    }

    let (sender, recver) = unbounded();
    let rw = Arc::new(RwLock::new(Null()));
    CTX_SEND.insert(ctx, (sender, rw));

    let (resolve_reject_sender, resolve_reject_recver) = unbounded();
    std::thread::spawn(move || {
      let ex = async_executor::Executor::new();

      Parallel::new()
        .each(0..num_cpus::get(), |_| {
          #[allow(clippy::await_holding_lock)]
          block_on(ex.run(async {
            while let Ok(i) = recver.recv().await {
              drop(
                resolve_reject_sender
                  .send(Done {
                    _lock: i._lock,
                    resolve_reject: i.resolve_reject,
                    val: i.task.await,
                  })
                  .await,
              );
            }
          }));
        })
        .run();
    });
    CTX_RECV.insert(ctx, resolve_reject_recver);
  }
}

#[no_mangle]
pub extern "C" fn rust_run(rt: *mut JSRuntime) -> bool {
  let mut r = false;
  unsafe {
    if let Some(ctx_li) = RT_CTX.get(&(rt as _)) {
      for ctx in ctx_li {
        if rust_ctx_run(*ctx) {
          r = true;
        }
      }
    }
  }
  r
}

#[no_mangle]
pub extern "C" fn rust_ended(ctx: *mut JSContext) -> bool {
  unsafe {
    let ctx_id = ctx as _;
    if let Some((_, lock)) = CTX_SEND.get(&ctx_id) {
      if lock.try_write().is_none() {
        let _lock = lock.try_write_until(Instant::now() + Duration::from_millis(20));
        return false;
      }
    }
  }
  true
}

#[no_mangle]
pub extern "C" fn rust_ctx_exit(ctx: *mut JSContext, rt: *mut JSRuntime) {
  unsafe {
    let ctx = ctx as _;
    ctx_remove(ctx);
    let rt = rt as _;
    if let Some(li) = RT_CTX.get_mut(&rt) {
      if let Some(pos) = li.iter().position(|x| *x == ctx) {
        li.swap_remove(pos);
      }
    }
  }
}

#[no_mangle]
pub extern "C" fn rust_rt_exit(rt: *mut JSRuntime) {
  unsafe {
    let rt = rt as _;
    if let Some(li) = RT_CTX.remove(&rt) {
      for ctx in li {
        ctx_remove(ctx);
      }
    }
  }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn run(
  ctx: *mut JSContext,
  future: impl Future<Output = Result<impl Into<js::Val>>> + Sync + Send + 'static,
) -> JSValue {
  unsafe {
    let p = JS_NewPromise(ctx);
    let promise = p.promise;
    let ctx = ctx as _;
    if JS_IsException(promise) == 0 {
      if let Some((sender, lock)) = CTX_SEND.get(&ctx) {
        drop(block_on(sender.send(Promise {
          _lock: lock.read(),
          resolve_reject: p.resolve_reject,
          task: Box::pin(async { Ok(future.await?.into()) }),
        })));
      }
    }
    promise
  }
}

fn rust_ctx_run(ctx: usize) -> bool {
  let has_recv;
  if let Some(recver) = unsafe { CTX_RECV.get(&ctx) } {
    let ctx = ctx as _;
    let recved = recver.try_recv();

    if let Ok(r) = recved {
      has_recv = true;
      on_recv(ctx, r);
      while let Ok(r) = recver.try_recv() {
        on_recv(ctx, r);
      }
    } else {
      has_recv = false;
    }
  } else {
    has_recv = false;
  };

  has_recv
}

fn on_recv(ctx: *mut JSContext, done: Done) {
  let resolve_reject = done.resolve_reject;
  let func;
  let r;
  let resolve = resolve_reject.resolve;
  let reject = resolve_reject.reject;
  unsafe {
    match done.val {
      Ok(v) => {
        func = resolve;
        r = js::val(ctx, v);
        JS_FreeValue(ctx, reject);
      }
      Err(err) => {
        //参考 https://github.com/HiRoFa/quickjs_es_runtime/blob/3d8b53d6097738c3ac58a0c66ced5e8dd2914270/src/quickjs_utils/errors.rs#L75
        r = js::error(ctx, err);
        func = reject;
        JS_FreeValue(ctx, resolve);
      }
    }
    JS_FreeValue(ctx, js::call(ctx, func, &mut [r]));
    JS_FreeValue(ctx, func);
  }
}

fn ctx_remove(ctx: usize) {
  unsafe {
    CTX_SEND.remove(&ctx);
    CTX_RECV.remove(&ctx);
  }
}
