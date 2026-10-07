# Cargo Workspace
A workspace is a collection of related **Cargo Packages**. Packages in a workspace share a `Cargo.lock` file.
Meaning that if multiple packages have same dependency that dependency will be resolved to one version. Packages
in a namespace also share one output directory and various build settings such as profiles. Workspaces are 
useful when you are working on multiple related packages together.

E.g. Let's say we wanted to have a monolithic codebase for a blogging website. A blogging website will have three
packages, one containing the API, another containing the frontend and third containing the code that is shared 
between the two.

There are two ways to create a workspace:
- Using root package
- Using virtual manifest

We are going to use **virtual manifest**. To start will create a new directory called `blog` and create a `Cargo.toml` file:

```sh
mkdir blog
cd blog
touch Cargo.toml
```

Add workspace section in `Cargo.toml`.

```toml
[workspace]
members = [
    "blog_api",
    "blog_web",
    "blog_shared"
]
```

This `Cargo.toml` file is called virtual manifest because it defines a workspace rather than a package.

Next create these packages.

```sh
cargo new --vcs none blog_api
cargo new --vcs none blog_web
cargo new --vcs none --lib blog_shared
```

Lets run `cargo build` from the root blog directory. Notice that a `Cargo.lock` and a `target` directory were
generated in the root blog directory. If we look at `blog_api`, `blog_web` and `blog_shared`, they don't have a target
directory or `Cargo.lock` file.

If we open the top level `target` directory and click debug, you can see that it contains `blog_api`, `blog_web` binary 
and `blog_shared` library.

Running cargo commands such as `cargo build` or `cargo test` from the root of our workspace will build or test every
package in the workspace, however, we could target a specific package with the `-p` flag. E.g. we can build blog_api
by typing in below command:

```sh
cargo build -p blog_api
```

Next let's update the three packages in our workspace, starting off with shared library. First we will add `serde` library
as a dependency which is used for serialization. Also we will turn on the `derive` feature, so that we have access to 
Serialize & Deserialize derive macros.

```toml
[package]
name = "blog_shared"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

Then we will open `lib.rs`. Delete the default test module and implement a `post` struct and we will also add a Constructor
function. Next we will import Serialize & Desrialize from serde. Then we will derive Serialize, Deserialize and Debug traits
for post.

```rust
use serde::{Serialize, Deserialize}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    title: String,
    body: String,
}

impl Post {
    pub fn new(title: String, body: String) -> Post {
        Post { title, body }
    }
}
```

Now that the Post is defined, let's use it in our `blog_api` and `blog_web`.

First open `Cargo.toml` inside blog_api and add `blog_shared` as a dependency.

```toml
[package]
name = "blog_api"
version = "0.1.0"
edition = "2021"

[dependencies]
blog_shared = { path = "../blog_shared" }
```

Then we will open `main.rs` and import Post.

```rust
use blog_shared::Post;

fn main() {
    let post = Post::new(
        "Post on the server".to_owned(),
        "Let's get Rusty!".to_owned(),
    );

    println!("{post:?}");
}
```

And we can do the same thing on blog_web.

```toml
[package]
name = "blog_web"
version = "0.1.0"
edition = "2021"

[dependencies]
blog_shared = { path = "../blog_shared" }
```

And main.rs as below:

```rust
use blog_shared::Post;

fn main() {
    let post = Post::new(
        "Post on the web".to_owned(),
        "Let's get Rusty!".to_owned(),
    );

    println!("{post:?}");
}
```

So we have backend, frontend and shared all being developed in same workspace.
