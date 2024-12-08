//! # altgetset
//!
//! An alternative library to [`getset`](https://crates.io/crates/getset).
//!
//! This library provide procedural macros to write getters and setters for struct fields.
//!
//! This macro is not intended for fields that requires custom logic inside their getters and
//! setters. Not yet. :)

mod expand;
use expand::*;
pub(crate) mod utils;

use proc_macro::TokenStream;
use proc_macro_error2::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};
use utils::parse_global_attr;

#[proc_macro_derive(Getter, attributes(get, getset))]
#[proc_macro_error]
pub fn getter(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let params = Params::new(Mode::Get, parse_global_attr(&ast.attrs, Mode::Get));

    produce(&ast, &params).into()
}

#[proc_macro_derive(GetterClone, attributes(get_clone, getset))]
#[proc_macro_error]
pub fn getter_clone(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let params = Params::new(
        Mode::GetClone,
        parse_global_attr(&ast.attrs, Mode::GetClone),
    );

    produce(&ast, &params).into()
}

#[proc_macro_derive(GetterMut, attributes(get_mut, getset))]
#[proc_macro_error]
pub fn getter_mut(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let params = Params::new(Mode::GetMut, parse_global_attr(&ast.attrs, Mode::GetMut));

    produce(&ast, &params).into()
}

#[proc_macro_derive(Setter, attributes(set, getset))]
#[proc_macro_error]
pub fn setter(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let params = Params::new(Mode::Set, parse_global_attr(&ast.attrs, Mode::Set));

    produce(&ast, &params).into()
}
