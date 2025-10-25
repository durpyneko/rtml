//                ____    ______  __  ___  __
//               / __ \  /_  __/ /  |/  / / /
//              / /_/ /   / /   / /|/  / / /
//             / _, _/   / /   / /  / / / /___
//            /_/ |_|   /_/   /_/  /_/ /_____/
//
//   rust macro for generating HTML from a rust-like syntax

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, LitStr, Result, Token, braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct Attribute {
    key: Ident,
    value: LitStr,
}

enum Content {
    Text(LitStr),
    Element(Element),
}

struct Element {
    tag: Ident,
    attributes: Vec<Attribute>,
    children: Vec<Content>,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let value: LitStr = input.parse()?;
        Ok(Attribute { key, value })
    }
}

impl Parse for Content {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            Ok(Content::Text(input.parse()?))
        } else if input.peek(Ident) {
            Ok(Content::Element(input.parse()?))
        } else {
            Err(input.error("Expected a string literal or an element"))
        }
    }
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        let tag: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let mut attributes = Vec::new();
        let mut children = Vec::new();

        while !content.is_empty() {
            if content.peek2(Token![:]) {
                attributes.push(content.parse()?);
            } else {
                children.push(content.parse()?);
            }
        }

        Ok(Element {
            tag,
            attributes,
            children,
        })
    }
}

fn generate_html_from_content(content: &Content, indent: usize) -> String {
    let indentation = " ".repeat(indent * 4);
    match content {
        Content::Text(text) => format!("{}{}\n", indentation, text.value()),
        Content::Element(element) => generate_html(element, indent),
    }
}

fn generate_html(element: &Element, indent: usize) -> String {
    let indentation = " ".repeat(indent * 4);
    let tag = element.tag.to_string().to_lowercase();

    let attrs = element
        .attributes
        .iter()
        .map(|attr| format!(r#" {}="{}""#, attr.key, attr.value.value()))
        .collect::<String>();

    if element.children.is_empty() {
        return format!("{}<{}{}/>\n", indentation, tag, attrs);
    }

    let children = element
        .children
        .iter()
        .map(|child| generate_html_from_content(child, indent + 1))
        .collect::<String>();

    format!(
        "{indentation}<{}{}>\n{children}{indentation}</{}>\n",
        tag, attrs, tag
    )
}

#[proc_macro]
pub fn rtml(input: TokenStream) -> TokenStream {
    let element = parse_macro_input!(input as Element);
    let html_output = generate_html(&element, 0);

    let expanded = quote! {
        #html_output
    };

    TokenStream::from(expanded)
}
