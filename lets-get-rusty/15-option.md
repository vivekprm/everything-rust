# Option
Let's imagine we are implementing the following program, if user_id is 1 we return db result else we return
Option enum. If other languages we have null. Options enum looks like below:

```rust
enum Option<T> {
    None,
    Some(T),
}
```
Option enum has two variants:
- Some variant which contains a generic value 
- None variant 

Option enum and it's variants are in the Rust prelude, which is a list of things that Rust automatically imports
to every Rust program. So we can start using them without any extra code.

```rust
fn main() {
    let username = get_username(1);
    println!("{username}");
}

fn get_username(user_id: i32) -> String {
    // get username from database
    let db_result = String::from("Ferris");

    if user_id == 1 {
        db_result
    } else {
        // return option
    }
}
```

Let's use Option in our above program. First thing that we need to do is change function signature.

```rust
fn main() {
    let username = get_username(1);
    match username {
        Some(name) => println!("{name"),
        None => {}
    }   
}

fn get_username(user_id: i32) -> Option<String> {
    // get username from database
    let db_result = String::from("Ferris");

    if user_id == 1 {
        Some(db_result)
    } else {
        None
    }
}
```

Above program works but the code could be improved, None match that does nothing is little bit strange. In this
situation we only care is the Some variant.

In situations where we only want to handle one variant and ignore all the other variants, you can use `If let`
syntax.

```rust
fn main() {
    let username = get_username(1);
    if let Some(name) = username {
        println!("{}", name)
    }
}
```


