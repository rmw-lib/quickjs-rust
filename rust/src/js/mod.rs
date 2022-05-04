mod val;
pub(crate) use val::{val, Val};

pub mod arg;

use const_cstr::const_cstr;
use quickjs_ffi::{
  JSContext, JSValue, JS_Call, JS_DefinePropertyValueStr, JS_NewError, JS_ThrowInternalError,
  JS_PROP_CONFIGURABLE, JS_PROP_WRITABLE, JS_UNDEFINED,
};
use std::{ffi::CString, fmt::Display};

pub(crate) fn call<const N: usize>(
  ctx: *mut JSContext,
  func: JSValue,
  argv: &mut [JSValue; N],
) -> JSValue {
  unsafe {
    JS_Call(
      ctx,
      func,
      JS_UNDEFINED, //this
      argv.len() as _,
      argv as _,
    )
  }
}

const_cstr! {
  MESSAGE = "message";
}

pub(crate) fn error<S: Display>(ctx: *mut JSContext, err: S) -> JSValue {
  unsafe {
    let r = JS_NewError(ctx);
    let err = CString::new(err.to_string()).unwrap();
    JS_DefinePropertyValueStr(
      ctx,
      r,
      MESSAGE.as_ptr(),
      val(ctx, err),
      (JS_PROP_WRITABLE | JS_PROP_CONFIGURABLE) as _,
    );
    r
  }
}

pub(crate) fn throw<T, S: Into<Vec<u8>>>(ctx: *mut JSContext, err: S) -> Result<T, JSValue> {
  let err = CString::new(err).unwrap();
  Err(unsafe { JS_ThrowInternalError(ctx, err.as_ptr()) })
}
