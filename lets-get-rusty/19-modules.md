# Modules
In rust **Modules** serve a couple of purposes:
- They organize code for readability and reuse.
- Control scope and privacy
- Modules contain items such as functions, structs, enums, traits, etc.
- Explicityly defined (using the mod keyword)
    - Not mapped to the file system
    - Flexibility & straight forward conditional compilation.
    - A single file can have multiple modules.

Let's look at code example. We will implement auth service without using modules and then we will see how we can
use modules to refactor our code. So let's first create a library crate called auth_service.

```sh
cargo new auth_service --lib
```

Let's open `src/lib.rs` and implement a function called authenticate. We will suppress few warnings but not
recommended in actual code.

```rust
#![allow(dead_code, unused_variables)]

struct Credentials {
    username: String,
    password: String,
}

enum Status {
    Connected,
    Interrupted,
}

fn connect_to_database() -> Status {
    Status::Connected
}

fn login(creds: Credentials) {
    // authenticate...
    get_user();
}

fn get_user() {
    // get user from database...
}

fn logout() {
    // log user out...
}

fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_to_database() {
        login(creds);
    }
}
```

With this our auth_service library is complete. However, this code could be improved. In this single file we have
multiple level of abstractions. We have high level function we want to expose from our library such as
`authenticate`. Then we have lower level functions like get_user, logout etc. This single file is also mixing
concerns. We have models and functions that to do with authentication then we also have functions and models 
which connect to database and status.

We can clean this code up by encapsulating the authentication related code and database related code into 
separate modules. Before we do that it's worth noting that we already have a module and to prove that let's 
open up the terminal and install `cargo modules`. 

**Cargo modules** is a cargo plugin which allows us to visualize our crate module tree.

```sh
cargo install cargo-modules
```

Then to visualize cargo module tree run:
```sh
cargo modules generate tree 
```

Currently we have only one module called crate. crate module is the root of our module tree and it's automatically
created for every crate. The contents of the crate module is either `lib.rs` or `main.rs` if we are working with
library or binary respectively.

In this case auth_service is a library crate so content of the crate module is `lib.rs`. We can see that if we
run the generate tree command again but specify `--with-types`.

```sh
cargo modules generate tree --with-types
```

Here we see that our crate module has:
- struct Credentials: pub(crate)
- enum Status: pub(crate)
- function authenticate: pub(crate)
- ...
-..

Here we can also see visibility of each item. In Rust by default most things are private. Each of the items
above are tagged as pub(crate) meaning they are visible inside out crate.



