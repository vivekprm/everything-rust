# Macros
Macros are also called `Syntax Extensions`. Or a way of writing code that writes other code. This is a form of **meta-programming**. Macros
are expanded at compile time.

Before diving into the details, let's look at the pros and cons.

Pros:
- On the upside `macros` help reduce the amount of code you have to write and maintain.
- Allow us to extend the syntax of the language.

Cons:
- Macros can be difficult to read, understand, debug and maintain.
- Also because `macros` are processed at compile time they can increase compile time.

# How Compilation Works
```
Source
Code
  |
  v

Lexical Analysis -> Syntax Analysis -> Semantic Analysis -> Intermediate Code Generation -> Code Optimization -> Target Code Generation
                                                                                                                            |
                                                                                                                            v
                                                                                                                          Binary
```
For the macros, we will focus on first three stages of compilation process.

## Lexical Analysis

<img width="1710" height="923" alt="Screenshot 2026-10-31 at 9 44 09 PM" src="https://github.com/user-attachments/assets/656e8022-8655-4516-9382-d47edf633819" />

From the perspective of compiler, your source code starts of as a meaningless text file. The Job of `Lexical Analysis` is to turn this
meaningless text into a stream of tokens and then classify the tokens. **Tokens** are indivisible lexical units. Think about them as 
words a language understands. And they can be grouped into several categories. Such as:

- Identifiers
- Literals
- Keywords
- Symbols

In above example, `impl`, `for` and `fn` are all `Keywords`. `Animal`, `Dog` and `speak` are all `Identifiers`. 
The string "woof!" is a `Literal` and `{`, `(` and `&` are all `Symbols`.

Note that in this stage of compilation process tokens have been identified but no meaning have been attached to them.

Once tokens are identified and categorized they could be used by tools such as syntax highlighters which we can see in the example to the right.

Stream of tokens is then turned into a stream of token trees. Almost all tokens are token trees themselves, specifically leaf nodes. The only
non-leaf nodes are grouping tokens such as parantheses, brackets or curly braces. 

<img width="843" height="250" alt="Screenshot 2026-10-31 at 9 50 47 PM" src="https://github.com/user-attachments/assets/28fb1f37-9b9d-44fa-b07b-c87089b7a0df" />

In this example `impl`, `Animal`, `for` and `Dog` are all `token trees` with one leaf node. The `{}` are a root node in a token tree containing
`fn`, `speak`, `()` and `{}`.

`Macros` in Rust deal with tokens and token trees. So it's important to understand these two concepts.

Next thing in the compilation process is, `Syntax Analysis`, which turns a `Stream of token trees` into an `Abstract Syntax Tree`.

<img width="845" height="360" alt="Screenshot 2026-10-31 at 9 57 30 PM" src="https://github.com/user-attachments/assets/24d6c96e-8fd0-4c93-a5db-68055a7451c7" />

The `Abstract Syntax Tree` represents the syntax structure of your program. The `nodes` in this syntax tree could be `literals`, `functioncalls`
`if else branches` and more. Using the `Abstract Syntax Tree` programs like `linters`, `test coverage tools` and `transpilers` are implemented.

In Rust, `macros` are processed or expanded, after the `Abstract Syntax Tree` is constructed but prior to `Semantic Analysis`.

<img width="836" height="376" alt="Screenshot 2026-10-31 at 10 03 46 PM" src="https://github.com/user-attachments/assets/5cc10a92-7ef5-46fc-ae79-bdb81b75982a" />

Looking at the code to the left we can see that it contains a `macro` the `println!` macro. 

<img width="839" height="419" alt="Screenshot 2026-10-31 at 10 04 08 PM" src="https://github.com/user-attachments/assets/74565f2c-a337-4e03-b75d-693d95782887" />

This `macro` will be identified as a node in the `Abstract Syntax Tree` and then be expanded into regular rust code.

After all `macros` are expanded `Semantic Analysis` checks to make sure that `Abstract Syntax Tree` semantically makes sense.

<img width="843" height="430" alt="Screenshot 2026-10-31 at 10 05 46 PM" src="https://github.com/user-attachments/assets/64ae095b-d093-45d7-8605-58bfa8cdcd9c" />

In this case, AST is semantically correct. However, Imagine if we implemented `Animal` for a type that didn't exist e.g. `xzy`, in this case
our tree is structurally correct but semantically it doesn't make sense.

<img width="836" height="426" alt="Screenshot 2026-10-31 at 10 07 15 PM" src="https://github.com/user-attachments/assets/82a96d75-0048-4208-a774-25218d0a1ebc" />

In summary, we learned that `macros` are a way of writing code that writes other code and they are expanded or processed at compile time.
Specifically, macros are expanded after the `Abstract Syntax Tree` is constructed but before it gets validated by `Semantic Analysis`.

# Declarative Macros
Let's create a new project called `declarative_macros` with a binary crate and a library crate. Rust has two types of `macros`:
- Declarative Macros
    - Otherwise known as `Macros By Example` are a way of writing macros by matching against code, similar to a match expression.
- Procedural Macros

Let's write our first declarative macro called `hello` in `lib.rs`.

```rust
macro_rules! hello {
    () => {
        println!("Hello World!");
    };
}
```

To create a `declarative marco` we use `macro_rules` which itself is a `macro`. Followed by the name of our macro and then body of the macro.

Here is how the general format looks like:

```rust
// macro_rules! name {
//    rule0;
//    rule1;
    // ..
//    ruleN;
//}

// (matcher) => { expansion aka transcriber }
```

`Declarative macros` are made up of one or more rules and each rule contains a `matcher` and an `expansion` also known as a `transcriber`.

Each rule is evaluated in the order that it's defined. Once a rule matches, the code in the `expansion` replaces the macro invocation at the
call site. For the `hello` macro we have one rule which matches if and only if input is empty `()` and replaces the macro invocation with one
line of code which prints `Hello World!`. Notice that `println!` itself is a macro. `Macros` could contain other `macros`.

Let's call our `hello` macro from `main.rs`. First we need to export this macro from the library crate by annotating it with the macro export
attribute.

```rust
#[macro_export]
macro_rules! hello {
    () => {
        println!("Hello World!");
    };
}
```

Then add below in `main.rs` and import `hello`.

```rust
use declarative_macros::*;

fn main() {
    hello!();
}
```

One thing to note is that these `()` can be any grouping token e.g. `{}` or `[]`. If we replace `hello!();` with `hello![]` or `hello!{}`, we
get the same output. This works because `declaratice macros` match against the root node of a single token tree which if you recall from last
lession, grouping tokens are. Usually the type of grouping token you would use, depends on a macro and the arguments it take. The `hello` macro
takes no arguments and could be treated as a function call so parantheses makes sense. For the `vec!` macro however `[]` are used which makes
more sense because the input it takes is a list of items to be stored in the vector.

Let's say that we wanted to create a hashmap associating team names with team scores. First we will import the hashmap struct.

```rust
use std::collections::HashMap;
use declarative_macros::*;

fn main() {
    hello!();

    let scores:: HashMap<String, i32> = HashMap::new();
}
```

Notice that without specifying the type while declaring `HashMap` we get an error as Rust compiler can't infer the type. But this is quite bit
of code for an empty hashmap. We could omit the explicit type annotation if we insert key and value pair into our hashmap. That way Rust would
infer the type of the key and value pair.

```rust
use std::collections::HashMap;
use declarative_macros::*;

fn main() {
    hello!();

    let scores: HashMap<String, i32> = HashMap::new();

    let mut scores2 = HashMap::new();
    scores2.insert("Red team".to_owned(), 3);
    scores2.insert("Blue team".to_owned(), 5);
    scores2.insert("Green team".to_owned(), 2);
}
```

Now we can see the types are infered for `scores2` hashmap. This is still quite a bit of code to create and initialize a `HashMap`. It would
be nice if we could create and initialize the hashmap at the same time with less verbose syntax. Let's see how we can use the `declarative macros`
to make it easier to create it empty and populate the hashmap.

Let's switch over to our library crate and create a new declarative macro called `map!`.

```rust
#[macro_export]
macro_rules! hello {
    () => {
        println!("Hello World!");
    };
}

macro_rules! map {
    ($key: ty, $val: ty) => 
}
```

Because we are matching against code the pattern we use in this match expression are different than the pattern we use in a regular match
expression. In this case, we are matching against a meta variable called `key` followed by `, ` and then another meta variable called `val`.
`Meta variables` use the following format.

```rust
// $ [identifier] : [fragment-specifier]
```

A `$` sign followed by an identifier or the variable name `:` and then a `fragment-specifier`. `fragment-specifier` define the type being 
captured. [Here](https://lukaswirth.dev/tlborm/decl-macros/minutiae/fragment-specifiers.html) is the list of possible fragment specifiers.
In our case we are using `ty` fragment specifier which is used for `types`. 

We are saying match against a `type` and store in variable `key` and then match against a second `type` and store in another variable `val`.
Inside the transcriber let's create a new HashMap and return it.

```rust
#[macro_export]
macro_rules! hello {
    () => {
        println!("Hello World!");
    };
}

#[macro_export]
macro_rules! map {
    ($key: ty, $val: ty) => {
        let map: HashMap<$key, $val> = HashMap::new();
        map
    }
}
```

We are returning a block expression, inside we create a new variable called `map` and set it equal to empty hashmap and then return map. The
type of `key` and `value` pair in our hashmap are mapped to the `key` and `val` meta variables. Let's see how this macro can be used in `main.rs`

```rust
use std::collections::HashMap;
use declarative_macros::*;

fn main() {
    hello!();

    let scores: HashMap<String, i32> = HashMap::new();
    let scores = map!(String, i32);

    let mut scores2 = HashMap::new();
    scores2.insert("Red team".to_owned(), 3);
    scores2.insert("Blue team".to_owned(), 5);
    scores2.insert("Green team".to_owned(), 2);
}
```

As you can see using this macro is lot less verbose than defining an empty map manually. Let's solve for second case where we want to create
and initialize a `HashMap` at the same time. First we will write what we want end result to look like.

```rust
fn main() {
    // ...
    let scores2 = map!(
        "Red team".to_owned() => 3,
        "Blue team".to_owned() => 5,
        "Green team".to_owned() => 2
    );
    // ...
}
```

We want the ability to create and initialize a hashmap by passing in a comma separated list of key and value pairs, separated by an arrow.
Let's add new rule to our `map` macro.

```rust
#[macro_export]
macro_rules! map {
    ($key: ty, $val: ty) => {
        let map: HashMap<$key, $val> = HashMap::new();
        map
    };
    ($key:expr => $val:expr) => {
        
    }
}
```

In this case we want to match against two expressions separated by an arrow. So far this pattern matches a single key and value pair. We
want it to match against arbitrary amount of key and value pairs. To do that we need to introduce the concept of repitition.

`Repitition` follows this general format.
```
// $ (...) sep rep
```

A `$` sign followed by parantheses containing the matcher to be repeated, an optional separator token in this case `,` and a required repeat 
operator, in this case `*`. `*` represents 0 or more repititions. `?` represents atmost one repeatition. `+` indicates one or more repeatitions.
The body of our `transcriber` would be similar to first rule.

```rust
#[macro_export]
macro_rules! map {
    ($key: ty, $val: ty) => {
        let map: HashMap<$key, $val> = HashMap::new();
        map
    };
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $( map.insert($key, $val); )*
            map
        }
    }
}
```

# Procedural Macros
Unlike `declarative macros`, `procedural macros` allow us to define a function which takes a **token stream as input** and returns a **token
stream as output**, rather than matching against patterns. Also unlike `declarative macros`, `procedural macros` must be defined in their
own crate **with a special crate type**. This is due to complex technical reasons that the Rust team hopes to eliminate in the future.

Let's start of with creating a new project called `procedural_macro` with a library crate.

```sh
cargo new procedural_macros --lib
cd procedural_macros
```

Make below change in `Cargo.toml` and set `proc-macro` to true for our library crate. This will implicitly link our library crate with
the compiler provided `proc-macro` crate, which will give us all the tool that we need to build procedural macros.

```toml
...
[lib]
proc-macro = true

[dependencies]

```

To get started, let's open `lib.rs` and import `proc-macro` crate, provided by the compiler.

```rust
extern crate proc_macro;
```

This is one of the few times you are going to see this `extern crate` syntax, which makes your Rust code aware of other crates in your
project. Your Rust project is automatically aware of crates you define as dependencies in `Cargo.toml` but in this case `proc_macro` is
provided by the compiler, so we have to use this special syntax.

All `procedural macros` are functions which take token stream as input and return token stream as an output. So let's bring token stream
into scope.

```rust
extern crate proc_macro;

use proc_macro::{TokenStream};
```

# Procedural Macros - Function Like
There are three types of procedural macros:
- Function Like
- Attribute Like
- Custom Derived

The Simplest of the three is **Function Like** macros. We will create a function like `log_info` which will take a `TokenStream` as input
and return a `TokenStream`.

```rust
extern create proc_macro;

use proc_macro::{TokenStream};

pub fn log_info(input: TokenStream) -> TokenStream {    // error
    input
}
```

We will get an error stating that "`proc_macro` crate types currently can not export any items other than functions tagged with `#[proc_macro]`,
`#[proc_macro_derive]`, or `#[proc_macro_attribute]`". To mark this function like `procedural_macro` will tag it with `#[proc_macro]`.

```rust
extern create proc_macro;

use proc_macro::{TokenStream};

#[proc_macro]
pub fn log_info(input: TokenStream) -> TokenStream {    // error
    input
}
```

Currently, this function is not doing anything, it's taking the `TokenStream` as input and simply returning that `TokenStream`. Let's see how
we can use this macro. Add below in `main.rs`.

```rust
use procedural_macors::*;

fn main() {
    log_info!(println!("Hello!"););
}
```

Notice that invoking a function like procedural macro is identical to invoking `declarative_macro`. Input in this case is like 
`println!("Hello");`. Let's go ahead and run our program. 
Currently, the `log_info` takes the input `TokenStream` and returns it, return `TokenStream` will replace the macro invocation, so this line
will turn into below:

```
println!("Hello!");
```

The goal of the `log_info` macro is to accept input like this, whatever we pass in should be logged and special strings like `TIME` in below
should be replaced. In this case `[TIME]` should be replaced with timestamp:

```rust
fn main() {
    log_info!([TIME] starting program...);
}
```
To accomplish this first we have to import 2 crates `quote`(turns rust code into TokenStream) and `chrono` (a date and time library).

```toml
...
[lib]
proc-macro = true

[dependencies]
quote = "1.0"
chrono = "0.4"
```

Inside `log_info` let's start with creating mutable owned string called `output`.

```rust
extern create proc_macro;

use proc_macro::{TokenStream};
use quote::quote;

#[proc_macro]
pub fn log_info(input: TokenStream) -> TokenStream {
    let mut output = "[Info]".to_owned();

    TokenStream::from(quote!{
        println!("{}", #output);
    })
}
```

The `quote` crate provides a macro also called `quote` which allows us to turn Rust code into a `TokenStream`. The `quote` macro also allows
variable interpolation as we are using `output` variable.

Lastly, `quote` returns a TokenStream that is slightly different than the TokenStream that is defined in the `proc_macro` crate. So we use the
`from` function on `TokenStream` to turn it into correct type.

The last thing to do is parse the input token stream. We want to take the input and append it to the ouptput stream, to do that we can use a
`for` loop and a match expression.


```rust
extern create proc_macro;

use proc_macro::{TokenStream};
use quote::quote;
use chrono::prelude::*;

#[proc_macro]
pub fn log_info(input: TokenStream) -> TokenStream {
    let mut output = "[Info]".to_owned();

    for token in input {
        let token_string = token.to_string();

        match token_string.as_str() {
            "[TIME]" => {
                let time = Utc::now().time().to_string();
                output.push_str(&format!("{} ", time));
            }
            _ => {
                output.push_str(&format!("{} ", token_string));
            }
        }
    }

    TokenStream::from(quote!{
        println!("{}", #output);
    })
}
```

We use the `for` loop to loop through the input which is a `TokenStream`. A `TokenStream` is made up of token trees. In this example, we don't
care about parsing the token tree, so we simply tune it into a string, we then match against that string, if it's special `[TIME]` string then
we use the `chrono` library to generate a timestamp and append it to the output stream. Otherwise if it's anything else we simply append the
string to the output stream.

Our macro is now complete, let's run it.

As expected, a string was printed out with the prefix `[Info]` and an actual timestamp.

# Procedural Macros - Custom Derive
Let's continue with our example of logging, here we have a trait defined called `Log` with three methods `info`, `warn` & `error`. Each of
these methods takes an immutable borrow to self and a message argument which is a string slice. This trait is implemented on types that can
emit various log messages.

```rust
use procedural_macros::*;

trait Log {
    fn info(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn error(&self, msg: &str);
}

#[derive(Debug)]
struct Database {
    url: String,
    connections: u32,
}
```

Underneath this trait, a struct called `Database` is defined. Let's say we wanted to implement `Log` for this `Database` struct. We could do
this manually but it would be nice to derive a default implementation. We are already deriving a default implementation of `Debug` trait. 
Ideally we should be able to add `Log` to the list as `#[derive(Debug, Log)]`. To do this, we need custom dervied macro.

`Custom Dervied Traits` work with `derive` attribute which allows items such as functions, traits and modules to be automatically implemented 
for user defined types. Let's switch over to `lib.rs` and define a new custom derived macro.

```rust
extern create proc_macro;

use proc_macro::{TokenStream};
use quote::quote;
use chrono::prelude::*;

#[proc_macro_derive(Log)]
pub fn log_derive(input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro]
pub fn log_info(input: TokenStream) -> TokenStream {
    let mut output = "[Info]".to_owned();

    for token in input {
        let token_string = token.to_string();

        match token_string.as_str() {
            "[TIME]" => {
                let time = Utc::now().time().to_string();
                output.push_str(&format!("{} ", time));
            }
            _ => {
                output.push_str(&format!("{} ", token_string));
            }
        }
    }

    TokenStream::from(quote!{
        println!("{}", #output);
    })
}
```

Just like function like macros, we start with a function that takes one input which is a `TokenStream` and returns a `TokenStream`. But instead
of annotating the function with `#[proc_macro]` we annotate it with `#[proc_macro_derive]`. This attribute also takes an argument which is the
identifier that is going to be used in the dervie attribute. The `input token stream` is going to be the item that derive attribute is attached
to, in this case the `Database` struct. Unlike the other macros, return token stream will not replace the input token stream, instead the return
token stream will be appeneded to the containing block module of the annotated item. In this case we are returning the empty token stream, so
our custom derived macro effectively does nothing.

Before creating default implementation of `Log` for the given input let's parse the input into a datastructure that is easier to work with than
a tokenstream. To do this we will add `syn` trait as a dependency.

```toml
...
[lib]
proc-macro = true

[dependencies]
quote = "1.0"
syn = "1.0"
chrono = "0.4"
```

The `syn` trait allows us to parse tokenstream into a syntax tree that is easier to work with. Let's bring `syn` into scope. Then we will parse
`input` into `DeriveInput` struct.

`DeriveInput` datastructure represents items that are annotated with `derive` attribute. It has useful fields like `attrs` than enum or struct
were tagged with, `vis` for visibility, `ident` name of the structure or enum, `generics` and `data` which represents the actual struct or enum.
Then using this name, we will generate a default trait implementation for `Log`.

```rust
extern create proc_macro;

use proc_macro::{TokenStream};
use quote::quote;
use syn;
use chrono::prelude::*;

#[proc_macro_derive(Log)]
pub fn log_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let trait_impl = quote! {
        impl Log for #name {
            fn info(&self, msg: &str) {
                println!("[Info] {}: {}", stringify!(#name), msg)
            }
            fn warn(&self, msg: &str) {
                println!("[Warn] {}: {}", stringify!(#name), msg)
            }
            fn error(&self, msg: &str) {
                println!("[Err] {}: {}", stringify!(#name), msg)
            }
        }
    };

    trait_impl.into()
}

#[proc_macro]
pub fn log_info(input: TokenStream) -> TokenStream {
    let mut output = "[Info]".to_owned();

    for token in input {
        let token_string = token.to_string();

        match token_string.as_str() {
            "[TIME]" => {
                let time = Utc::now().time().to_string();
                output.push_str(&format!("{} ", time));
            }
            _ => {
                output.push_str(&format!("{} ", token_string));
            }
        }
    }

    TokenStream::from(quote!{
        println!("{}", #output);
    })
}
```

Notice that we are also calling `unwrap` after parsing the input, because `parse` returns a `Result` type. If parsing fails we want this function
to panic. `procedural macro` functions return a `TokenStream` not a `Result` type so to notify the compiler of errors, we use panic.

Now that we have a datastructure that is easier to work with, let's extract out the name of the struct or enum being annotated.

Again we use the `quote` macro to transform Rust code into a tokenstream. We create an implementation block, implementing the `Log` trait for
the struct or enum being annotated. To print the name of the struct or enum, we use `stringify` macro which takes an expression and turns it
into a string literal without evaluating that expression.

We call the `into` method on `trait_impl`. `into` will convert the tokenstream returned by the `quote` macro into the tokenstream expected as
the return value. Our custom derive macro is now complete. Let's use it in `main.rs`.

```rust
#[derive(Debug, Log)]
struct Database {
    url: String,
    connections: u32,
}

impl Database {
    fn new(url: String) -> Database {
        Database { url, connections: 0 }
    }

    fn connect(&mut self) {
        self.info(format!("new connection to {}", self.url).as_str());
        self.connections += 1;

        if self.connections >= 100 {
            // a lot of connections 
            self.warn(format!("100 or more connections open!").as_str());
        }
    }
}

fn main() {
    let mut db = Database::new("localhost:5433".to_owned());
    for i in 0..100 {
        db.connect();
    }
}
```

# Procedural Macros - Attribute Like
The last type of procedural macro is an `attribute like macro`. Let' continue with the theme of logging. In below example, we have s struct
called `Product` with a `name` and a `price`.

```rust
#[derive(Debug)]
struct Product {
    name: String,
    price: u32,
}

fn main() {
    let laptop = Product {
        name: "MacBook Pro".to_owned(),
        price: 2000,
    }
    buy_product(laptop, 20);
}

fn buy_product(product: Product, discount: u32) {
    // ...
}
```

Let's say that we wanted to log every time `buy_product` was called. We could do this manually inside the function body, however, it would
be more convenient if we could just add in an attribute to this function that would do that for us. Something that looks like below:

```rust
#[log_call(verbose)]
fn buy_product(product: Product, discount: u32) {
    // ...
}
```

Here we are annotating our function with a procedural macro named `log_call`. `log_call` accepts a boolean argument called `verbose`. If it's
sepcified then the function name and it's arguments will be logged otherwise only the function name will be logged.

Let's define an `attribute like macro` called `log_call`. But before we do that, we need to update `Cargo.toml`. First we need to update the
`syn` create so that it's built with full feature.

```toml
...
[lib]
proc-macro = true

[dependencies]
quote = "1.0"
syn = { version = "1.0", features = ["full"] }
chrono = "0.4"
darling = "0.13"
```

This will allow us to parse functions. We will also add a dependency on a crate called `darling`. `darling` allows us to parse `proc-macro`
attributes into structs.

Let's switch over to `lib.rs` and define skeleton of our macro.

```rust
#[proc_macro_attribute]
pub fn log_call(args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
```

Attribute like procedural macros are annotated with `proc_macro_attribute`. `Attribute Like Macros` are also unique because instead of one
argument they take two arguments, `args` is a token tree of the arguments passed to our custom attribute. In this case we are going to accept
an optional argument called `verbose`. Second argument is a token tree which represents the item we are annotating.

Unlike `custom derived macros`, `attribute like macros` can annotate more than structs and enums e.g. in this case we are annotating a function.
Also unlike `custom derived macros` the return tokenstream will replace the annotated item.

Now that we have skeleton of `log_call` defined, let's create a struct which will hold the arguments passed to our attribute macro. First we
will import `FromMeta` trait from the `darling` library. `FromMeta` is used to extract values from a meta item in an attribute. Next we will 
create a struct called `MacroArgs` which derives from `FromMeta`. `MacroArgs` contains one field called `verbose` which is a boolean. We also
annotate the field with `default`, so that it's set to false by default.

```rust
#[derive(FromMeta)]
struct MacroArgs {
    #[darling(default)]
    verbose: bool,
}
```

Next let's parse the two arguments passed into `log_call`.

```rust
#[derive(FromMeta)]
struct MacroArgs {
    #[darling(default)]
    verbose: bool,
}

#[proc_macro_attribute]
pub fn log_call(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let mut input = parse_macro_input!(input as ItemFn);
    input
}
```
`parse_macro_input` parses tokenstreams passed into macros and turns them into syntax trees. In this case, we are turning the `args` tokenstream
into `AttributeArgs` and the `input` tokenstream into `ItemFn`. Because `input` is no longer a tokenstream we can't return it. Let's just return
empty tokenstream for now.

```rust
#[proc_macro_attribute]
pub fn log_call(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let mut input = parse_macro_input!(input as ItemFn);
    TokenStream::new();
}
```

Next we will take `attr_args` and transform it into `MacroArgs` struct, we defined earlier.

```rust
#[proc_macro_attribute]
pub fn log_call(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let mut input = parse_macro_input!(input as ItemFn);

    let attr_args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    }

    TokenStream::new();
}
```
First we call `from_list` functions on `MacroArgs` and pass in `attr_args`. This function returns a `Result` type containing either `MacroArgs`
or an error. If we get the `Ok` variant we will simply return the `MacroArgs`, otherwise if we get an error then we will return a tokenstream
with that error.

Now that the arguments are parsed, let's create another function that will hold the implementation of our macro.

```rust
fn impl_log_call(attr_args: &MacroArgs, input: &mut ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;

    input.block.stmts.insert(0, parse_quote!{
        println!("[Info] calling {}", stringify!(#fn_name));
    });
    
    input.to_token_stream().into()
}

#[proc_macro_attribute]
pub fn log_call(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let mut input = parse_macro_input!(input as ItemFn);

    let attr_args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    }

    impl_log_call(&attr_args, &mut input);
}
```

Now let's use it in `main.rs`.

```rust
fn main() {
    let laptop = Product {
        name: "MacBook Pro".to_owned(),
        price: 2000,
    }
    buy_product(laptop, 20);
}
#[log_call]
fn buy_product(product: Product, discount: u32) {
    // ...
}
```

If we run it we can see the function call being logged.

Now let's account for `verbose` variable. Let's modify `impl_log_call` to implement that.

```rust
fn impl_log_call(attr_args: &MacroArgs, input: &mut ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;

    if attr_args.verbose {
        // 1. Extract argument names
        let fn_args = extract_arg_names(input);
        // 2. Generate verbose log message
        // 3. Prepend verbose log message to function block
    } else {
        input.block.stmts.insert(0, parse_quote!{
            println!("[Info] calling {}", stringify!(#fn_name));
        });
    }
    
    input.to_token_stream().into()
}

fn extract_arg_names(func: &ItemFn) -> Vec<&Ident> {
    func.sig.inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(pat_Type) = arg {
            if let Pat::Ident(pat) = &(*pat_type.pat) {
                return Some(&pat.ident);
            }
        }
        None
    }).collect()
}
```

Each argument is a `FnArg`. `FnArg` is an enum which could either be `Receiver`, if the argument is self or `Typed` if argument is something
other than self.

`Typed` variant contains a `PatType`. `PatType` is a pattern representing the argument e.g. `foo: f64`. We only care about the function name,
and to get that name we need to access the `pat` field from `PatType`, which is a `Box Smart Pointer` containing `Pat`. Inside of `Pat` we
want to access `Ident` field which contains a `PatIdent`. Finally `PatIdent` contains a field called `ident` which is the argument name.

Our first `if_let` statement gives us a `PatType`. Then we call `pat_type.pat` and dereference the `Box Smart Pointer`. This will give us a
`Pat` object, which we take a reference to.
Then we use another `if_let` statement to check that `pat` is `Ident` variant and if it is we return a reference to `pat.ident` wrapped in
the some variant. Otherwise we return the `None` variant.

Because we are calling `filter_map` all the `None` variants will be filtered out and we will get an `Iterator` over identifiers and finally
we call `collect` to turn that iterator into a vector.

This function is now complete, next we need to generate a verbose log message.

```rust
fn impl_log_call(attr_args: &MacroArgs, input: &mut ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;

    if attr_args.verbose {
        // 1. Extract argument names
        let fn_args = extract_arg_names(input);
        // 2. Generate verbose log message
        let statements = generate_verbose_log(fn_name, fn_args);
        // 3. Prepend verbose log message to function block
    } else {
        input.block.stmts.insert(0, parse_quote!{
            println!("[Info] calling {}", stringify!(#fn_name));
        });
    }
    
    input.to_token_stream().into()
}

fn extract_arg_names(func: &ItemFn) -> Vec<&Ident> {
    func.sig.inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(pat_Type) = arg {
            if let Pat::Ident(pat) = &(*pat_type.pat) {
                return Some(&pat.ident);
            }
        }
        None
    }).collect()
}
```

`generate_verbose_log` will take the function name and function arguments as input and then return a vector of statements. Let's implement
this function.

```rust
fn generate_verbose_log(fn_name: &Ident, fn_Args: Vec<&Ident>) -> Vec<Stmt> {
    let mut statements = vec![parse_quote!{
        print!("[Info] calling {} | ", stringify(#fn_name));
    }];

    for arg in fn_args {
        statements.push(
            parse_quote! {
                print("{} = {:?} ", stringify!(#arg), #arg);
            }
        );
    }

    statements.push(
        parse_quote! { println!(); }
    );

    statements
}
```

Lat thing we need to do is prepend the vector of statements to the function block being annotated. We can do that using `splice` method.

```rust
fn impl_log_call(attr_args: &MacroArgs, input: &mut ItemFn) -> TokenStream {
    let fn_name = &input.sig.ident;

    if attr_args.verbose {
        // 1. Extract argument names
        let fn_args = extract_arg_names(input);
        // 2. Generate verbose log message
        let statements = generate_verbose_log(fn_name, fn_args);
        // 3. Prepend verbose log message to function block
        input.block.stmts.splice(0..0, statements);
    } else {
        input.block.stmts.insert(0, parse_quote!{
            println!("[Info] calling {}", stringify!(#fn_name));
        });
    }
    
    input.to_token_stream().into()
}
```

`splice` will take in a range and replace it with the passed in `Iterator` or something that can be turned into an `Iterator`.

