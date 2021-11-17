# A Stack Based Language in Rust

This project is an experimental cross between a stack-based language like Forth and an array-based language like APL.

Array-based languages like APL typically make heavy use of infix operators. This project gives a user the ability to write APL-like code in a reverse-polish notation which can make it easier to express a program from left to right.

One goal in this project is to demonstrate writing Rust in a pure functional style with functional-but-in-place mutation facilitated by the ownership system. 

## Usage
`cargo run` - this will open a prompt where you can write a series of numbers and "words" that will be executed after hitting enter. The output will be "the stack" after executing the line. Enter a blank line to exit. 

## Example

```
> 1 2 +
3

> 1 2 3
3
2
1

> 1 2 3 |
[[1.0, 2.0, 3.0,]]

> 1 2 3 | 5
5
[[1.0, 2.0, 3.0]]

> 1 2 3 | 5 *
[[5.0, 10.0, 15.0]]

> 1 2 3 | 5 * sum
30
```

## Available words
- `+` Addition
- `-` Subtraction
- `*` Multiplication
- `/` Division
- `|` Create array from scalers at the top of the stack
- `negate`, `neg` Negate 
- `reverse`, `rev` Reverse array
- `length`, `len` Length of array
- `sum` Sum of array
- `product`, `prod` Product of array
- `iota` Produce array that is a range from 1 to n
- `right` Copy first element on the stack
- `left` Copy second element on the stack
- `commute` Swap the first and second element on the stack
