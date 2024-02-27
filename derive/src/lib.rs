extern crate proc_macro;

use proc_macro2;

#[proc_macro_derive(SmtDatatype)]
pub fn smt_datatype(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    // apparently you can parse the struct fields with the syn or darling crate. syn seems to be a
    // bit more low-level.

    let output: proc_macro2::TokenStream = {
        for i in input {
            println!("{i:?}");
        }

        let foo: Vec<proc_macro2::TokenTree> = Vec::new();
        proc_macro2::TokenStream::from_iter(foo)
    };

    proc_macro::TokenStream::from(output)
}

struct Let(smtlib_syntax::terms::VarBinding);

impl syn::parse::Parse for Let {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        //let assignment: syn::Local = input.parse()?;

        let kw_let: syn::token::Let = input.parse()?;
        let name: syn::Ident = input.parse()?;
        let tok_eq: syn::token::Eq = input.parse()?;
        let expr: syn::Expr = input.parse()?;

        todo!()
    }
}

#[proc_macro]
pub fn let_term(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}
