# Basic Components
There are three basic components we need to know about:
- **Packages**
    - When we run `cargo new` a package is created.
    - Contains one or more crates that provides a set of functionality and allow us to build, test & share crates.
    - Inside a package we have:
        - **Cargo.toml**: Describes the package and defines how to build crates.
        - **Rules**: Packages also have some rules:
            - They must contain at-least one crate.
            - At most one library crate.
            - Any number of binary crates.
- **Crates**
    - A tree of modules that produces a library or executable.
- **Modules**
    - Allows to organize our code and control scope and privacy.

Let's look at a visual example:

pic

- At the top we have a **package** which is created when we run `cargo new`
- The **package** contains **crates**. In this case we have one **binary crate** and one **library crate**.
- Inside each **crate** we have a tree of modules. Every **crate** has a **root module** which is also called 
**crate root**.
- **Modules** can have **submodules**.

Let's jump into example code.
Let's create a new cargo package called `my_package`
```sh
cargo new my_package
```

Our package comes with Gopkg.toml which contains project metadata and dependencies.
It also comes with binary crate which starts in `main.rs`.

Cargo follows the convention that if `main.rs` file exists in the source directory then it will be the **crate 
root** of a binary crate with the same name as the package. So in this case my_package.

Likewise if we had a file called `lib.rs` in the source directory, cargo would treat that as a crate root for
a **library crate** with the same name as our package.

If we have both `main.rs` and `lib.rs` in the source directory then our package has two crates a binary crate
and a library crate. This is a common pattern for cli applications.

Our package can have at most 1 library crate and any number of binary crates. Cargo has convention for that if 
we wanted to add more binary crates, we could create a sub directory within the source directory called `bin`.

Inside bin directory lets add a file called `another_one.rs`

```sh
mkdir src/bin
touch src/bin/another_one.rs
```

And add main function such as:

```rust
fn main() {
    println!("another one!");
}
```

Now our package has three crates:
- A library crate called `my_package` which starts in `lib.rs`.
- A binary crate called `my_package` which starts in `main.rs`.
- And another binary crate called `another_one`

Let's try to run our binary crates.

```sh
cargo run
```

Now it will not work because we have two binary crates now. So we have to specify which binary crate. So we
can specify which binary to run using `--bin` flag.

```sh
cargo run --bin my_package
cargo run --bin another_one
```
