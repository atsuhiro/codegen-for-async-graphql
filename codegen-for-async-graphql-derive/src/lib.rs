extern crate proc_macro;

use codegen_for_async_graphql_template::{generate_token_from_path, Config};
use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn DynSchema(args: TokenStream, input: TokenStream) -> TokenStream {
    let path: &str = &format!("{}", args).replace("\"", "");
    let config = Config {
        output_bnase_path: "./".to_string(),
    };

    let mut res: Option<proc_macro2::TokenStream> = None;

    let streams: Vec<proc_macro2::TokenStream> = generate_token_from_path(path, config);
    streams.iter().for_each(|f| res = Some(f.clone()));
    match res {
        Some(f) => {
            let i = proc_macro2::TokenStream::from(input);
            quote!(
                #i
                #f
            )
            .into()
        }
        _ => panic!("Not implemented"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
