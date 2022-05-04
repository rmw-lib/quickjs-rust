# 修改 quickjs 以导入 rust 函数 —— 一种扩展北海的新思路

## 代码库

* [github](https://github.com/rmw-lib/quickjs-rust)
* [gitee](https://gitee.com/rmw-link/quickjs-rust)
* [gitflic](https://gitflic.ru/project/rmw-link/quickjs-rust)
* [bitbucket](https://bitbucket.org/rmw-link/quickjs-rust)

## 起因

[北海](https://openkraken.com) 是基于 `Flutter` 构建高性能 Web 渲染引擎，它使用了 [quickjs](https://github.com/openkraken/kraken/tree/main/bridge/third_party/quickjs)，作为脚本引擎。

我期望用 `rust` 写一些北海的扩展。

北海 [支持用 `dart` 写扩展](https://openkraken.com/guide/advanced/custom-js-api)。

用 [`flutter_rust_bridge`](https://github.com/fzyzcjy/flutter_rust_bridge) 可以打通 `rust` 和 `dart`。

结合以上两点，用 `rust` 写北海的扩展不难。
但，此方案性能开销感觉比较大，因为 `dart` 调用 `rust` 有一次性能损耗，`quickjs` 调用 `dart` 又有一次性能损耗。

另一方面，虽然 `rust` 社区有 [`rquickjs`](https://github.com/DelSkayn/rquickjs) 这种在 `rust` 中调用 `quickjs` 库。
但，它们是调用 `quickjs` 而不是嵌入 `quickjs`，也没法用来魔改 `quickjs` 。

在此代码库中，我实现一种新方案：直接修改 `quickjs` 源代码，使其支持 `rust` 扩展。

这是一个通用的解决方案，不仅仅可以用于修改北海，在所有嵌入了 `quickjs` 框架和库都适用。

## 演示

test.js 代码如下 :

```js
#include ./test.js
```

运行 `./quickjs/qjs test.js`, 输出 :

```
#include ./test.js.out
```

### fib 在 rust 中的实现

js 中导入的 fib 函数来自 `rust/src/export/fib.rs` ，代码如下 :

```rust
#include ./rust/src/export/fib.rs
```

目前，过程宏 `#[js]` 只是添加了一个常量 `fib_args_len`，标识函数的参数个数。

日后，可以进一步编写过程宏 `./rust_macro` 以实现全自动的函数导出。

### 异步函数 sleep 在 rust 中的实现

js 中导入的 sleep 函数来自 `rust/src/export/sleep.rs` ，代码如下 :

```rust
#include ./rust/src/export/sleep.rs
```

从上面可以看到，所有导出的函数都定义在目录 `./rust/src/export` 中，这个目录的 `mod.rs` 会在运行 `./rust/build.xsh` 自动生成，导出其下所有的 `.rs` 文件。

### js 传入参数的读取和校验
参数的读取和校验在 `src/js/arg.rs` 中，代码如下 :

```rust
#include ./rust/src/js/arg.rs
```

目前只提供了参数个数的校验，以及对 i64 类型参数的读取。

可以按需求自行添加，读取函数参见 [qjs_sys](https://docs.rs/qjs-sys/0.1.2/qjs_sys/)  中以 `JS_To` 开头的函数。

### 从 rust 到 js 数据类型转换

类型转换在 `src/js/val.rs` 中，代码如下 :

```rust
#include ./rust/src/js/val.rs
```

目前只定义了 `None`、`()`、`i64`、CString 这 4 种类型到 `js` 的转换，可以根据需要自己添加。

更多数据类型的声明方式可参见 [qjs_sys](https://docs.rs/qjs-sys/0.1.2/qjs_sys/)  中以 `JS_New` 开头的函数。

## 开发环境

我是在苹果笔记本上开发的，rust 用的是 1.62.0-nightly。

先安装 [direnv](https://direnv.net) ，进入目录后，`direnv allow` 一下

安装 python3，然后 `pip3 install -r ./requirements.txt`

运行 `./build.xsh` 即可编译并运行演示

默认是会克隆 quickjs 的官方仓库，如果想修改北海仓库的中的 quickjs，先

`git clone --recursive git@github.com:openkraken/kraken.git --depth=1`

然后进行如下操作

```bash
rm -rf quickjs
ln -s ../kraken/bridge/third_party/quickjs .
```

最后重新运行 `./build.xsh`

## 目录结构

* `./quickjs_rust`
  修改 quickjs 代码的 c 文件

* `./quickjs_ffi`
  导出 `quickjs` 头文件的函数到 `rust`

* `./rust`
  用 `rust` 实现 `quickjs` 中的函数

  * `./rust/src/qjs.rs`
    异步调用的实现。因为 `quickjs` 是单线程的，所以涉及 `quckjs` 函数调用都写在主线程。

* `./rust_macro`
  `rust` 过程宏 `#[js]` 的实现

  未来可以参考 wasmedge-quickjs 实现 rust 函数自动导出为 js 函数。[wasmedge-quickjs → JsFunctionTrampoline](https://github.com/second-state/wasmedge-quickjs/blob/70efe8520dc65636cb81b7225e2a6dae47cfad49/src/quickjs_sys/mod.rs#L122)

## 构建脚本 `build.xsh`

不多说，直接看 `build.xsh` 构建脚本源代码

```xonsh
#include build.xsh
```

## 原理解析

### `quickjs_rust/patch.py`

运行 `./quickjs_rust/patch.py` 会对 `quickjs` 源码做一些小修改。

其中 `JS_AddRust` 是用来注入 rust 模块的函数。

在 `JS_ExecutePendingJob` 中注入了 `rust_run` 来调用异步函数。

全部改动截图如下 :

![](https://raw.githubusercontent.com/gcxfd/img/gh-pages/ep2Xgg.png)

### `quickjs_rust.h`

从上面改动，可以看到，我们引入了一个新的头文件 `quickjs_rust.h` ，其代码如下

```c
#include quickjs_rust/quickjs_rust.h
```

### `rust/rust.h`

可以看到 `quickjs_rust/quickjs_rust.h` 引入了 `quickjs_rust/js_rust_funcs.h`，此是根据 rust 导出函数的头文件 `rust/rust.h` 自动生成的，不要手工修改。

而 `rust/rust.h` 是在 `./rust/build.xsh` 中调用 cbindgen 生成的。

### `rust/build.xsh`

```xonsh
#include rust/build.xsh
```

## 开发备忘

### `quickjs_ffi`

代码来自 [quijine/main/quijine_core/src/ffi.rs](https://raw.githubusercontent.com/taskie/quijine/main/quijine_core/src/ffi.rs)

做了一些小修改，替换

```rust
pub use libquickjs_sys::*;
```

为

```rust
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/c.rs"));
```

### `Undefined symbols for architecture x86_64: "_JS_ToInt32"`

修改 './rust/Cargo.toml' 如下，只保留 staticlib

```toml
[lib]
#crate-type = ["lib", "cdylib", "staticlib"]
crate-type = ["staticlib"]
```

## 参考文献

0. 从 `JS` 引擎到 `JS` 运行时 [(上)](https://github.com/doodlewind/blog/blob/master/source/_posts/js-engine-to-js-runtime-1.md) [（下）](https://github.com/doodlewind/blog/blob/master/source/_posts/js-engine-to-js-runtime-2.md)
0. [使用 C 语言为 `QuickJS` 开发一个原生模块](https://github.com/quickjs-zh/QuickJS/wiki/%E4%BD%BF%E7%94%A8C%E8%AF%AD%E8%A8%80%E4%B8%BAQuickJS%E5%BC%80%E5%8F%91%E4%B8%80%E4%B8%AA%E5%8E%9F%E7%94%9F%E6%A8%A1%E5%9D%97)
0. [Use Rust to implement JS API](https://wasmedge.org/book/en/dev/js/rust.html)
0. [QuickJS examples](https://github.com/Kozova1/quickjs-example)
0. [rust-bindgen](https://rust-lang.github.io/rust-bindgen/)
0. [如何为 `QuickJS` 创建异步代码](https://calbertts.medium.com/how-to-create-asynchronous-apis-for-quickjs-8aca5488bb2e)
0. [rquickjs → JS_NewPromiseCapability](https://github.com/DelSkayn/rquickjs/blob/master/core/src/context/ctx.rs#L104)
0. [wasmedge-quickjs → new_promise](https://github.com/second-state/wasmedge-quickjs/blob/8a65582265ecdd3171380feebf56b3bb8c34d183/src/quickjs_sys/mod.rs#L515)
0. [wasmedge-quickjs → JsMethod](https://github.com/second-state/wasmedge-quickjs/blob/da887752fdc9c36aca0f4b7499c5b115862ce771/src/internal_module/wasi_net_module.rs#L46)
0. [wasmedge-quickjs → call](https://github.com/second-state/wasmedge-quickjs/blob/8a65582265ecdd3171380feebf56b3bb8c34d183/src/quickjs_sys/mod.rs#L756)
0. [不易察觉的陷阱——Rust 中的锁](https://mp.weixin.qq.com/s/BKto24ItwXbeHon_LaF_0w)

## 关于

本项目隶属于 **人民网络 ([rmw.link](//rmw.link))** 代码计划。

![人民网络](https://raw.githubusercontent.com/rmw-link/logo/master/rmw.red.bg.svg)
