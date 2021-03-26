use proc_macro::{TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{Expr, Ident, ItemFn, Token, parse::{Parse, ParseStream, Result}, punctuated::Punctuated, spanned::Spanned};

#[proc_macro_attribute]
pub fn test_with_parameters(attr: TokenStream, item: TokenStream) -> TokenStream {
    let TableSyntax {
        column_names,
        test_inputs
    } = syn::parse_macro_input!(attr as TableSyntax);
    let test_fn = syn::parse_macro_input!(item as ItemFn);

    if column_names.len() != test_fn.sig.inputs.len() {
        return (quote_spanned! {
            column_names.span() =>
            compile_error!("Number of parameters does not match the test function's arity.");
        }).into()
    }

    for args in test_inputs.iter() {
        if args.len() != test_fn.sig.inputs.len() {
            return (quote_spanned! {
                args.span() =>
                compile_error!("This case has the wrong number of arguments.");
            }).into()
        }
    }

    let cases: Vec<_> = test_inputs.into_iter().enumerate().map(|(idx, args)| {
        let fn_name = format_ident!("{}_case{}", &test_fn.sig.ident, idx);
        let call = &test_fn.sig.ident;
        let args = args.iter();

        let args_splat = quote! {
            #(#args),*
        };

        quote! {
            #[test]
            fn #fn_name() {
                #call(#args_splat)
            }
        }
    }).collect();

    (quote! {
        #test_fn
        #(#cases)*
    }).into()
}

struct TableSyntax {
    column_names: Punctuated<Ident, Token![,]>,
    test_inputs: Vec<Punctuated<Expr, Token![,]>>,
}

impl Parse for TableSyntax {
    fn parse(input: ParseStream) -> Result<Self> {
        let names_input;
        syn::bracketed!(names_input in input);

        let column_names = names_input.parse_terminated(Ident::parse)?;

        let mut test_inputs = vec!();
        while !input.is_empty() {
            let args_input;
            syn::bracketed!(args_input in input);
            let args = args_input.parse_terminated(Expr::parse)?;
            test_inputs.push(args);
         }

        Ok(TableSyntax { column_names, test_inputs })
    }
}