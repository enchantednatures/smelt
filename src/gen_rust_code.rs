use proc_macro2::TokenStream;
use quote::quote;
use serde_json::Value;

fn generate_rust_code(openapi: &Value) -> TokenStream {
    let definitions = openapi
        .get("definitions")
        .expect("Expected 'definitions' field in OpenAPI spec");

    let mut tokens = TokenStream::new();

    for (name, definition) in definitions.as_object().unwrap() {
        let struct_name = syn::Ident::new(name, proc_macro2::Span::call_site());
        let mut struct_fields = TokenStream::new();

        if let Some(properties) = definition.get("properties").and_then(|p| p.as_object()) {
            for (field_name, field_type) in properties {
                let field_ident = syn::Ident::new(field_name, proc_macro2::Span::call_site());
                let field_type = map_openapi_type_to_rust_type(field_type);

                struct_fields.extend(quote! {
                    #field_ident: #field_type,
                });
            }
        }

        tokens.extend(quote! {
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
            pub struct #struct_name {
                #struct_fields
            }
        });
    }

    tokens
}

fn map_openapi_type_to_rust_type(field_type: &Value) -> TokenStream {
    let openapi_type = field_type
        .get("type")
        .expect("Expected 'type' field in property definition")
        .as_str()
        .expect("Property type should be a string");

    match openapi_type {
        "string" => quote! { String },
        "integer" => quote! { i32 },
        "number" => quote! { f64 },
        "boolean" => quote! { bool },
        "array" => {
            let items = field_type
                .get("items")
                .expect("Expected 'items' field in array property definition");
            let item_type = map_openapi_type_to_rust_type(items);
            quote! { Vec<#item_type> }
        }
        _ => panic!("Unsupported OpenAPI type: {}", openapi_type),
    }
}
