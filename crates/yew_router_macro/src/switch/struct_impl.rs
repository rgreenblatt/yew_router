use crate::switch::SwitchItem;
use proc_macro2::Ident;
use quote::quote;
use syn::export::{TokenStream, TokenStream2};
use syn::{Field, Fields, Type};

pub fn generate_struct_impl(item: SwitchItem) -> TokenStream {
    let SwitchItem {
        matcher,
        ident,
        fields,
    } = item;
    let build_from_captures = build_variant_from_captures(&ident, fields);
    let matcher = super::build_matcher_from_tokens(matcher);

    let token_stream = quote! {
        impl ::yew_router::Switch for #ident {
            fn from_route_part<T: ::yew_router::route::RouteState>(route: ::yew_router::route::Route<T>) -> (Option<Self>, Option<T>) {

                #matcher
                let mut state = route.state;
                let route_string = route.route;

                #build_from_captures

                return (None, state)
            }

            fn build_route_section<T>(self, route: &mut String) -> Option<T> {
                unimplemented!()
            }
        }
    };
    TokenStream::from(token_stream)
}

fn build_variant_from_captures(ident: &Ident, fields: Fields) -> TokenStream2 {
    match fields {
        Fields::Named(named_fields) => {
            let fields: Vec<TokenStream2> = named_fields.named.into_iter()
                .filter_map(|field: Field| {
                    let field_ty: Type = field.ty;
                    field.ident.map(|i| {
                        let key = i.to_string();
                        (i, key, field_ty)
                    })
                })
                .map(|(field_name, key, field_ty): (Ident, String, Type)|{
                    quote!{
                        #field_name: {
                            let (v, s) = match captures.remove(#key) {
                                Some(value) => {
                                    <#field_ty as ::yew_router::RouteItem>::from_route_part(
                                        ::yew_router::route::Route {
                                            route: value,
                                            state,
                                        }
                                    )
                                }
                                None => {
                                    (
                                        <#field_ty as ::yew_router::RouteItem>::key_not_available(),
                                        state,
                                    )
                                }
                            };
                            match v {
                                Some(val) => {
                                    state = s; // Set state for the next var.
                                    val
                                },
                                None => return (None, s) // Failed
                            }
                        }
                    }
                })
                .collect();

            return quote! {
                if let Some(mut captures) = matcher.capture_route_into_map(&route_string).ok().map(|x| x.1) {
                    return (
                        Some(
                            #ident {
                                #(#fields),*
                            }
                        ),
                        state
                    );
                };
            };
        }
        Fields::Unnamed(unnamed_fields) => {
            let fields =
                unnamed_fields
                    .unnamed
                    .iter()
                    .map(| f: &Field| {
                        let field_ty = &f.ty;
                        quote! {
                                {
                                    let (v, s) = match drain.next() {
                                        Some((_key, value)) => {
                                            <#field_ty as ::yew_router::RouteItem>::from_route_part(
                                                ::yew_router::route::Route {
                                                    route: value,
                                                    state,
                                                }
                                            )
                                        },
                                        None => {
                                            (
                                                <#field_ty as ::yew_router::RouteItem>::key_not_available(),
                                                state,
                                            )
                                        }
                                    };
                                    match v {
                                        Some(val) => {
                                            state = s; // Set state for the next var.
                                            val
                                        },
                                        None => return (None, s) // Failed
                                    }
                                }
                            }
                    });



            quote! {
                // TODO put an annotation here allowing unused muts.
                if let Some(mut captures) = matcher.capture_route_into_vec(&route_string).ok().map(|x| x.1) {
                    let mut drain = captures.drain(..);
                    return (
                        Some(
                            #ident(
                                #(#fields),*
                            )
                        ),
                        state
                    );
                };
            }

        }
        Fields::Unit => {
            return quote! {
                let mut state = if let Some(_captures) = matcher.capture_route_into_map(&route_string).ok().map(|x| x.1) {
                    return (Some(#ident), state);
                } else {
                    state
                };
            }
        }
    }
}
