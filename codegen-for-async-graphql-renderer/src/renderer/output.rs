use proc_macro2::TokenStream;

use super::Context;

pub trait Output {
    fn generate_files(context: &Context);
    fn generate_token_stream(context: &Context) -> Vec<TokenStream>;
}
