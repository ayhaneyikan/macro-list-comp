# macro-list-comp
A rust macro implementing Python list comprehensions.
Based on the YouTube video by Logan Smith: [Comprehending Proc Macros](https://youtu.be/SMCRQj9Hbx8?si=eGoCg6oOoZJ3Tiy2)

If unfamiliar, Python list comprehensions allow you to iterate over an iterable, while mapping values, and optionally filtering values.

For example we want to go through a list and square every value that is even:
```python
even_squared = [x*x for x in numbers if x % 2 == 0]
```

This is done using a procedural macro.
Rust's procedural macros essentially take a series of tokens and output another (possibly different) series of tokens.
As well-explained by Logan, a macro like this which converts Python to Rust, at its core works exactly like a compiler or transpiler.
These macros are applied at compile-time, so our Python list comprehensions will be expanded into valid Rust code prior to execution.

Compilers are made up of two key actions:
1. Parsing and mapping inputs to an intermediate representation
2. Mapping that intermediate representation into our output

