#ifndef CBINDGEN_BINDING_RUST_H
#define CBINDGEN_BINDING_RUST_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define fib_args_len (int32_t)1

#define sleep_args_len (int32_t)1

JSValue js_fib(JSContext *ctx, JSValue _this, int argc, JSValue *argv);

JSValue js_sleep(JSContext *ctx, JSValue _this, int argc, JSValue *argv);

void rust_init(JSContext *ctx, JSRuntime *rt);

bool rust_run(JSRuntime *rt);

bool rust_ended(JSContext *ctx);

void rust_ctx_exit(JSContext *ctx, JSRuntime *rt);

void rust_rt_exit(JSRuntime *rt);

#endif /* CBINDGEN_BINDING_RUST_H */
