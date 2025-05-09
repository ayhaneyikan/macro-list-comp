// comprehension: mapping for_if_clause+
// mapping: expression
// for_if_clause: 'for' pattern 'in' expression ('if' expression)*
// pattern: name (, name)*

use syn::{parse::{Parse, ParseStream}, parse_macro_input};
use quote::{quote, ToTokens};
use proc_macro2::TokenStream as TokenStream2;


struct Comp {
    mapping: Mapping,
    for_if_clause: ForIfClause,
}

impl Parse for Comp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapping: input.parse()?,
            for_if_clause: input.parse()?,
        })
    }
}

// Using `quote` crate to convert intermediate representation into an output TokenStream
// `quote` provides the `quote!` macro which lets you define rust code that the crate converts into an equivalent TokenStream
// We'll use this macro to define the shape of the rust Tokens we want, and output it so the compiler knows how to expand our macro at compile-time
impl ToTokens for Comp {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        // unpack representation
        let Mapping(mapping) = &self.mapping;
        let ForIfClause {
            pattern,
            iterable,
            conditions,
        } = &self.for_if_clause;

        // convert each condition into its token representation
        let conditions = conditions.iter().map(|c| {
            let inner = &c.0;
            quote! { #inner }
        });

        // use quote to create the token representation of the list comprehension
        tokens.extend(quote! {
            ::core::iter::IntoIterator::into_iter(#iterable).flat_map(move |#pattern| {
                (true #(&& (#conditions))*).then(|| #mapping)
            })
        });
    }
}


// Using `syn` crate for representing Rust types
struct Mapping(syn::Expr);

impl Parse for Mapping {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Result returned by `parse()` is propegated if Err, but value is converted to self if Ok
        // E.g., `Ok(v) becomes Ok(Self(v))`, turning it into a Mapping
        input.parse().map(Self)
    }
}

impl ToTokens for Mapping {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}


struct ForIfClause {
    pattern: Pattern,
    iterable: syn::Expr,
    // optional: zero or more conditions
    conditions: Vec<Condition>,
}

impl Parse for ForIfClause {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // parse `for`, this parse should fail if no `for ` exists
        let _: syn::Token![for] = input.parse()?;
        // parse iterator pattern
        let pattern = input.parse()?;
        // parse `in`
        let _: syn::Token![in] = input.parse()?;
        // parse iterable expression
        let iterable = input.parse()?;

        // optionally, parse filter conditions while there are valid conditions
        let conditions = parse_zero_or_more(input);

        Ok(Self {
            pattern,
            iterable,
            conditions,
        })
    }
}

/// Iterates over a ParseStream and parses T from the stream until unable to do
/// so, returning successfully parsed values in a Vector
fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut output = Vec::new();
    while let Ok(value) = input.parse() {
        output.push(value);
    }
    output
}


// Using `syn`'s representation of Rust patterns
struct Pattern(syn::Pat);

impl Parse for Pattern {
    // `ParseStream` is conceptually a queue of tokens; matching and consuming elements sequentially
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // see Mapping impl for explanation of `map()` usage
        syn::Pat::parse_single(input).map(Self)
    }
}

impl ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}


struct Condition(syn::Expr);

impl Parse for Condition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // parse `if` , this parse should fail if no `if` exists
        // unused if parse is successful
        let _: syn::Token![if] = input.parse()?;
        // parse the condition expression
        input.parse().map(Self)
    }
}


// create actual macro `comp!`
#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // parse input as intermediate representation
    let c = parse_macro_input!(input as Comp);

    // convert repr to TokenStream2 then TokenStream
    quote! { #c }.into()
}