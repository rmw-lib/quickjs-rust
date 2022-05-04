use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn js(_attr: TokenStream, input: TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::ItemFn);
  let syn::ItemFn {
    attrs,
    vis,
    block,
    sig,
  } = input;

  let syn::Signature {
    output: return_type,
    inputs: params,
    unsafety,
    asyncness,
    constness,
    abi,
    ident,
    generics: syn::Generics {
      params: gen_params,
      where_clause,
      ..
    },
    ..
  } = sig;

  let args_len = params.len();

  let varname = ident.to_string() + "_args_len";
  let args_len_var = syn::Ident::new(&varname, ident.span());

  quote::quote!(
      #(#attrs) *
      #vis #constness #unsafety #asyncness #abi fn #ident<#gen_params>(#params) #return_type
      #where_clause
      {
          #block
      }
      #[no_mangle]
      #[allow(non_upper_case_globals)]
      pub const #args_len_var:i32 = #args_len as i32;
  )
  .into()
}
