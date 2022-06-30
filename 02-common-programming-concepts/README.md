# Common Programming Concepts

## Keyword
Cannot use the *keyword* as names of variables or functions (reserved for use by the language only).

## Variables and Mutability
1. Variable by default is set to immutable.
2. Cannot assign twice to immutable variable.

## Constants
1. Constants aren't just immutable by default, they're always immutable.
2. The type of the value must be annotated.
3. Constant may be set only to a constant expression.
```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 & 3;
```
4. Rust's naming convention for constans is to use all uppercase with underscores between words.

## Shadowing
1. Shadowing means declaring a new variable with the same name as a previous variable.
2. The difference with error when assign a variable in mutable variable is. <br>
```rust
let x = 7;
x = 10; //error will occured
```
<br>
When use Shadowing
```rust
let x = 7;
let x = 10; //let must be inserted
```

# Data Types

1. Rust is a *statistically typed language*, which means that it must know the types of all variables at compile time.

## Scalar Types
1. A scalar type represents a single value
2. Four primary scalar types: integers, floating-point numbers, Booleans, and characters.

## Integer Types
An integer is a number without a fractional component.
|Length|Size|Signed|Unsigned|
|---|---|---|---|
|8-bit|i8|u8|
|16-bit|i16|u16|
|32-bit|i32|u32|
|64-bit|i64|u64|
|128-bit|i128|u128|
|arch|isize|usize|
Signed integers store both negative and positive values. Unsigned integers can only store positive values.

