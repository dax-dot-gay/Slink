use cache::handle_cache;
use proc_macro::TokenStream;

mod cache;

#[proc_macro_attribute]
pub fn cache(args: TokenStream, input: TokenStream) -> TokenStream {
    handle_cache(args, input)
}