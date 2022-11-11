# UV Lang

UVL is a interpreted, dynamically typed, programming language. The goal of this
project is to study Programming Language Theory and Rust. The syntax if heavly inspired by Rust with minor differences.

At the moment, this language represents all numbers (Number) as 64-bit float and
the string type is part of the language at the moment because we don't have arrays
functions, and `struct` yet (in fact, not even statements).

## Example

REPL

```bash
$ cargo run
::> 5 + 12
17
::> 5.12 + 12.5
17.62
::> "Hello"
"Hello"
::> "Hello " + "World!"
"Hello World!"
::> "a string" + 123
File "<main.uvl>", line 0, in <root>
    Operator '+' is not supported for "a string" of type String and 123 of type Number
::> 5/0
File "<main.uvl>", line 0, in <root>
    Division by zero: 5/0

# Assignment and immutability
::> let n = 1;
::> n = 2;
File "<main.uvl>", line 0, in <root>
    Name 'n' is immutable

# Opt-in mutability
::> let mut p = 5;
::> println p;
5
::> p = 1;
::> println p;
1
```

File
```
cargo run -- <path to source>/main.uvl
```
