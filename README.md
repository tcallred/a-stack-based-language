# A Stack Based Language in Rust

This project is an experimental cross between a stack-based language like Forth and an array-based language like APL.

Array-based languages like APL typically make heavy use of infix operators. This project gives a user the ability to write APL-like code in a reverse-polish notation.

One goal in this project is to demonstrate writing Rust in a pure functional style with functional-but-in-place mutation facilitated by the ownership system. 

## Usage
`cargo run` - this will open a prompt and you can write a series of numbers and "words" that will be executed. Enter a blank line to exit. 

## Example

```
> 1 2 +
[[3.0]]

> 1 2 3
[[1.0]]
[[2.0]]
[[3.0]]

> 1 2 3 |
[[1.0, 2.0, 3.0,]]

> 1 2 3 | 5
[[5.0]]
[[1.0, 2.0, 3.0]]

> 1 2 3 | 5 *
[[5.0, 10.0, 15.0]]
```
