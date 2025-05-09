// comprehension: mapping for_if_clause+
//
// mapping: expression
//
// for_if_clause: 'for' pattern 'in' expression ('if' expression)*
//
// pattern: name (, name)*

use syn::parse::Parse;

struct Comp {
    mapping: Mapping,
    for_if_clause: ForIfClause,
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
    expr: syn::Expr,
    // optional: zero or more conditions
    condition: Vec<Condition>,
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