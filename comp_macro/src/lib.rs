// comprehension: mapping for_if_clause+
//
// mapping: expression
//
// for_if_clause: 'for' pattern 'in' expression ('if' expression)*
//
// pattern: name (, name)*

struct Comp {
    mapping: Mapping,
    for_if_clause: ForIfClause,
}

// Using `syn` crate for representing Rust types
struct Mapping(syn::Expr);

struct ForIfClause {
    pattern: Pattern,
    expr: syn::Expr,
    // optional: zero or more conditions
    condition: Vec<Condition>,
}

// Using `syn`'s representation of Rust patterns
struct Pattern(syn::Pat);

struct Condition(syn::Expr);