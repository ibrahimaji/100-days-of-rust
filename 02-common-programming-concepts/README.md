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
let x = 10; //'let' must be inserted
```
