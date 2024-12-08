use proc_macro2::Span;
use proc_macro_error2::abort;
use syn::{
    parse_str, punctuated::Punctuated, spanned::Spanned, Attribute, Expr, Lit, Meta, MetaNameValue,
    Token, Visibility,
};

use crate::Mode;

pub(crate) fn expr_str(expr: &Expr) -> Option<String> {
    if let Expr::Lit(s) = expr {
        if let Lit::Str(s) = &s.lit {
            return Some(s.value());
        }
    }

    None
}

pub(crate) fn parse_vis(vis: &str, span: Span) -> Visibility {
    match parse_str(vis) {
        Ok(vis) => vis,
        Err(e) => abort!(span, "invalid visibility found {}", e),
    }
}

pub(crate) fn parse_global_attr(attrs: &[Attribute], mode: Mode) -> Option<Meta> {
    attrs
        .iter()
        .filter_map(|attr| parse_attr(attr, mode))
        .last()
}

pub(crate) fn parse_attr(attr: &Attribute, mode: Mode) -> Option<Meta> {
    if attr.path().is_ident("getset") {
        let meta_list = match attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
        {
            Ok(list) => list,
            Err(e) => abort!(attr.span(), "Failed to parse getset attribute: {}", e),
        };

        let (last, skip, mut coll) = meta_list
            .into_iter()
            .inspect(|meta| {
                let known_attr_ident = ["get", "set", "get_mut", "get_clone", "skip"];

                if known_attr_ident
                    .into_iter()
                    .all(|attr| !meta.path().is_ident(attr))
                {
                    abort!(meta.path().span(), "unknown getter / setter attribute")
                }
            })
            .fold(
                (None, None, Vec::new()),
                |(last, skip, mut collected), meta| {
                    if meta.path().is_ident(mode.name()) {
                        (Some(meta), skip, collected)
                    } else if meta.path().is_ident("skip") {
                        (last, Some(meta), collected)
                    } else {
                        collected.push(meta);
                        (last, skip, collected)
                    }
                },
            );

        if let Some(skip) = skip {
            if last.is_none() && coll.is_empty() {
                return Some(skip);
            } else {
                abort!(
                    last.or_else(|| coll.pop()).unwrap().path().span(),
                    "use of getters / setters with skip is invalid"
                );
            }
        } else {
            return last;
        }
    } else if attr.path().is_ident(mode.name()) {
        return attr.meta.clone().into();
    }

    None
}

pub(crate) fn parse_visibility(attr: Option<&Meta>, meta_name: &str) -> Option<Visibility> {
    let meta = attr?;

    let Meta::NameValue(MetaNameValue { value, path, .. }) = meta else {
        return None;
    };

    if !path.is_ident(meta_name) {
        return None;
    }

    let valstr = expr_str(value)?;
    Some(parse_vis(&valstr, value.span()))
}
