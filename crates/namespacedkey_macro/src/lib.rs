use namespacedkey_core::Identifier;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Ident, LitStr, Token, Type, Visibility,
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Comma,
};

struct Entry {
    vis: Visibility,
    ident: Ident,
    value: LitStr,
}

/// Macro input: an optional `T` followed by one or more `Entry` definitions.
struct MacroInput {
    ty: Option<Type>,
    entries: Vec<Entry>,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty = {
            let fork = input.fork();
            if fork.parse::<Type>().is_ok() && fork.peek(Token![;]) {
                // Consume it from the real input
                let ty: Type = input.parse()?;
                input.parse::<Token![;]>()?;
                Some(ty)
            } else {
                None
            }
        };

        let mut entries = Vec::new();
        while !input.is_empty() {
            let vis: Visibility = if input.peek(Token![pub]) {
                input.parse()?
            } else {
                Visibility::Inherited
            };

            let ident: Ident = input.parse()?;
            input.parse::<Token![=>]>()?;
            let value: LitStr = input.parse()?;

            // Validate the literal at compile time
            if let Err(err) = Identifier::<()>::parse(value.value()) {
                return Err(syn::Error::new_spanned(
                    &value,
                    format!("Invalid Identifier: {err}"),
                ));
            }

            entries.push(Entry { vis, ident, value });

            // Consume an optional trailing comma
            if input.peek(Comma) {
                input.parse::<Comma>()?;
            } else {
                break;
            }
        }

        Ok(MacroInput { ty, entries })
    }
}

#[proc_macro]
pub fn define_identifier(input: TokenStream) -> TokenStream {
    let MacroInput { ty, entries } = parse_macro_input!(input as MacroInput);

    // Default to `()` if no type provided.
    let ty = ty.unwrap_or_else(|| syn::parse_quote! { () });

    let fns = entries.into_iter().map(|Entry { vis, ident, value }| {
        let fn_name = format_ident!("id_{}", ident);
        quote! {
            #vis fn #fn_name() -> namespacedkey_core::Identifier<#ty> {
                static ONCE: ::std::sync::OnceLock<namespacedkey_core::Identifier<#ty>> =
                    ::std::sync::OnceLock::new();
                ONCE
                    .get_or_init(|| <namespacedkey_core::Identifier<#ty> as ::core::str::FromStr>::from_str(#value).unwrap())
                    .clone()
            }
        }
    });

    TokenStream::from(quote! {
        #( #fns )*
    })
}
