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
`authenticate`. Then we have lower level functions like `get_user`, `logout` etc. This single file is also mixing
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
library or binary crate respectively.

In this case `auth_service` is a library crate so content of the crate module is `lib.rs`. We can see that if we
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
above are tagged as pub(crate) meaning they are visible inside our crate but they are not visible to consumers
to our crate. So currently our auth_service has no public API.

We want to expose `authenticate` function to consumers and in order to do that we can change it's visibility by
using the pub keyword. Let's prefix our `authenticate` function with pub keyword.

```rust
pub fn authenticate(creds: Credentials) {
    if let Status::Connceted = connect_to_database() {
        login(creds);
    }
}
```

Now here we get error as we are making authenticate function public however it's argument Credentials is private.
So let's also make credential public:

```rust
pub struct Credentials {
    username: String,
    password: String,
}
```

Then will generate the module tree again:

```sh
cargo modules generate tree --with-types
```

Now we can see that our crate module has a public interface with two items, `Credentials` struct and the `authenticate` 
function. This is exactly the public interface we want. Now let's focus on organizing our code by taking the database
related code and putting it into a module and taking the authentication related code and putting it into another 
module.

Let's create our database module and auth modules. To create a module in Rust, we use `mod` keyword.

```rust
mod database {
    enum Status {
        Connected,
        Interrupted,
    }

    fn connect_to_database() -> Status {
        return Status::Connected;
    }

    fn get_user() {
        // get user from the database...
    }
}

mod auth_utils {
    fn login(creds: Credentials) {
        // authenticate...
        get_user();
    }

    fn logout() {
        // log user out
    }
}
```

We will also create submodule under `auth_utils` module called `models` which will store the credentials struct.

```rust
mod auth_utils {
    fn login(creds: Credentials) {
        // authenticate...
        get_user();
    }

    fn logout() {
        // log user out
    }

    mod models {
        pub struct Credentials {
            username: string,
            password: string,
        }
    }
}
```

Now the problem with above is get_user & Credentials is not found in `auth_utils` module. We need to tell `auth_utils`
module how to find these items. To do that we can specify items fully qualified name. Which is relative path to that
item in the module tree or the absolute path to that item.

```rust
mod database {
    enum Status {
        Connected,
        Interrupted,
    }

    fn connect_to_database() -> Status {
        return Status::Connected;
    }

    pub fn get_user() {
        // get user from the database...
    }
}
mod auth_utils {
    fn login(creds: models::Credentials) {
        // authenticate...
        crate::database::get_user();
    }

    fn logout() {
        // log user out
    }

    mod models {
        pub struct Credentials {
            username: string,
            password: string,
        }
    }
}

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_to_database() {
        login(creds);
    }
}
```

Now we get different error that `get_user` function is private. If ww want get_user to be visible outside database
module then we need to use pub keyword. Now we see compilation error in authenticate function, as it's not able to
find Credentials ctruct, connect_to_database & login functions let's fix it by giving path to these items.

```rust
pub fn authenticate(creds:: auth_utils::models::Credentials) {
    if let database::Status::Connected = database::connect_to_database() {
        auth_utils::login()
    }
}
```

Now we again see bunch of privacy errors. The models module is private, the Status enum is private, connect_to_database()
function is private and the login() function is private. Now let's fix these by making them public.


```rust
mod database {
    pub enum Status {
        Connected,
        Interrupted,
    }

    pub fn connect_to_database() -> Status {
        return Status::Connected;
    }

    pub fn get_user() {
        // get user from the database...
    }
}
mod auth_utils {
    pub fn login(creds: models::Credentials) {
        // authenticate...
        crate::database::get_user();
    }

    fn logout() {
        // log user out
    }

    pub mod models {
        pub struct Credentials {
            username: string,
            password: string,
        }
    }
}

pub fn authenticate(creds: auth_utils::models::Credentials) {
    if let database::Status::Connected = database::connect_to_database() {
        auth_utils::login()
    }
}
```

Now all the errors are resolved. Now you might have noticed that these fully qualified names can get quite long especially
if we have multiple levels of sub modules. To cleanup this code a bit, we could use the advantage of `use` declaration to create
local name bindings to a given path and bring it into scope. e.g.

```rust
use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login()
    }
}
```

Now let's look at our module tree again:

```sh
cargo modules generate tree 
```

So far modules that we have defined have been inline, meaning the module declaration and it's content are defined together. However, you
can define the content of your module into a different file.

In Rust there are two ways of doing this, both of which can be awkward and confusing if you are coming from other languages. So let's go 
through each way. If you have module with no sub-modules such as `database` module then the process is straight forward.

In our source directory we could create a new file called `database.rs`

```sh
touch src/database.rs
```

And take the content of datbase module and paste it in this file.

```rust
mod database {
    pub enum Status {
        Connected,
        Interrupted,
    }

    pub fn connect_to_database() -> Status {
        return Status::Connected;
    }

    pub fn get_user() {
        // get user from the database...
    }
}

```

An then change the `src/lib.rs` as:

```rust
mod database;
mod auth_utils {
    pub fn login(creds: models::Credentials) {
        // authenticate...
        crate::database::get_user();
    }

    fn logout() {
        // log user out
    }

    pub mod models {
        pub struct Credentials {
            username: string,
            password: string,
        }
    }
}

pub fn authenticate(creds: auth_utils::models::Credentials) {
    if let database::Status::Connected = database::connect_to_database() {
        auth_utils::login()
    }
}
```

Since in Rust modules are not mapped to the filesystem, in addition to create `src/database.rs` we need to add 
this line in `lib.rs` as well.

Now let's look at modules with submodules such as `auth_utils` module. There are two ways to structure this:
- Created a folder with same name as our module which contains a `mod.rs` file.

```sh
mkdir src/auth_utils
touch src/auth_utils/mod.rs
```

Take the contents of `auth_utils` in `lib.rs` and paste in mod.rs file.

```rust
pub fn login(creds: models::Credentials) {
    // authenticate...
    crate::database::get_user();
}

fn logout() {
    // log user out
}

pub mod models {
    pub struct Credentials {
        username: string,
        password: string,
    }
}
```

Next we will create another file inside `auth_utils` called `models.rs` then we go back to mod.rs and take the
contents of models module and move them into `models.rs` file.

src/auth_utils/models.rd
```rust
pub struct Credentials {
    username: string,
    password: string,
}
```

src/auth_utils/mod.rs
```rust
pub fn login(creds: models::Credentials) {
    // authenticate...
    crate::database::get_user();
}

fn logout() {
    // log user out
}

pub mod models;
```

So from `lib.rs` Rust searches for `auth_utils.rs` if it's not found next Rust looks at `mod.rs` in folder called auth_utils.
Which does exist.

Downside of using `mod.rs` approach is, imagine having a bunch of mod.rs files opened in our code editor, it can be hard to
tell which module we are modifying. Which bring us to other approach instead of creating `mod.rs` file in `auth_utils` directory
we create `auth_utils.rs` file and copy the content of `mod.rs` and delete `mod.rs`. It's submodules are in `auth_utils` directory.

Now let's create a binary crate to demonstrate how we can use our `auth_service` library. To do that let's add main.rs in src.

```sh
touch src/main.rs
```

Add below code:

Because our library is in the same package we can bring it into scope using use keyword.

```rust
use auth_service::{Credentials};

fn main() {
    let creds = Credentials {
        username: "letsgetrusty".to_owned(),
        password: "password".to_owned(),
    }

    auth_service::authenticate(creds);
}
```

We get an error saying that "Username and password field can't be used because it's private. To fix this we can take two approaches:
- Make both the fields public.
- Keep the fields private and create an implementation block with an associated function called `new`.
