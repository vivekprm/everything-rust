# Panics
In Rust if you endup in error state that's unrecoverable, where the programmer doesn't know how to handle that state then your program
can panic. Panicking will cause your program to printout an error message and exit abruptly. To get our program to panic we can use the
`panic` macro and pass in a failure message.

```rust
fn main() {
    panic!("Sonething went wrong!");
}
```

In this example, we have explicitly told our program to panic. Let's look at another example where our program panics because of a bug
in our code.

```rust
fn main() {
    let v = vec!["one", "two", "three"];
    println!("{}", v[3]);
}
```

Now when we run our program, it panics with "index out of bounds" message. The console output also tells us that we can get a backtrace
by setting the `RUST_BACKTRACE` environment variable to 1. Let's set this variable and run our program again. Now we get a backtrace showing
the chain of function calls causing our program to panic. If we wanted even more information we can set `RUST_BACKTRACE` to `full`.

# Recoverable Errors
In Rust if you get into a situation where an operation could succeed or could fail in a recoverable way, you can use the **Result** enum.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `Result` enum has two variants:
- `Ok` which will return when operation succeeds and holds a generic value.
- `Err` which will be returned if the operation fails and holds a generic error.

To see how it works, let's create a new function called `get_user_id`.

```rust
fn main() {

}

fn get_user_id(username: &str) -> Result<i32, String> {
    if username.is_empty() {
        Err("Username can't be empty".to_owned())
    } else {
        Ok(1)
    }
}
```

Next let's look at how `Result` is used in the standard library by doing a little bit of file handling.

```rust

    let file = match file {
        Ok(file) => file,
        Err(error) => {
            panic("Failed to open file: {:?}", error)
        }
    };
}
```

To ignore error handling may be in prototype application, we can use `unwrap` or `expect` method on the `Result` type. E.g.

```rust
use std::fs::File;

fn main() {
    let file = File::open("example.txt").unwrap();
}
```

If `open` returns the `Ok` variant, `unwrap` will return the value stored inside the `Ok` variant. Otherwise if an `Err` type is
returned, `unwrap` will panic.

`expect` method is similar to `unwrap` except you can provide a custom failure message.

```rust
use std::fs::File;

fn main() {
    let file = File::open("example.txt").expect("Failed to open file!");
}
```

Except `unwrap` and `except` they are many other useful methods on `Result` type.

# Propagating Errors
To understand this we will create a `read_file` function which will take a filename and then read the contents of that file. The return type
of this function is going to be a Result Enum.

```rust
use std::{io::{self, Read}, fs::File};

fn main() {
    let contents = read_file("example.txt");
}

fn read_file(filename: &str) -> Result<String, io::Error>) {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

`?` operator unwraps valid values or returns erroneous values, propagating them to the calling function. In this case if `open` method returns
`Ok` variant then our file variable is going to be set equal to the value inside `Ok`, in this case file type. Otherwise if `open` returns `Err`
variant then our function will return early with that error variant, tn this case it's going to be an `io::Error`.

`read_to_string` will read the bytes of a file and store them into the passed in string. Notice the `?` at the end of `read_to_string` call, because
`read_to_string` returns a `Result` type. If an `Err` variant is returned we want our function to return early with that error. Otherwise, we
will return an `Ok` variant with the contents of the file.

`?` operator saved us from writing boiler plate code.

We can make this code little more concise by inlining call to `open`.

```rust
fn read_file(filename: &str) -> Result<String, io::Error>) {
    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;
    Ok(contents)
}
```

The `?` operator also works with Optional values. E.g. let's say we have a struct called `User` and a function called `get_initials`.

```rust
struct User {
    firstname: String,
    lastname: String
}

fn get_initials(user: User) -> Option<String> {
    let first_initial = user.firstname.chars().next()?;
    let last_initial = user.lastname.chars().next()?;
    Some(format!("{first_initial}.{last_initial}."));
}
```

# Result & Option
In this example we have a function called `read_first_line` which takes a filename as a parameter and returns a `Result` type. Containing
either the first line in the file or an io error.

```rust
use std::{fs, io};

fn main() {
    let first_line = read_first_line("example.ext");
}

fn read_first_line(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename).and_then(|s| {
        s.lines().next().map(|s| s.to_owned())
    })
}
```

The `read_to_string` function returns a `Result` type, we call the `and_then` method on that `Result` type. If we get the `Ok` variant,
the value stored inside that `Ok` variant is passed to a **Closure** which in-turn returns a `Result` type. Otherwise we return the error.

For now consider `closures` as anonymous functions which we can pass into other functions. In this case our closure gets the argument `s`,
which is the content stored within the file and it's of type String. Then to get the first line in our file we call `s.lines()` which will
return an iterator over the lines in a string then `next()` will give us the first line and finally `map`. `map` is used to take one value 
and transform it into another value. In this case `next()` returns an `Option` containing a string slice and we want to convert that to 
Option containing an owned string.

Notice that we get an error "mismatched type". Our closure is expected to return `Result<_, std::io::Error>` however, we are returning
`Option<String>`.

The reason we are getting an `Option<String>` here, because `next()` returns an `Option` type and that's because we might not have first
line in the file.

While developing in Rust you will find yourself in these situations where you have to deal with both `Result` types and `Option` types.
- Oneway to handle these cases is to wrap an `Option` in a `Result` or wrap a `Result` in an `Option`. E.g. in case of success, instead
of returning a `String` we can return an `Option<String>`. And instead of calling `and_then` method, we will call `map`, because in this case
we are mapping a `Result<String>` to `Result<Option<String>>`.

```rust
fn read_first_line(filename: &str) -> Result<Option<String>, io::Error> {
    fs::read_to_string(filename).map(|s| {
        s.lines().next().map(|s| s.to_owned())
    })
}
```

- The otherway to handle these cases is to convert between `Option` and `Result`. So let's change our method so that it returns `Option`
instead of `Result` so instead of `map` we will call `ok()` followed by `and_then()`:

```rust
fn read_first_line(filename: &str) -> Option<String> {
    fs::read_to_string(filename).ok().and_then(|s| {
        s.lines().next().map(|s| s.to_owned())
    })
}
```

The `ok()` method takes a `Result` type and converts it to an `Option` type. `and_then` converts that `Option` to an `Option` containing
first line in our file.

Methods, such as `ok`, `and_then`, `map` etc. are known as `combinators`. 

`Combinators` are functions which perform operations on a value and can even change the value. The cool thing about `Combinators` is that
you can chain function calls.

# Multiple Error Types
In this example we have a function called `parse_file` which accepts a file name and returns a `Result` type containing either an integer
or an io error. This function will take the content of a file and attempt to parse them into an integer.

```rust
use std::{fs, io};

fn main() {
    let i = parse_file("example.txt");
}

fn parse_file(filename: &str) -> Result<i32, io::Error> {
    let s = fs::read_to_String(filename)?;
    let i = s.parse()?; // error
    Ok(i)
}
```

Just like `read_to_string`, `parse` returns a `Result` type, so we use a `?` operator to propagate any error. But as you can see, we get
 a compile time error which states that "`?` couldn't convert the error to `std::io::Error`". The problem is `read_to_string` and `parse`
return different errors, but our return type only accounts for `io::Error`. In Rust, there are a couple of ways to handle such situations
where function return different types of errors.
- Instead of specifying a concrete error type we can specify a trait object.
```rust
fn parse_file(filename: &str) -> Result<i32, Box<dyn error::Error>> {
    let s = fs::read_to_String(filename)?;
    let i = s.parse()?; // error
    Ok(i)
}
```

Here we are saying that, result will either contain an integer or any type that implements the `Error` trait defined in the `error` module.
`read_to_string` and `parse` return errors defined in standard library which all implement the `Error` trait.

Also not that, even though `read_to_string` and `parse` return concrete errors, `?` operator will implicitly convert those concrete errors
into the trait object.

The benefit of this approach is that it makes error handling simple. The downside is the callers of our function will not know what
concrete error types are being returned at compile time and therefore can't handle individual errors differently.

So this approach should be used when callers of a function don't care about the specific errors being returned.

- Another approach is to create custom error enum where the variance are the different types of errors a function could return. e.g. let's
create an enum called `ParseFileError`

```rust
use std::{fs, io, error, num::ParseIntError};

fn main() {
    let i = parse_file("example.txt");
}

enum ParseFileError{
    File,
    Parse(ParseIntError)
}

fn parse_file(filename: &str) -> Result<i32, ParseFileError> {
    let s = fs::read_to_string(filename).map_err(|e| ParseFileError::File?);
    let i = s.parse().map_err(|e| ParseFileError::Parse(e))?;
    Ok(i)
}
```

The `map_err` method will map a result of one error type to a result of another error type. In the case of `read_to_string` we map any error
to the `File` variant of `ParseFileError` and in the case of `parse`, we take the `ParseIntError` that may be returned and store inside the 
`Parse` variant of `ParseFileError`. 

Note that we are still using the `?` operator at the end of our calls so that our `ParseFileError` are propagated.

Implementing error handling this way requires more code, however the advantage is callers of this function can now distinguish between a 
File error and a Parse error. To demonstrate that let's add a match expression in main.

```rust
fn main() {
    let i = parse_file("example.txt");

    match i {
        Ok(i) => println!("{i}")
        Err(e) => {
            match e {
                ParseFileError::File => {
                    // ...
                },
                ParseFileError::Parse(e) => {
                    // ...
                }
            }
        }
    }
}
```
