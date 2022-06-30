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

Integer Range for unsigned variant is 0 to 255. For example if we use u8 (unsigned 8-bit), we cannot assign 256 as it will be error (overflow value).

## Float Types

Float data type in Rust can be classified as f32 and f64. f32 type is a single-precision float and f64 has double precision.
The default type is f64 because on modern CPUs it's roughly the same speed as f32 but is capable of more precision. All floating-point types are signed. Floating-point numbers are represented according to the IEEE-754 standard.

## Numeric Operations
```rust
fn main(){
    //addition
    let sum = 5 + 10;

    //subtraction
    let difference = 95.5 - 4.3;

    //multiplication
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;

    //remainder
    let remainder = 43 % 5;
    }
```
<br>
## Booleans
```rust
fn main(){
    let t = true;
    let f: bool = false; // with explicit type annotation
    }
```
<br>

## The Character Type
```rust
fn main(){
    let c = 'z';
    let z = 'Z';
    }
```
<br>

# Compound Types
Compound Types in Rust are tuples and arrays.

## Tuple
A tuple is a general way of grouping togther a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

```rust
fn main(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    }
```
<br>
To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value.
```rust
fn main(){
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is : {}", y);
    }
```
<br>
This is called destructuring, because it breaks the single tuple into three parts.

For accessing a tuple element directly by using a period followed by the index of the value we want to access.
```rust
fn main(){
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hunred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    }
```
<br>

