mod gen_rust_code;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, LitStr};

struct OpenApiInput {
    spec: Expr,
}

impl Parse for OpenApiInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let spec = input.parse()?;
        Ok(OpenApiInput { spec })
    }
}

#[proc_macro]
pub fn openapi(input: TokenStream) -> TokenStream {
    let OpenApiInput { spec } = parse_macro_input!(input as OpenApiInput);
    let spec_lit = match &spec {
        Expr::Lit(lit) => &lit.lit,
        _ => panic!("Input must be a string literal"),
    };

    let spec_str = if let LitStr::Verbatim(s) = spec_lit {
        s
    } else {
        panic!("Input must be a string literal");
    };

    let json = if spec_str.starts_with("http://") || spec_str.starts_with("https://") {
        // Fetch the spec from a URL
        reqwest::blocking::get(spec_str)
            .expect("Failed to fetch OpenAPI spec from URL")
            .text()
            .expect("Failed to read OpenAPI spec")
    } else {
        // Read the spec from a file
        std::fs::read_to_string(spec_str).expect("Failed to read OpenAPI spec from file")
    };

    // Parse the OpenAPI JSON spec
    let openapi: serde_json::Value =
        serde_json::from_str(&json).expect("Failed to parse OpenAPI spec");

    // Generate Rust models and API clients (you'll need to implement this function)
    let generated_code = generate_rust_code(&openapi);

    // Return the generated Rust code
    TokenStream::from(quote! {
        #generated_code
    })
}
