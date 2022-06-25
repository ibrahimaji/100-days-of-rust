## Setting up a new project
1. Make a new project using Cargo<br>
`cargo new guessing_game`<br>
`cd guessing_game`
2. The inside of *src/main.rs* file will be :
```rust
fn main(){
    println!("Hello, world!");
    }
```
3. Compile the program with *cargo run*

## Processing a Guess
1. Change the code inside *main.rs* with :
```rust
use std::io;

fn main(){
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}",guess);
    }
```
2. To obtain the user input, we must use *io* standard library (input/output).<br>
`use std::io;`
3. `fn main(){}` is the entry point into the program.
4. `println!` is a macro that prints a string to the screen.

## Storing values with variables
1. `let mut guess = String::new();` creates new variable named guess and binds it to the value *String::new()*. In Rust, variables are immutable by default. The `::` syntax indicated that *new* is an *associated function* of the `String` type. This *new* function creates a new, empty string.
2. `io::stdin` refers to `use std::io;`.
3. `.read_line(&mut guess)` calls the read_line method on the standard input handle to get input from the user. & indicates this argument is a *reference*. References are immutable by default, so it must use `(&mut guess)`.

##Handling potential failure with the result type
1. read_line returns a value, an `io::Result`. The result types are *enumerations* or *enums*. An enumeration is a type that can have a fixed set of values, and those values are called the enum's *variants*. For the Result, the *variants* are *Ok* or *Err*. *Ok* indicates operation is successful, and *Err* means operation failed. *expect* is used for handling *Err*.

##Printing values with println! Placeholders
1. {} is a placeholder which hold a value in place.

##Testing the First Part
use `cargo run`
