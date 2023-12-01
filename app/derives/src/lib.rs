extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn retrial(_args: TokenStream, input: TokenStream) -> TokenStream { 
    let mut item: syn::Item = syn::parse(input).unwrap();

    let fn_item = match &mut item {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("Function is expected.")
    };

    let ItemFn { attrs, vis, sig, block } = fn_item;
    let stmts = &block.stmts;

    TokenStream::from(
        quote! {
            #(#attrs)* #vis #sig {
                let mut counter: u32 = 0;

                let res = while counter < 3 {
                    let res = {
                        #(#stmts)*
                    };

                    if Err(_) = res {
                        continue;
                    };

                    res
                };

                res.unwrap()
            }
        }
    )
}

