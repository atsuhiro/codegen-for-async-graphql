use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::Save;
use crate::Config;

struct Mod;
impl Save for Mod {}

fn generate_token_stream(names: &[String]) -> TokenStream {
    let mut src = quote!(
        use crate::DataSource;
    );
    names.iter().for_each(|f| {
        let name = Ident::new(f, Span::call_site());
        src = quote!(
          #src
          pub mod #name;
        )
    });
    src
}

pub fn generate_file(names: &[String], config: &Config) {
    let src = generate_token_stream(names);
    let name = "mod".to_string();
    let output_path = &config.output_bnase_path;
    Mod::save(&name, &src.to_string(), output_path);
}
