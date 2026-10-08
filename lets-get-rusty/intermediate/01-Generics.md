# Generics
Generics allow us to define structs, enums and functions with Generic types that will be substituted for concrete types
at compile time. E.g. Let's define a new struct called `Browser Command`.

```rust
struct BrowserCommand {
    name: String,
    payload: String,
}

fn main() {
    let cmd1 = BrowserCommand {
        name: "navigate".to_owned(),
        payload: "https://www.letsgetrusty.com".to_owned(),
    };

    let cmd2 = BrowserCommand {
        name: "zoom".to_owned(),
        payload: 200
    }
}
```

In above code, cmd2 give error while defining payload as it expects string, however for zoom command we want a number to be passed.
Similarly there can be a command which takes tuple as payload. To allow apyload to support a variety of concrete types, we can use
generics. Let's make that change:


```rust
struct BrowserCommand <T> {
    name: String,
    payload: T,
}

fn main() {
    let cmd1 = BrowserCommand {
        name: "navigate".to_owned(),
        payload: "https://www.letsgetrusty.com".to_owned(),
    };

    let cmd2 = BrowserCommand {
        name: "zoom".to_owned(),
        payload: 200
    }
}
```

Convention is to use T for generic type. If we want to use another generic type we can use alphabets afterwards like U, V etc.
Another convention is to use T followed by interger e.g. T0, T1 etc. For more meaning we can give a a name in camel case
e.g. `BrowserCommand<PayloadType>`.

Next let's talk about generics in implementation blocks. First we will create an implementation block for BrowserCommand. 
Convention is to use `<T>` after the impl keyword and the struct we are implementing.


```rust
struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand {
            name,
            payload,
        }
    }
}

fn main() {
    let cmd1 = BrowserCommand::new(
        "navigate".to_owned(),
        "https://www.letsgetrusty.com".to_owned(),
    );

    let cmd2 = BrowserCommand::new(
        "zoom".to_owned(),
        200
    );
}
```

Lets talk about impl block and why we had to define `<T>` twice?
The reason we need to declare `<T>` after the `impl` keyword and after the sruct name is, so that Rust knows that we are
implementing functionality for BrowserCommand struct of any type rather than a concrete type.

To clarify the difference, lets create an implementation block for a concrete type. In Rust we can define multiple impl
blocks on the same type. Lets define another impl block and instead of using `T` lets use concrete type `String`.

```rust
impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}
```

Reason we are able to print payload in this implementation is, we know that it's a string. Let's call print_payload on our 
commands.

```rust
fn main() {
    let cmd1 = BrowserCommand::new(
        "navigate".to_owned(),
        "https://www.letsgetrusty.com".to_owned(),
    );

    let cmd2 = BrowserCommand::new(
        "zoom".to_owned(),
        200
    );
}
cmd1.print_payload();
cmd2.print_payload(); // error
```

calling print_payload doesn't work on cmd2 as payload is integer.

So far we have defined methods that take generic paramters, now let's define function that returns generic paramter.

Let's add `get_payload` function in our first implementation block.

```rust
impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand {
            name,
            payload,
        }
    }

    fn get_payload(&self) -> &T {
        &self.payload
    }
}
```

`get_payload` takes an immutable reference to self and returns a reference to `payload`. In our return type we use `T` 
because payload is generic. Let's call our new method in main.

```rust
fn main() {
    let cmd1 = BrowserCommand::new(
        "navigate".to_owned(),
        "https://www.letsgetrusty.com".to_owned(),
    );

    let cmd2 = BrowserCommand::new(
        "zoom".to_owned(),
        200
    );
    cmd1.print_payload();

    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();
}
```

So far we have been using Generics in Struct definitions however, generics can also be used in Enum definitions. We have
already seen it with `Option` and `Result` enum.

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Besides Enum, Structs and Implementation blocks generics could also be used in free functions, functions that are not tied to
Enum, Struct & trait. E.g. lets define a function called `SerializePayload`.

```rust
fn serialize_payload<T>(payload: T) -> String {
    // Convert payload to JSON string...
    "placeholder".to_owned();
}

fn main() {
    let cmd1 = BrowserCommand::new(
        "navigate".to_owned(),
        "https://www.letsgetrusty.com".to_owned(),
    );

    let cmd2 = BrowserCommand::new(
        "zoom".to_owned(),
        200
    );
    cmd1.print_payload();

    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();

    serialize_payload(p1);
    serialize_payload(p2);
}
```

As you can see we were able to pass payload1 and payload2 both even though they are different types.

# How Generics Work in Rust
When using Generics Rust there is no runtime performance cost. Defining `serialize_payload` with generics is just as fast as having
two separate serialize functions, one that takes string payload and the other that takes integer payload and that's because that's
exactly what Rust does at compile time through a process called **Monomorphization**.

At compile time, Rust will take generic `serialize_payload` function and create two concrete function, one that takes `String` and 
other one that takes signed 32-bit integer. Rust will then update the call sites to update call to these concrete functions.
