extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn test_csv(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as syn::ItemFn);
    let file = parse_macro_input!(args as syn::LitStr);

    let func_name = &func.sig.ident;
    let args: Vec<_> = func.sig.inputs.iter().collect();
    if args.len() != 1 {
        return syn::Error::new_spanned(func.sig.inputs, "Require a single argument")
            .to_compile_error()
            .into();
    }
    let body = &func.block;

    let out = quote! {
        #[test]
        fn #func_name() {
            let bytes = include_bytes!(#file);
            let mut rdr = csv::ReaderBuilder::new()
                .comment(Some(b'#'))
                .has_headers(false)
                .from_reader(&bytes[..]);
            let mut rdr = rdr.deserialize();
            while let Some(value) = rdr.next() {
                let #(#args)* = value.unwrap();
                let line_no = || {
                    rdr.reader().position().line() - 1
                };
                #body
            }
        }
    };
    out.into()
}
