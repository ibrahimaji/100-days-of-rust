## Installation
1. Install Rust
```term
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Install Rust-Analyzer
```term
rustup component add rust-src
```
3. Enable [Rust Language Server](https://github.com/neovim/nvim-lspconfig/blob/master/doc/server_configurations.md#rust_analyzer) in init.lua
`require'lspconfig'.rust_analyzer.setup{}`
4. Create Rust File
```term
nvim main.rs
```
`fn main(){
        println!("Hello World");
    }`
5. Make executable file from that file and run it
```term
rustc main.rs
```
```term
./main
```
6. `fn main()` is the first function which will be executed in every Rust file.
7. `println!` is Rust Macro which different with `println()` function.
8. `;` is the end of expression.


## Cargo
1. Cargo is Rust's build system and package manager
2. Create project using Cargo
```term
cargo new hello_cargo
```
3. `cargo check` is to make sure it compiles but doesn't produce executable file (much faster).
4. `cargo build` is to compile and produce executable
5. `cargo build --release` is to optimize and produce executable in folder `target/release` (slower, for final product)
6. `cargo run` is to run the executable file (also build executable file, if there's not built yet)
