use heck::*;
use std::{fs, path::Path};

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, TokenStreamExt};

use crate::profile_types::FitType;
use syn::LitInt;

pub fn convert_primitive_type_name(type_name: &str) -> Option<TokenStream> {
    match type_name {
        "uint8" | "uint8z" | "byte" | "enum" => Some(quote!(u8)),
        "sint8" => Some(quote!(i8)),
        "uint16" | "uint16z" => Some(quote!(u16)),
        "sint16" => Some(quote!(i16)),
        "uint32" | "uint32z" => Some(quote!(u32)),
        "sint32" => Some(quote!(i32)),
        "float32" => Some(quote!(f32)),
        "uint64" | "uint64z" => Some(quote!(u64)),
        "sint64" => Some(quote!(i64)),
        "float64" => Some(quote!(f64)),
        _ => None,
    }
}

pub fn generate_enum_code_type(fit_type: &FitType) -> TokenStream {
    let enum_name = Ident::new(&fit_type.name.to_pascal_case(), Span::call_site());

    let value_names = fit_type
        .values
        .iter()
        .map(|x| {
            // deal with names that start with numbers
            let name = match x.name.chars().nth(0).unwrap() {
                digit @ '0'..='9' => {
                    let digit_name = match digit {
                        '1' => "One",
                        '2' => "Two",
                        '3' => "Three",
                        '4' => "Four",
                        '5' => "Five",
                        '6' => "Six",
                        '7' => "Seven",
                        '8' => "Eight",
                        '9' => "Nine",
                        _ => "SomeDigit",
                    };

                    format!("{digit_name}{}", x.name.get(1..).unwrap())
                }
                _ => x.name.clone(),
            };
            Ident::new(&format!("{}", name.to_pascal_case()), Span::call_site())
        })
        .collect::<Vec<Ident>>();

    let values = fit_type
        .values
        .iter()
        .filter(|type_value| !type_value.comment.to_ascii_lowercase().contains("deprecated"))
        .map(|type_value| LitInt::new(&type_value.value.to_pascal_case(), Span::call_site()))
        .collect::<Vec<LitInt>>();

    let mut enum_values = vec![];

    let type_name = convert_primitive_type_name(&fit_type.base_type).unwrap();

    enum_values.push(quote! {
        #( #value_names = #values ),*
    });

    let token = quote! {
        #[repr(#type_name)]
        #[derive(Debug, FromPrimitive)]
        pub enum #enum_name {
            #(#enum_values),*,
            #[num_enum(catch_all)]
            UnknownValue(#type_name),
        }
    };

    token
}

fn generate_primitive_alias(value: &FitType) -> Option<TokenStream> {
    let alias = Ident::new(&value.name.to_pascal_case(), Span::call_site());
    let type_name = convert_primitive_type_name(&value.base_type)?;

    let tokens = quote! {
        pub type #alias = #type_name;
    };

    Some(tokens)
}

pub fn generate_code_type(value: &FitType) -> Option<TokenStream> {
    // making a special case for mesg_num...
    if value.base_type == "enum" || value.name == "mesg_num" {
        Some(generate_enum_code_type(value))
    } else {
        generate_primitive_alias(value)
    }
}

pub fn generate_code_types(fit_types: &[FitType], output_file: &str) {
    let mut tokens: Vec<TokenStream> = vec![];

    tokens.push(quote! {
        /// Auto-generated, do not edit.
        use num_enum::FromPrimitive;
    });

    tokens.extend(fit_types.iter().map(generate_code_type).filter_map(|s| s));

    let mut token = TokenStream::new();

    token.append_all(tokens);

    let syntax_tree = syn::parse2(token).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    let dest_path = Path::new(output_file);
    fs::write(dest_path, formatted).unwrap();
}
