extern crate syn;
#[macro_use] extern crate quote;
extern crate proc_macro2;
extern crate proc_macro_error;
extern crate convert_case;


use proc_macro::{
    TokenStream,
};
use syn::{
    parse_macro_input,
    DeriveInput,
    Ident,
    Field,
    Fields,
    Data,
    export::{
        TokenStream2,
    },
    punctuated::Iter,
    Variant,
    Attribute,
    LitStr,
    parse::Result,
    Meta,
    Lit,
    MetaNameValue,
    Error,
};
use proc_macro_error::{
    abort,
    Diagnostic,
    Level,
    proc_macro_error,
};
use convert_case::{
    Case,
    Casing,
};

#[proc_macro_error]
#[proc_macro_derive(AsPath, attributes(segment_as))]
pub fn derive_as_path(item: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        ..
    } = parse_macro_input!(item as DeriveInput);
    let variants = match data {
        Data::Enum(data) => data.variants,
        _ => abort!(Diagnostic::new(Level::Error, "Can only derive AsPath for enums.".into())),
    };
    let variants = variants.iter();
    let (as_snippets, parse_snippets) = variant_snippets(variants);
    let name = ident.to_string();
    TokenStream::from(quote!{
        impl AsPath for #ident {
            fn as_path(self) -> String {
                match self {
                    #(#as_snippets),*
                }
            }
        }
        impl ParsePath for #ident {
            fn parse_path(path: &str) -> std::result::Result<Self, ParseError> {
                let next = path.trim_start_matches("/");
                Err(ParseError::NoMatch)
                    #(.or_else(|err|
                        #parse_snippets
                        )
                    )*
                    .map_err(|err| ParseError::By(#name.to_string(), Box::new(err)))
            }
        }
    })
}
/// extract path name from attribute
/// #[name = "name"]
fn get_path_from_attribute(attr: &Attribute) -> Result<Option<LitStr>> {
    if !attr.path.is_ident("segment_as") {
        return Ok(None); // not our attribute
    }
    match attr.parse_meta()? {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(name),
            ..
        }) => Some(Some(name)),
        _ => None,
    }
    .ok_or(Error::new_spanned(attr, "expected #[name = \"...\"]"))
}
fn variant_path_segment(ident: Ident, attrs: std::slice::Iter<'_, Attribute>) -> Option<String> {
    let mut attrs = attrs.filter_map(|attr|
        match get_path_from_attribute(attr) {
            Ok(op) => op,
            Err(err) => abort!(Diagnostic::new(Level::Error, err.to_string())),
        }
    );
    let name = if attrs.clone().count() > 1 {
        abort!(Diagnostic::new(Level::Error, "Multiple path names defined.".into()))
    } else if let Some(name) = attrs.next() {
        name.value()
    } else {
        ident.to_string().to_case(Case::Snake)
    };
    if name.to_string().is_empty() {
        None
    } else {
        Some(name)
    }
}
fn variant_snippets(variants: Iter<'_, Variant>) -> (Vec<TokenStream2>, Vec<TokenStream2>) {
    let len = variants.len();
    let snippets = variants
        .enumerate()
        .map(|(i, variant)| {
        let Variant {
            attrs,
            ident,
            fields,
            ..
        } = variant;
        let name = variant_path(ident.clone(), attrs.iter());
        match fields {
            Fields::Unit => {
                if let None = name {
                    if (i+1) != len {
                        abort!(Diagnostic::new(Level::Error, "Unit variant without a name must be declared last.".into()))
                    }
                }
                unit_variant_snippets(ident.clone(), name)
            },
            Fields::Unnamed(fields) => tuple_variant_snippets(ident.clone(), name, fields.unnamed.iter()),
            _ => abort!(Diagnostic::new(Level::Error, "Only unit or single tuple variants allowed.".into()))
        }
    });
    snippets.fold((Vec::with_capacity(len), Vec::with_capacity(len)),
        |mut acc, x| { acc.0.push(x.0); acc.1.push(x.1); acc })
}
fn unit_variant_snippets(ident: Ident, name: Option<String>) -> (TokenStream2, TokenStream2) {
    (
        as_unit_variant(ident.clone(), name.clone()),
        parse_unit_variant(ident, name)
    )
}
fn as_unit_variant(ident: Ident, name: Option<String>) -> TokenStream2 {
    let format = match name {
        Some(name) => quote!{ format!("/{}", #name) },
        None => quote!{ String::new() },
    };
    quote!{
        Self::#ident => #format
    }
}

fn parse_unit_variant(ident: Ident, name: Option<String>) -> TokenStream2 {
    let parser = match name {
        Some(name) => quote!{ 
            next.strip_prefix(#name).ok_or(err)
        },
        None => quote!{ 
            if next.is_empty() {
                Some(())
            } else {
                None
            }
            .ok_or(ParseError::RemainingSegments)
        },
    };
    quote! {
        #parser.map(|_| Self::#ident)
    }
}
fn tuple_variant_snippets(ident: Ident, name: Option<String>, fields: Iter<'_, Field>) -> (TokenStream2, TokenStream2) {
    (
        as_tuple_variant(ident.clone(), name.clone(), fields.clone()),
        parse_tuple_variant(ident, name, fields),
    )
}
fn as_tuple_variant(ident: Ident, name: Option<String>, fields: Iter<'_, Field>) -> TokenStream2 {
    if fields.clone().count() != 1 {
        abort!(Diagnostic::new(Level::Error, "Tuple variants may only have a single field.".into()))
    }
    let format = match name {
        Some(name) => quote!{ format!("/{}{}", #name, nested.as_path()) },
        None => quote!{ nested.as_path() },
    };
    quote!{
        Self::#ident(nested) => #format
    }
}
fn parse_tuple_variant(ident: Ident, name: Option<String>, fields: Iter<'_, Field>) -> TokenStream2 {
    if fields.clone().count() != 1 {
        abort!(Diagnostic::new(Level::Error, "Tuple variants may only have a single field.".into()))
    }
    let parser = match name {
        Some(name) => quote!{ 
            next.strip_prefix(#name).ok_or(err)
                .and_then(|rest|
                    ParsePath::parse_path(rest)
                )
        },
        None => quote!{ 
            ParsePath::parse_path(next)
        },
    };
    quote!{
        #parser.map(Self::#ident)
    }
}
