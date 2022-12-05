use proc_macro::TokenStream;
use quote::{format_ident, ToTokens};

const PACKAGE_NAME: &str = "org_automerge_AutomergeSys";

/// A very simple macro that adds `Java_org_automerge_jni_AutomergeSys_` to the function name
#[proc_macro_attribute]
pub fn jni_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn: syn::ItemFn = match syn::parse(item) {
        Ok(i) => i,
        Err(e) => return TokenStream::from(e.into_compile_error()),
    };
    let new_fn_name = format_ident!("Java_{}_{}", PACKAGE_NAME, item_fn.sig.ident);
    item_fn.sig.ident = new_fn_name;
    TokenStream::from(item_fn.to_token_stream())
}
