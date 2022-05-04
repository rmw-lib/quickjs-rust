static const JSCFunctionListEntry js_rust_funcs[] = {
  % for fn in li:
  JS_RUSTFUNC_DEF(${fn|n}),
  % endfor
};
