extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn test_csv(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as syn::ItemFn);
    let file = parse_macro_input!(args as syn::LitStr);

    let func_name = &func.sig.ident;
    let args = func.sig.inputs.iter();
    let body = &func.block;

    let out = quote! {
        #[test]
        fn #func_name() {
            let mut rdr = csv::ReaderBuilder::new()
                .comment(Some(b'#'))
                .has_headers(false)
                .from_path(#file)
                .unwrap();
            let mut rdr = rdr.deserialize();
            while let Some(value) = rdr.next() {
                let #(#args)* = value.unwrap();
                #body
            }
        }
    };
    out.into()
}
