use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Expr, ExprClosure, Token};

struct StartEnd {
    start: ExprClosure,
    comma: Token![,],
    end: ExprClosure,
}

impl Parse for StartEnd {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            start: input.parse()?,
            comma: input.parse()?,
            end: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn start_end(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as StartEnd);

    let start = input.start;
    

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // ...
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
