use proc_macro2::{Ident, Span, TokenStream as TokStr2};
use proc_macro_error2::{abort, abort_call_site};
use quote::quote;
use syn::{ext::IdentExt, spanned::Spanned, Data, DataStruct, DeriveInput, Field, Meta};

use crate::utils::{parse_attr, parse_visibility};

pub(crate) struct Params {
    pub mode: Mode,
    pub attr: Option<Meta>,
}

impl Params {
    pub(crate) fn new(mode: Mode, attr: Option<Meta>) -> Self {
        Self { mode, attr }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mode {
    Get,
    GetMut,
    GetClone,
    Set,
}

impl Mode {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::GetMut => "get_mut",
            Self::GetClone => "get_clone",
            Self::Set => "set",
        }
    }

    pub(crate) fn ident(&self) -> &'static str {
        match self {
            Self::Get => "Getter",
            Self::GetMut => "MutGetter",
            Self::GetClone => "CloneGetter",
            Self::Set => "Setter",
        }
    }

    pub(crate) fn prefix(&self) -> &'static str {
        match self {
            Mode::Set => "set_",
            _ => "get_",
        }
    }

    pub(crate) fn suffix(&self) -> &'static str {
        match self {
            Mode::GetMut => "_mut",
            Mode::GetClone => "_clone",
            _ => "",
        }
    }

    pub(crate) fn is_get(&self) -> bool {
        !matches!(self, Mode::Set)
    }
}

pub(crate) fn produce(ast: &DeriveInput, params: &Params) -> TokStr2 {
    let name = &ast.ident;
    let generics = &ast.generics;

    let (impl_gen, ty_gen, where_clause) = generics.split_for_impl();

    if let Data::Struct(DataStruct { ref fields, .. }) = ast.data {
        let gen = fields.iter().map(|field| r#impl(field, params));

        quote! {
            impl #impl_gen #name #ty_gen #where_clause {
                #(#gen)*
            }
        }
    } else {
        abort_call_site!(
            "#[derive({})] is only supported on structs.",
            params.mode.ident()
        )
    }
}

fn r#impl(field: &Field, params: &Params) -> TokStr2 {
    let name = if let Some(name) = field.ident.clone() {
        name
    } else {
        abort!(field.span(), "field must have a name")
    };

    let fn_name = Ident::new(
        &format!(
            "{}{}{}",
            params.mode.prefix(),
            name.unraw(),
            params.mode.suffix()
        ),
        Span::call_site(),
    );

    let ty = field.ty.clone();
    let doc = field.attrs.iter().filter(|v| v.meta.path().is_ident("doc"));

    let attr = field
        .attrs
        .iter()
        .filter_map(|v| parse_attr(v, params.mode))
        .last()
        .or_else(|| params.attr.clone());
    let vis = parse_visibility(attr.as_ref(), params.mode.name());

    match attr {
        None => quote! {},
        Some(meta) if meta.path().is_ident("skip") => quote! {},
        Some(_) => match params.mode {
            Mode::Get => {
                quote! {
                    #(#doc)*
                    #[inline(always)]
                    #vis fn #fn_name(&self) -> &#ty {
                        &self.#name
                    }
                }
            }
            Mode::Set => {
                quote! {
                    #(#doc)*
                    #[inline(always)]
                    #vis fn #fn_name(&mut self, val: #ty) -> &mut Self {
                        self.#name = val;
                        self
                    }
                }
            }
            Mode::GetClone => {
                quote! {
                    #(#doc)*
                    #[inline(always)]
                    #vis fn #fn_name(&self) -> #ty {
                        self.#name.clone()
                    }
                }
            }
            Mode::GetMut => {
                quote! {
                    #(#doc)*
                    #[inline(always)]
                    #vis fn #fn_name(&mut self) -> &mut #ty {
                        &mut self.#name
                    }
                }
            }
        },
    }
}
