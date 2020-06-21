extern crate proc_macro;

use codegen_for_async_graphql_template::{generate_token_from_path, Config};
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn DynSchema(args: TokenStream, input: TokenStream) -> TokenStream {
    let path = path(&args);
    let config = conf();
    match generate(&path, &config) {
        Some(f) => aggregate(input, &f),
        _ => panic!("Not implemented"),
    }
}

fn conf() -> Config {
    Config {
        output_bnase_path: "./".to_string(),
    }
}

fn generate(path: &str, config: &Config) -> Option<proc_macro2::TokenStream> {
    let mut res: Option<proc_macro2::TokenStream> = None;
    generate_token_from_path(path, config)
        .iter()
        .for_each(|f| res = Some(f.clone()));
    res
}

fn path(args: &TokenStream) -> String {
    format!("{}", args).replace("\"", "")
}

fn aggregate(input: TokenStream, generated: &proc_macro2::TokenStream) -> TokenStream {
    let i = proc_macro2::TokenStream::from(input);
    quote!(
        #i
        #generated
    )
    .into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
