# Result
In the last lession we implemented get_username function, which takes user_id as parameter and returns username
as Option<String> value.

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

Let's improve this little bit by introducing a function that allows us to execute database queries. Quering
database may result in failure may be because of network connections etc.

Rust has a builtin enum to handle situations where an operation could return a valid value or an error. That
enum is called **Result**.

The **Result** enum has two variants:
- Ok(T): holds a value
- Err(E): holds an error

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Just like Option enum, Result enums are automatically added in every Rust program.

```rust
fn main() {
    let username = get_username(1);
    match username {
        Some(name) => println!("{name"),
        None => {}
    }   
}

fn get_username(user_id: i32) -> Option<String> {
    let query = format!("GET username FROM users WHERE id={user_id}");
    // get username from database
    let db_result = query_db(query);

    db_result.ok()
}

fn query_db(query: String) Result<String, String> {
    if query.is_empty() {
        Err(String::from("Query string is empty"))
    } else {
        Ok(String::from("Ferris"))
    }
}
```
