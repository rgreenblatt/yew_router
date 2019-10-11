use proc_macro::TokenStream;
//use proc_macro2::TokenStream as TokenStream2;
//use quote::quote;
use syn::parse_macro_input;
//use syn::punctuated::IntoIter;
use syn::{
    Attribute, Data, DeriveInput, Ident, Lit, Meta, MetaNameValue, Variant,
};
use crate::switch::enum_impl::{SwitchVariant, generate_enum_impl};

mod enum_impl;

const ATTRIBUTE_TOKEN_STRING: &str = "to";

pub fn switch_impl(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let enum_ident: Ident = input.ident;

    match input.data {
        Data::Struct(_ds) => panic!("Deriving Switch not supported for Structs."),
        Data::Enum(de) => {
            let switch_variants = de.variants
                .into_iter()
                .map(|variant: Variant| SwitchVariant {
                    route_string: get_route_string(variant.attrs),
                    ident: variant.ident,
                    fields: variant.fields,
                });
            generate_enum_impl(enum_ident, switch_variants)
        }
        Data::Union(_du) => panic!("Deriving FromCaptures not supported for Unions."),
    }
}

/// Gets this section:
/// `#[to = "/route/thing"]`
/// `       ^^^^^^^^^^^^^^`
/// After matching the "to".
fn get_route_string(attributes: Vec<Attribute>) -> String {
    attributes.iter()
        .filter_map(|attr: &Attribute| attr.parse_meta().ok())
        .filter_map(|meta: Meta| {
           match meta {
               Meta::NameValue(x) => Some(x),
               _ => None,
           }
       })
       .filter_map(|mnv: MetaNameValue| {
           mnv.path.clone()
               .get_ident()
               .filter(|ident| ident.to_string() == ATTRIBUTE_TOKEN_STRING.to_string())
               .map(move |_| {
                   match mnv.lit {
                       Lit::Str(s) => Some(s.value()),
                       _ => None
                   }
               })
               .flatten_stable()
       })
       .next()
       .unwrap_or_else(|| panic!(r##"The Switch derive expects all variants to be annotated with [{} = "/route/string"] "##, ATTRIBUTE_TOKEN_STRING))
}




trait Flatten<T> {
    /// Because flatten is a nightly feature. I'm making a new variant of the function here for stable use.
    /// The naming is changed to avoid this getting clobbered when object_flattening 60258 is stabilized.
    fn flatten_stable(self) -> Option<T>;
}

impl<T> Flatten<T> for Option<Option<T>> {
    fn flatten_stable(self) -> Option<T> {
        match self {
            None => None,
            Some(v) => v,
        }
    }
}
