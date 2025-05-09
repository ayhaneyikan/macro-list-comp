# Rust List Comprehension Macro

## Overview

A rust macro implementing Python list comprehensions.
Based on the YouTube video by Logan Smith: [Comprehending Proc Macros](https://youtu.be/SMCRQj9Hbx8?si=eGoCg6oOoZJ3Tiy2)

If unfamiliar, Python list comprehensions allow you to iterate over an iterable, while mapping values, and optionally filtering values.

For example we want to go through a list and square every value that is even:
```python
# Python
even_squared = [x*x for x in numbers if x % 2 == 0]
```

With the completed rust macro `comp!`, we can now write the following valid rust:
```rust
// Rust
let even_squared: Vec<_> = comp![x * x for x in numbers if x % 2 == 0].collect();
```

This is done using a procedural macro.
Rust's procedural macros essentially take a series of tokens and output another (possibly different) series of tokens.
As well-explained by Logan, a macro like this which converts Python to Rust, at its core works exactly like a compiler or transpiler.
These macros are applied at compile-time, so our Python list comprehensions will be expanded into valid Rust code prior to execution.

## Implementation

Compilers are made up of two key actions:
1. Parsing and mapping inputs to an intermediate representation
2. Mapping that intermediate representation into our output

At this [documentation link](https://docs.python.org/3/reference/grammar.html) is the full Python grammar.
Derived from the grammar used for the CPython parser.
We can use this a strong basis for our intermediate representation of list comprehensions.
Below is the section pertaining to list comprehensions:

```
# Comprehensions & Generators
# ---------------------------

for_if_clauses:
    | for_if_clause+ 

for_if_clause:
    | 'async' 'for' star_targets 'in' ~ disjunction ('if' disjunction )* 
    | 'for' star_targets 'in' ~ disjunction ('if' disjunction )* 

listcomp:
    | '[' named_expression for_if_clauses ']' 

setcomp:
    | '{' named_expression for_if_clauses '}' 

genexp:
    | '(' ( assignment_expression | expression !':=') for_if_clauses ')' 

dictcomp:
    | '{' kvpair for_if_clauses '}' 
```

To implement a subset of this functionality (list comprehensions only), Logan effectively simplifies this down to:
```
comprehension: mapping for_if_clause+

mapping: expression

for_if_clause: 'for' pattern 'in' expression ('if' expression)*

pattern: name (, name)*
```
Any list comprehension can be described completely by this intermediate representation.