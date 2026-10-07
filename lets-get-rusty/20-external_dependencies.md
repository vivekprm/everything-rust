So far we have created a library called `auth_service` and demonstrated how to use it inside binary crate.
Now let's say we wanted to update our library specifically the authenticate function, we don't want people
to spam call the authenticate function, with random credentials trying to break into somebody's account.

To protect against this we will add a layer of protection by simulating a timeout. So if somebody called
this function they can call it again only after a certain period of time has passed. In this case we want
that time between 100ms to 500ms. We can do that by generating a random number between 100-500. Rust
standard library doen't have function to generate random numbers and we don't want to implement that
functionality ourselves. Luckily for us, we could use a dependency.

Dependencies are easy to find because Rust has a public crate registry called `crates.io`. If we navigate
to `crates.io` and scroll little bit, we'll notice that most popular crate is called `rand`. Let's add
this dependency to our `auth_service` package. Modify `Cargo.toml` as below:

```toml
[package]
name = "auth_service"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.4"  
```

Then we can use rand dependency in `lib.rs`. Cargo will automatically let the Rust compiler know of our
dependency so we could use it right away in our code. First lets bring all the items in the `rand` prelude
into scope with the `use` keyword.

```rust
use rand::prelude::*;

pub mod database;
mod auth_utils;

pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    let timeout = thread_rng().gen_range(100..500);

    println!("The timeout is: {timeout}");
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}
```
