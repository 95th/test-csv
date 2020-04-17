extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

macro_rules! bail {
    ($span_src: expr, $msg: expr) => {
        return Err(syn::Error::new_spanned($span_src, $msg));
    };
}

#[proc_macro_attribute]
pub fn test_csv(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as syn::ItemFn);
    let file = parse_macro_input!(args as syn::LitStr);

    to_tokens(func, file)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn to_tokens(func: syn::ItemFn, file: syn::LitStr) -> syn::Result<proc_macro2::TokenStream> {
    let func_name = &func.sig.ident;
    let args: Vec<_> = func.sig.inputs.iter().collect();
    if args.len() != 1 {
        bail!(func.sig.inputs, "Require a single argument");
    }

    if let syn::ReturnType::Type(_, _) = &func.sig.output {
        bail!(func.sig.output, "Return type is not allowed");
    }

    if func.sig.asyncness.is_some() {
        bail!(func.sig.asyncness, "async functions are not allowed");
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

    Ok(out)
}
