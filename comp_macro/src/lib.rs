// comprehension: mapping for_if_clause+
//
// mapping: expression
//
// for_if_clause: 'for' pattern 'in' expression ('if' expression)*
//
// pattern: name (, name)*

use syn::parse::{Parse, ParseStream};

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


// Using `syn` crate for representing Rust types
struct Mapping(syn::Expr);

impl Parse for Mapping {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Result returned by `parse()` is propegated if Err, but value is converted to self if Ok
        // E.g., `Ok(v) becomes Ok(Self(v))`, turning it into a Mapping
        input.parse().map(Self)
    }
}


struct ForIfClause {
    pattern: Pattern,
    expression: syn::Expr,
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
        let expression = input.parse()?;
        
        // optionally, parse filter conditions while there are valid conditions
        let conditions = parse_zero_or_more(input);

        Ok(Self {
            pattern,
            expression,
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