use std::{convert, env, error::Error, fs, path::Path};

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, TokenStreamExt};

use rust_fit::fit_record::read_fit;
use rust_fit::profile_types::{convert_types, read_types, FitType};
use syn::LitInt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod fit_blah {
    pub struct FitBlah(i32);
    pub const FIT_BLAH_0: FitBlah = FitBlah(0);
    pub const FIT_BLAH_1: FitBlah = FitBlah(1);
}

fn generate_code_type(fit_type: &FitType) -> TokenStream {
    let enum_name = Ident::new(&fit_type.name, Span::call_site());

    let value_names = fit_type
        .values
        .iter()
        .map(|x| Ident::new(&format!("{}_{}", fit_type.name, x.name), Span::call_site()));

    let values = fit_type
        .values
        .iter()
        .map(|x| LitInt::new(&x.value, Span::call_site()));

    let mut enum_values = vec![];

    enum_values.push(quote! {
        #( #value_names = #values ),*
    });

    let token = quote! {
        enum #enum_name {
            #(#enum_values),*
        }
    };

    token
}

fn generate_code_types(fit_types: &[FitType]) {
    // let names = vec![
    //     Ident::new("one", Span::call_site()),
    //     Ident::new("two", Span::call_site()),
    //     Ident::new("four", Span::call_site())
    // ];
    // let numbers = vec![1i8, 2i8, 4i8];

    // let mut enums = vec![];
    // let enum_name = Ident::new("Stuff", Span::call_site());

    // enums.push(quote! {
    //     #( #names = #numbers ),*
    // });

    // let enum_token = quote! {
    //     enum #enum_name {
    //         #(#enums),*
    //     }
    // };

    let tokens: Vec<TokenStream> = fit_types.iter().map(generate_code_type).collect();
    println!("{}", tokens[0]);

    let mut token = TokenStream::new();
    token.append_all(tokens);

    // let token = quote! {
    //     #(#tokens),*
    // };

    println!("{}", token);

    let syntax_tree = syn::parse2(token).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    let dest_path = Path::new("./").join("out.rs");
    fs::write(dest_path, formatted).unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    // let types_csv = read_types("./fit-profile-types.csv")?;
    // let types = convert_types(&types_csv)?;
    // generate_code_types(&types);

    // for t in &types {
    //     println!("{:?}", t);
    // }

    let args: Vec<String> = env::args().collect();
    
    let file = File::open(&args[1])?;
    let mut reader = BufReader::new(file);

    let fit = read_fit(&mut reader)?;

    let json = serde_json::to_string(&fit)?;

    println!("{}", json);

    Ok(())
}
