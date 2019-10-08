use proc_macro::{TokenStream};
use syn::token::Enum;
use syn::{Data, DeriveInput, Ident, Variant, Attribute, Meta, MetaNameValue, Lit, Fields};
use syn::parse_macro_input;
use syn::punctuated::IntoIter;
use quote::quote;
use proc_macro2::TokenStream as TokenStream2;


const ATTRIBUTE_TOKEN_STRING: &str = "to";

pub fn switch_impl(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let enum_ident: Ident = input.ident;

    let variants: IntoIter<Variant> = match input.data {
        Data::Struct(ds) => {
            panic!("Deriving Switch not supported for Structs.")
        }
        Data::Enum(de) => {
            de.variants.into_iter()
        }
        Data::Union(_du) => {
            panic!("Deriving FromCaptures not supported for Unions.")
        }
    };

    let switch_variants: Vec<SwitchVariant> = variants
        .map(|variant: Variant| {
            SwitchVariant {
                route_string: get_route_string(variant.attrs),
                ident: variant.ident,
                fields: variant.fields
            }
        })
        .collect();

    generate_trait_impl(enum_ident, switch_variants)

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
       .unwrap_or_else(|| panic!(r##"The Switch derive function expects all variants to be annotated with [{} = "/route/string"] "##, ATTRIBUTE_TOKEN_STRING))
}


pub struct SwitchVariant {
    route_string: String,
    ident: Ident,
    fields: Fields
}


fn generate_trait_impl(enum_ident: Ident, switch_variants: Vec<SwitchVariant>) -> TokenStream {

    let variant_matchers: Vec<TokenStream2> = switch_variants.into_iter()
        .map(|sv| {
            let SwitchVariant {
                route_string, ident, fields
            } = sv;
            quote! {
                let matcher = ::yew_router::matcher::route_matcher::RouteMatcher::try_from(#route_string).expect("Invalid matcher");
                let matcher = Matcher::from(matcher);
                if let Some(captures) = matcher.match_route_string(&route.to_string()) {
                    unimplemented!()
                    // TODO return enum instance;
                    // TODO, this will require specific versions for each enum type: unit, struct, tuple.
                }

            }
        })
        .collect::<Vec<_>>();


    let token_stream = quote! {
        impl ::yew_router::switch::Switch for #enum_ident {
            fn switch<T>(route: RouteInfo<T>) -> Self {
                #(#variant_matchers)*
            }
        }
    };
    TokenStream::from(token_stream)
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
