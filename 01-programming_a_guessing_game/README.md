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

## Generating Random Number
1. Using `use rand::Rng;` as a random-number generator.
2. Using `rand::thread_rng` using a thread as a random-number generator (I don't know what it means).
3. Using `gen_range` to generate random value (ex.1..100, not including 100).

## Comparing the Guess to the Secret Number
1. Using `std::cmp::Ordering` from Standard Library to use Ordering, which is another enum and has the variants Less, Greater, and Equal.
2. A *match* expression is made up of *arms*, which is Less, Greater and Equal.
3. When running `cargo build` there will be error states, because *guess_number* is String type and *secret_number* is a number.
4. We shadow the previous value of *guess* with a new one.<br>
```rust
guess.trim().parse()
```
<br>
5. *trim* method will eliminate any whitespace at the beginning and end.
6. *parse* method will only work on characters that can logically be converted into numbers and so can easily cause errors. So the *parse* method returns a *Result* type and we should add *expect* method.

## Allowing Multiple Guesses with Looping
1. use *loop* with curly brackets before asking to input the guess.
2. We can't exit the program *properly*, what we can do is using Ctrl+c or entering a non-number answer.

## Quitting After a Correct Guess
1. Add *break* statement in *Equal* arms. So when guess number same with secret number, the program will be exited.

## Handling Invalid input
1. As *expect* returns *Result* of *Ok* and *Err*, we can expand the statements and use *match* to check the Result.
2. *num* means, if number entered it will be inside the variable. *_* means that all *Err* values will be checked and continue to the next iteration.
3. Delete the secret number print statement.
