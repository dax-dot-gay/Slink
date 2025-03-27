use darling::{Error, FromMeta, ast::NestedMeta};
use humantime::parse_duration;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    FnArg, ItemFn,
    parse::{Parse, Parser as _},
};

#[derive(Clone, Debug, FromMeta)]
struct CacheArgs {
    pub key: String,

    #[darling(default)]
    pub life_time: Option<String>,

    #[darling(default)]
    pub idle_time: Option<String>,
}

pub(crate) fn handle_cache(_args: TokenStream, _input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(_args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };
    let input = syn::parse_macro_input!(_input as ItemFn);

    let args = match CacheArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let other_attrs = input.attrs.clone();
    let mut signature = input.sig.clone();
    let new_arg =
        match FnArg::parse.parse2(quote! {_cache_state: slink_common::utilities::ResponseCache}) {
            Ok(arg) => arg,
            Err(e) => {
                return e.to_compile_error().into();
            }
        };
    signature.inputs.push(new_arg);
    let visibility = input.vis.clone();
    let body = input.block.clone();
    let ttl = if let Some(lifetime) = args.life_time {
        let duration = match parse_duration(&lifetime) {
            Ok(d) => d,
            Err(e) => {
                return TokenStream::from(darling::Error::custom(e.to_string()).write_errors());
            }
        };
        let secs = duration.as_secs();
        quote! {Some(chrono::TimeDelta::seconds(#secs))}
    } else {
        quote! {None}
    };

    let til = if let Some(lifetime) = args.idle_time {
        let duration = match parse_duration(&lifetime) {
            Ok(d) => d,
            Err(e) => {
                return TokenStream::from(darling::Error::custom(e.to_string()).write_errors());
            }
        };
        let secs = duration.as_secs();
        quote! {Some(chrono::TimeDelta::seconds(#secs))}
    } else {
        quote! {None}
    };

    let key = args.key.as_str();

    quote! {
        #(#other_attrs)*
        #visibility #signature {
            _cache_state.cache_request(format!(#key), async move #body, Some(slink_common::utilities::Expiration {
                lifetime: #ttl,
                idletime: #til
            })).await
        }
    }.into()
}
