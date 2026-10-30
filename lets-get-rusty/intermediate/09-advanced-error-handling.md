# Error Handling
It's compilcated in nature because we have to deal with several different things:
- We have to think about **Defining** errors.
- We have to think about **Propagating** errors.
- **Handling or Discarding** errors.
- **Reporting** errors to end users and to developers.

If you look at the above list, you'll notice that errors have three different audiances.
- Machines or programs which needs error for control flow.
    - Programs use errors to decide which execution path to take.
- Endusers which needs friendly error messages.
- Developers need detailed diagnostics to debug the error. 

Error handling is active area of developement in the Rust community.

Rust Language team has special working group focused on making error handling better in Rust.

In Rust there are two types of errors:
- Non-recoverable Errors: Thrown with panic macro.
- Recoverable Errors

# Defining Idiomatic Recoverable Errors
In Rust **Idiomatic Errors** should implement the **Error** trait. The **Error** trait defines the bare minimum requirement for a good error.
This trait has one method called `source` which optionally returns the underlying source or cause of the error and it's default implementation
returns `None`. 

The return type is optional containing a reference to a trait object. The trait object could be any type that implements the `Error` trait and
the `static` trait bound. The `static trait bound` means that the type can't have any non-static references. This means that we can hold on to 
the type as long as we want because the memory will always be valid.

```rust
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static')> {
        None
    }
}
```

The `Error` trait also has a `backtrace` method. However, currently that's only available in `nightly` release of Rust.

The `Error` trait has two super traits `Debug` and `Display`. 
- The `Debug` trait describes how the error should be reported to developers.
- The `Display` trait describes how the error should be reported to users.

Implementing the `Error` trait on your custom error types achieves a few things:
- First of all, it's a way to semantically mark your types as errors.
    - Meaning that when developers look at your struct or your enum and see that it implements `Error` trait, they know to treat it as an Error.
- Second of all, the `Error` trait standardizes a few things:
    - It standardizes checking the source of the error with the `source` method.
    - User facing reports with the `Display` trait.
    - Developer facing reports with the `Debug` trait.

The `Error` trait is defined in standard library. Error handling infrastructure and third party libraries are built on top of this trait.
So it's important that your type implements this trait, so that they work with error handling ecosystem.

Now let's discuss how to actually structure you custom error types.

# Custom Error Types
Custom Error Types can be structs or Enums. In this case we have a struct called `ServerError`, with three fields:

```rust
struct ServerError {
    status_code: u8,
    body: String,
    source: Box<dyn Error>
}
```

`status_code` is going to be a http status code.
`body` is the error message string.
`source` is the underlying error.

Using structs for custom error types is appropriate when you want to attach more context to your error and your code doesn't need to 
distinguish between different error types.

If your code does need to distinguish between different error types then an `Enum` is more appropriate.

Here is an example of Enum error type called `APIError`. It has two variants `UserInputError` with a custom error string and `InternalError` 
which contains the underlying error:

```rust
enum APIError {
    UserInputError(String),
    InternalError(Box<dyn Error>)
}
```

Because this enum has two variants, your code can distinguish between user input errors and internal errors. This is useful when you want
to change your code's behaviour based on the different variants. Enum error types are what you are going to see in use for most of the time.

Lastly, you can mix `Enums` and `Structs`.

```rust
enum APIErrorType {
    UserInputError,
    InternalError
}

struct APIError {
    msg: String,
    source: Option<Box<dyn Error>>,
    err_type: APIErrorType
}
```

In this example, we have a custom error type called `APIError`, which is a struct containing three fields and one of the field points to
`APIErrorType` enum.

This approach is flexible because you can add a lot of context to the struct and you can still pattern match on the error type.

# Basic Error Handling
In the following example, we have a CLI program which takes a name as input and returns credit card information from some sort of persisted
storage. Persisted storage could be a database, a file etc. but for simplicity in this example it's simply a hashmap.

```rust
fn main() {
    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27 123"),
        ("Bob", "1234567 12 27 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());

    println!("\nCredit Card Info: {result:?}");
}

fn get_credit_card_info(credit_cards: &HashMap<&str, &str>, name: &str) -> Card {
    let card_string = credit_cards.get(name).unwrap();
    let card = parse_card(card_string);

    card
}

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32,
}

fn parse_card(card: &str) -> Card {
    let mut numbers = parse_card_numbers(card);

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Card {
        number,
        exp: Expiration { year, month },
        cvv
    }
}

fn parse_card_numbers(card: &str) -> Vec<u32> {
    let numbers = card.split(" ")
        .into_iter()
        .map(|s|{
            s.parse()
        })
        .collect::<Result<Vec<u32>, _>>()
        .unwrap();

    numbers
}
```

In this hashmap, keys are name and values are credit card information stored as a string of four numbers. The credit card number, expiration
month, expiration year and the cvv.

`parse_card_numbers` takes `card` string and split it by space character. Then we turn that into a vector using `into_iter` function. Next
we map over each substring and turn it into an unsigned 32-bit integer. Finally, we call collect and turn that iterator into a `Result` type
containing a `Vector` of unsigned 32-bit integers or an error which we ignore.

With correct input this program works, however we are not doing any error handling, instead we are calling `unwrap` everywhere. Which means
our program will panic if we get the error variant.

Let's test this with different error scenarios:
- Enter invalid name: Our program panics saying "we called `unwrap` on an `Option` type whose variant was `None`."
    - This message is not really helpful, we have no idea why the optional value was `None`.
- Missing CVV: Again we get the same error message as in above case, eventhough it's a different error case.
- Payment information malformed: Instead of month in number mention it in words e.g. Dec. This time we get a slightly better error saying
`ParseIntError` invalid digit. But we are still missing a lot of context, e.g. whose credit card is being parsed and which part of the
credit card info was invalid digit.

Let's improve this output by introducing logging and error handling to this CLI application.

First thing we need to do is get rid of all the `unwrap` statements. So that our program doesn't crash when an error occurs.

```rust
fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParseIntError> {
    let numbers = card.split(" ")
        .into_iter()
        .map(|s|{
            s.parse()
        })
        .collect::<Result<Vec<u32>, _>>()?;

    Ok(numbers)
}
```

Now let's change `parse_card` function.

```rust
fn parse_card(card: &str) -> Result<Card, String> {
    let mut numbers = parse_card_numbers(card).map_err(|e| e.to_string())?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(format!("Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}"))
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Ok(Card {
        number,
        exp: Expiration { year, month },
        cvv
    })
}
```

We are propagating error using `?` operator.

Now let's change get_cred_card_info function.

```rust
fn get_credit_card_info(credit_cards: &HashMap<&str, &str>, name: &str) -> Result<Card, String> {
    let card_string = credit_cards.get(name).ok_or(format!("No credit card was found for {name}."))?;
    let card = parse_card(card_string)?;

    Ok(card)
}

```

ok_or() method converts and `Option` to `Result` type.

Now let's fix the main:

```rust
fn main() {
    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27 123"),
        ("Bob", "1234567 12 27 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());

    match result {
        Ok(card) => println!("\nCredit Card Info: {card:?}"),
        Err(err) => println!("{err}"),
    }
}
```

Now let's run the above error cases again. Now we see friendly messages with more context.

It's better than previous code but it still has few problems:
- Because all these error variants are strings, we can't react programatically to different error cases. E.g. distinguishing against user
errors e.g. putting the wrong name versus internal error such as last two cases.

At this level of abstraction user should get a friendly error message if their input is invalid. Becuase that's something they could fix.
However, they shouldn't be exposed to details of internal errors, because that's something they can't fix.

This brings up the second problem which is that there is no separation between user level reporting and developer level reporting. While
end user should not see details of internal errors, developers should see these details because they could actually fix them.

The last problem is as developer not only we want to see the internal details of these errors but we also want to see an error chain, meaning
the top level error and also all the errors that caused it.

To fix these issues we can introduce the custom error types and logging.

# Custom Errors 1
First thing we need to do is distinguish between **UserInputError** versus internal errors. If we look at `get_credit_card_info` function, the
only user error in this application so far is generated, when user enters an invalid name and we say "No credit card was found for {name}".

Internal Error could possibly be returned when calling the `parse_card` function. `get_credit_card_info` returns a `Result` type, where the
error type is string. If we want to disambiguate between User errors and Internal errors, we need to change String to a different type. E.g.
an Enum with two variants.

```rust
enum CreditCardError{
    InvalidInput(String),
    Other(String, String),
}

fn get_credit_card_info(credit_cards: &HashMap<&str, &str>, name: &str) -> Result<Card, CreditCardError> {
    let card_string = credit_cards.get(name).ok_or(
        CreditCardError::InvalidInput(format!("No credit card was found for {name}."))
    )?;
    let card = parse_card(card_string)
        .map_err(|e| {
            CreditCardError::Other(e, format!("{name}'s card couldn't be parsed."))
        })?;

    Ok(card)
}
```

If the user inputs an invalid name, we return `InvalidInput` variant, containing a custom error message and if we get an internal error, we 
return the `Other` variant containing the underlying error and a custom error message. Now let's update `main` function.

```rust
fn main() {
    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27 123"),
        ("Bob", "1234567 12 27 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());

    match result {
        Ok(card) => println!("\nCredit Card Info: {card:?}"),
        Err(err) => {
            match err {
                CreditCardError::InvalidInput(msg) => println!("{msg}"),
                CreditCardError::Other(_, _) => println!("\nSomething went wrong! Try again!")
            }
        },
    }
}
```

From user's perspective that's all they need to know because they can't debug the internal errors. However, we still want to give details to
developers about internal errors. To do that, we can log the error, however we first need to import few crates. Two crated we need are `log`
and `env_logger`.

```toml
...
[dependencies]
log = "0.4"
env_logger = "0.9"
```

`log` is a rust library which provides a light weight facade for other log libraries and `env_logger` is an implementation of `log` that
uses environment variables to set log levels. We are also logging the error with debug formatting which means `CreditCardError` needs to
derive the `Debug` trait. Now let's initialize and use it in `main`:

```rust
#[derive(Debug)]
enum CreditCardError{
    InvalidInput(String),
    Other(String, String),
}

fn main() {
    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27 123"),
        ("Bob", "1234567 12 27 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());

    match result {
        Ok(card) => println!("\nCredit Card Info: {card:?}"),
        Err(err) => {
            match err {
                CreditCardError::InvalidInput(msg) => println!("{msg}"),
                CreditCardError::Other(_, _) => println!("\nSomething went wrong! Try again!")
            }

            log::error!("\n{err:?}");   // error
        },
    }
}
```

We are getting the error saying "we are borrowing a partially moved value" and that's because when we match `err` in the `InvalidInput` case,
we are taking ownership of msg. To fix this, instead of matching against `err`, we will match against a reference to err.

```rust
match result {
        Ok(card) => println!("\nCredit Card Info: {card:?}"),
        Err(err) => {
            match &err {
                CreditCardError::InvalidInput(msg) => println!("{msg}"),
                CreditCardError::Other(_, _) => println!("\nSomething went wrong! Try again!")
            }

            log::error!("\n{err:?}");
        },
    }
```

Let's run our program first with logging turned off using:

```sh
RUST_LOG=off cargo run
```

Next time run with error log using:

```sh
RUST_LOG=error cargo run
```

This is better than what we had before but there are still a few issues. The errors themselves are hard to read and don't provide enough
information. e.g. what was the inavlid digit in credit card string. Let's improve that.

# Custom Errors 2
So far we added one custom error called `CreditCardError`, which is used in `get_credit_card_info` function. Let's add another custom error
for our parsing function. So that we can attach more information to the error. This time instead of `enum` we will create a struct called
`ParsePaymentInfoError`.

```rust
struct ParsePaymentInfoError {
    source: Option<Box<dyn Error>>,
    msg: Option<String>
}
```

It contains two fields, `source` represents the underlying error type. It's going to be an optional containing a Trait object. Here we are
saying this optional can store any time that implements the `Error` trait. `msg` is going to be an optional error message for more context.

The reason we are using a struct instead of an enum is because in this case we don't need to distinguish between the various parsing errors.    
All we want to do is add more context to the error. Next let's update `parse_card_number` to return `ParsePaymentInfoError` instead of 
`ParseIntError`.

```rust
fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card.split(" ")
        .into_iter()
        .map(|s|{
            s.parse()
        })
        .collect::<Result<Vec<u32>, _>>()?; // error

    Ok(numbers)
}
```

We get an error because Rust can't automatically convert a `ParseIntError` to a `ParsePaymentInfoError`. We can fix this in couple of ways:
- Using `map_err` method, we used before.
- We can implement `from` trait on `ParsePaymentInfoError`. The `from` trait is used to convert a value of one type to a value of different type.

```rust
impl From<ParseIntError> from ParsePaymentInfoError {
    fn from(e: ParseIntError) -> Self {
        ParsePaymentInfoError{
            source: Some(Box::new(e)),
            msg: None
        }
    }
}
```

Because we have implemented the `from` trait `?` operator automatically converts `ParseIntError` into `ParsePaymentInfoError` and propagates it.
`from` trait is useful and convenient, however in this case it won't work that's because we want to attach a custom message but inside `from`
function above we don't have enough context to attach that message, so instead we are going to use `map_err`.

```rust
fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card.split(" ")
        .into_iter()
        .map(|s|{
            s.parse().map_err(|_| ParsePaymentInfoError{
                source: None,
                msg: Some(format!("{s:?} couldn't be parsed as u32"))
            })
        })
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|e| ParsePaymentInfoError{
            source: Some(Box::new(e)), // error
            msg: Some(format!("Failed to parse input as numbers. Input: {card}"))
        })?;

    Ok(numbers)
}
```

After calling `collect` we want to map this lower level parsing error to a higher level parsing error which states "Failed to parse input 
as numbers and specifies the input". However when we try to set this lower level error as a source, we get an error because 
`ParsePaymentInfoError` doesn't implement `std::error::Error` trait. Remember `source` must be a trait object which is anything that implements
the `Error` trait. Let's go ahead and implement it for `ParsePaymentInfoError`.

```rust
impl Error for ParsePaymentInfoError {  // error
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}
```

Here we are implementing the `source` function and returning `self.source`. We also have to call `as_deref` because the type of `source` is
a reference to `Option<Box<dyn Error>>` but what we need to return is `Option<&(dyn Error + 'static)>`. `as_deref` will take a reference to 
a Option containing a value `T` and convert it to Option with reference to `T` while also calling `deref` on `T`. Which will turn our `Box`
into a reference.

We still get a compile time error because implementing the `Error` trait also requires that we implement `Display` and `Debug`.
For `Debug` we can simply derive the trait. But for `Display` we have to implement it ourselves.

```rust
#[derive(Debug)]
struct ParsePaymentInfoError {
    source: Option<Box<dyn Error>>,
    msg: Option<String>
}

impl Display for ParsePaymentInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Parsing payment error: invalid payment info")
    }
}
```

It implements `Display` for user facing error messages and `Debug` for developer facing error messages. Next let's update the `parse_card`
function to return `ParsePaymentInfoError` in the error case.

```rust
fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    let mut numbers = parse_card_numbers(card)?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(ParsePaymentInfoError{
            source: None,
            msg: Some(format!("Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}"))
        })
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Ok(Card {
        number,
        exp: Expiration { year, month },
        cvv
    })
}
```

Now we get the missing context in the error. But it's still very hard to read. Let's fix that next by customizing the implementation of the
`Debug` trait.

# Custom Error 3
In the list lesson we implemented the `Error` trait for `ParsePaymentInfoError`. Now let's also implement the `Error` trait for 
`CreditCardError`.

```rust
#[derive(Debug)]
enum CreditCardError {
    InvalidInput(String),
    Other(Box<dyn Error>, String)
}

impl Display for CreditCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Credit card error: Could not retrieve credit card.")
    }
}

impl Error for CreditCardError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CreditCardError::InvalidInput(_) => None,
            CreditCardError::Other(e, _) => {
                Some(e.as_ref())
            }
        }
    }
}
```
Now let's customize the implementation of the `Debug` trait. Instead of deriving `Debug` we will implement it ourselves.

```rust
enum CreditCardError {
    InvalidInput(String),
    Other(Box<dyn Error>, String)
}

impl std::fmt::Debug for CreditCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "{self}\n{msg}"),
            Self::Other(e, msg) => write!(f, "{self}\n{msg}\n\nCaused by:\n\t{e:?}")
        }
    }
}
```

In case of `Other` variant, we will print out self which is the display string, the `msg` stored in other variant and the underlying cause `e`.
For the underlying cause we will printout the error stored inside the `Other` variant with debug formatting `{e:?}`.

Next let's add a custom implementation of the `Debug` trait for `ParsePaymentInfoError`.

```rust
struct ParsePaymentInfoError {
    source: Option<Box<dyn Error>>,
    msg: String
}

impl std::fmt::Debug for ParsePaymentInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self} \n\t{}", self.msg)?;

        if let Some(e) = self.source.as_ref() {
            write!(f, "\n\nCaused by:\n\t{e:?}")?;
        }

        Ok(())
    }
}
```

Now we get detailed error messages that are nicely formatted.

# thiserror and anyhow
In the last lessons, we customized the `Debug` trait for custom error types which finally got us nicely formatted error messages.

Now we will see how to reduce amount of code we have to write by using third part crates. Specifically `thiserror` crate which provides
a `derive` macro for the `Error` trait. By extension the macro will also implement the `Debug` and `Display` traits and `from` traits.

We will also be using the `anyhow` trait which provides a trait object based error type. Unlike the boxed trait object errors we have 
been using `anyhow` allows us to attach context to errors. Let's import these crates:

```toml
...
[dependencies]
log = "0.4"
env_logger = "0.9"
thiserror = "1.0"
anyhow = "1.0"
```

Next let's derive the `Debug` and `Error` trait for `ParsePaymentInfoError` and get rid of the custom implementations of `Debug`, `Error`
and `Display` traits.

```rust
#[derive(Debug, Error)]
#[error("{msg}")]
struct ParsePaymentInfoError {
    source: Option<Box<dyn Error>>,
    msg: String
}
```

To implment the `Display` trait automatically we can use attribute like macro called `error` from `thiserror`. Inside the quotes we write
the message we want printed for `Display` trait. In this case, we want `msg` field to be printed.

Next let's update the `source` field. Instead of using Boxed error trait object, we will replace it with `anyhow` error.

```rust
#[derive(Debug, Error)]
#[error("{msg}")]
struct ParsePaymentInfoError {
    source: Option<anyhow::Error>,
    msg: String
}
```

Then we will update `parse_card_numbers`. Instead of mapping `ParseIntError` to `ParsePaymentInfoError` we are going to call `with_context`.
`with_context` will convert an error into `anyhow` error and add additional context. In this case a custom error message.

```rust
fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card.split(" ")
        .into_iter()
        .map(|s|{
            s.parse().with_context(|| format!("{s:?} could not be parsed as u32"))
        })
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|e| ParsePaymentInfoError{
            source: Some(e),
            msg: Some(format!("Failed to parse input as numbers. Input: {card}"))
        })?;

    Ok(numbers)
}
```

Next let's update `CreditCardError`.

```rust
#[derive(Error, Debug)]
enum CreditCardError {
    #[error("{0}")]
    InvalidInput(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
```

For `InvalidInput` the `Display` trait will simply print the string stored inside. For the `Other` variant the transparent keyword and
from attribute indicate that this error is going to be the source error and that the Display implementation should be forwared to the 
source error. Next let's update `get_credit_card_info` function.

```rust
fn get_credit_card_info(credit_cards: &HashMap<&str, &str>, name: &str) -> Result<Card, CreditCardError> {
    let card_string = credit_cards.get(name).ok_or(
        CreditCardError::InvalidInput(format!("No credit card was found for {name}."))
    )?;
    let card = parse_card(card_string)
        .with_context(|| format!("{name}'s card couldn't be parsed."))
        .map_err(|e| {
            CreditCardError::Other(e)
        })?;

    Ok(card)
}
```

Next let's update the main function.

```rust
fn main() {
    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27 123"),
        ("Bob", "1234567 12 27 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());

    match result {
        Ok(card) => println!("\nCredit Card Info: {card:?}"),
        Err(err) => {
            match err {
                CreditCardError::InvalidInput(msg) => println!("{msg}"),
                CreditCardError::Other(_) => println!("\nSomething went wrong! Try again!")
            }

            log::error!("\n{err:?}");   // error
        },
    }
}
```

# Bonus: error-stack
In some languages we throw and catch exceptions, in Rust if an operation could fail, returns a `Result` enum. A `Result` enum could either
be an `Ok` variant or an `Err` variant. Now when you consider creating custom error types, propagating errors, reporting errors both to the
user and to the developers with extra debug information, abstraction layers and best practices things, get really complicated. Complicated
enough for Rust to have an error handling project group dedeicated to improving error handling in Rust.This group have been working on 
improving the `Error` trait in the standard library. Besides that third party libraries like `thiserror` and `anyhow` havw been created
to make error handling easier in Rust.

Despite these crates error handling in Rust can still be complicated, especially to new-comers. That's why developers at [hash](https://hash.dev/), 
a startup focussed on building tools to improve decision making have decided to create their own error handling library called `error_stack`.

In a blog post, announcing error_stack developers at hash explain what the error handling approach used to be and why they created `error_stack`.
They used to define custom error types as large top level enums at the module or crate level with many different variants.

```rust
#[derive(ThisError)]
enum Error {
    #[error("Simulation error: {0}")]
    Simulation(#[from] crate::simulation::Error)
    ...
}
```

When crossing boundaries they would transform from one error to another using `from` trait. They also took advantage of `ThisError` crate 
to help reduce boiler plate. However, this approach led to error messages that weren't very helpful and had very little context.

Defining custom error types with enums with many variants also introduced overhead that hurt them in periods of rapid development.
Because of these pain points and others they decided to create an error handling library. Error handling library like `thiserror` 
and `anyhow` already exist, so what makes `error_stack` different?

The developers at hash wanted to create an error handling library with a few key features:
- Wanted to encourage users to provide a new error type when crossing module/crate boundaries or when changing levels of abstractions. E.g.
returning a `ConfigParseError` when parsing a configuration file vs. returning an `IoError` when reading a file from disk.
- They wanted this to be possible without needing many implementations of the `From` trait.
- They wanted ability to attach any data to an error without a lot of configuration.

With this features in mind they created error_stack. It is centered around taking a base error and building a full richer picture of it as
it propagates. We call this a `Report`. 

A `Report` is organized as a stack of `Frames` where each `Frame` contains either a `Context` or an `attachment`.

`Context` is a high level description of the error. This is going to be a type that implements the `Error` trait. `Attachments` are extra 
information you can add. They could be `String` like types or arbitrary data types. The caveat is arbitrary types are added using the provider
API, which requires a nightly version of the Rust compiler.

The `Report` is able to iterate through the `Frame` stack from the most recent one to the root cause. Producing easy to read error msg.

Now let's take an example and add error handling using `error_stack`.

In the following example, we have a cli program which takes a name as input and returns credit card information from some sort of persisted
storage. For simplicity in this case we are using hashmap. In this hashmap keys are names and values are credit card informations stored as
string of 4 numbers credit card number, expiration month, expiration year & cvv.

```rust
fn main() {
    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27 123"),
        ("Bob", "1234567 12 27 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());

    println!("\nCredit Card Info: {result:?}");
}

fn get_credit_card_info(credit_cards: &HashMap<&str, &str>, name: &str) -> Card {
    let card_string = credit_cards.get(name).unwrap();
    let card = parse_card(card_string);

    card
}

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiraiton,
    cvv: u32,
}

fn parse_card(card: &str) -> Card {
    let mut numbers = parse_card_numbers(card);

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Card {
        number,
        exp: Expiration { year, month },
        cvv
    }
}

fn parse_card_numbers(card: &str) -> Vec<u32> {
    let numbers = card.split(" ")
        .into_iter()
        .map(|s|{
            s.parse()
        })
        .collect::<Result<Vec<u32>, _>>()
        .unwrap();

    numbers
}
```

Here we are not doing any error handling, instead we are calling `unwrap` everywhere, which means our program will panic if we get the
error variant.

Lets add logging and error handling in this program. Lets first open `Cargo.toml` and add some dependencies.

```toml
...
[dependencies]
log = "0.4.17"
env_logger = "0.9.0"
error-stack = "0.1.1"
```

Let's add the required imports:

```rust
use std::{collections::HashMap, io, fmt, error::Error};
use error_stack::{IntoReport, Report, Result, ResultExt}
```

`IntoReport` is a trait which allows us to convert `Err` variant into a `Report` struct. `Report` is the type we are using for error
handling. `Result` is a type alias for the `Result` enum in the standard library, where the `Err` variant is a `Report`. And `ResultExt`
is a trait for `Result` which will allow us to provide context infromation on `Report`.

Next let's add error handling to the `parse_card_numbers` function. First let's create a custom error type for parsing errors.

```rust
#[derive(Debug)]
struct ParsePaymentInfoError;

impl fmt::Display for ParsePaymentInfoError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Parsing payment error: invalid payment info")
    }
}

impl Error for ParsePaymentInfoError {}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card.split(" ")
        .into_iter()
        .map(|s|{
            s.parse()
                .report()
                .attach_printable_lazy(|| {
                    format!("{s:?} could not be parsed as u32")
                })
        })
        .collect::<Result<Vec<u32>, _>>()?; // error

    Ok(numbers)
}
```

We call the `attach_printable_lazy` to lazily add a new printable `attachment` which will print out the specific substring that couldn't
be parsed as u32 integer. Remove `unwrap` and add `?` to propagate any error. We get an compile time error because function signature
expects `ParsePaymentInfoError` but be are returing `ParseIntError`. Without `error_stack` we might do that by implementing `from` trait
but with `error_stack` all we have to do is, change the context. To do that after `collect`, simply call `change_context` and pass in 
`ParsePaymentInfoError`. 

```rust
fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card.split(" ")
        .into_iter()
        .map(|s|{
            s.parse()
                .report()
                .attach_printable_lazy(|| {
                    format!("{s:?} could not be parsed as u32")
                })
        })
        .collect::<Result<Vec<u32>, _>>()
        .change_context(ParsePaymentInfoError)
        .attach_printable(format!(
            "Failed to parse input as numbers. Input: {card}"
        ))?;

    Ok(numbers)
}
```

Nice thing is that, `error_stack` uses the type system to force to change context when we are switching from a lower level abstraction, parsing
individual substrings to a higher level abstraction, returning a result for this function call. 

The last thing we will do is add another printable attachment after changing context. As you can see, little by little we are building up
the report before propagating it.

Now let's update `parse_card` function. In this case we didn't have to change context because `parse_card` and `parse_card_numbers` have the
same context i.e. `ParsePaymentInfoError`. This makes sense because both functions are at the similar level of abstraction.

```rust
fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    let mut numbers = parse_card_numbers(card)?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(Report::new(ParsePaymentInfoError).attach_printable(format!(
            "Incorrect number of elements parsed. Expected {expected_len} but go {len}. Elements: {numbers:?}"
        )))
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Ok(Card {
        number,
        exp: Expiration { year, month },
        cvv
    })
}
```

Now let's update `get_credit_card_info` function. Just like with other function we want to return a `Result` type but what should the `Err`
variant be? It doesn't make sense to use the `ParsePaymentInfoError` because this function is at higher level of abstraction. So instead lets
create another custom error type. If you notice previous custom error was empty struct but this time we are using an enum with two variants.
Why is that?
With the parsing error we simply wanted to attach some extra information and propagate the error. With this error however, we want to 
programatically handle two different cases:
- If the user gave invalid input. e.g. they gave use a name that didn't exist in our hashmap. In that case we want to tell user that their
input is invalid.
- If we get error unrelated to the user's input in that situation we simply want to say "something went wrong. try again"

```rust
#[derive(Debug)]
enum CreditCardError {
    InvalidInput(String),
    Other
}

impl fmt::Display for CreditCardError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Credit card error: Could not retrieve credit card.")
    }
}

impl Error from CreditCardError{}
    
fn get_credit_card_info(credit_cards: &HashMap<&str, &str>, name: &str) -> Result<Card, CreditCardError> {
    let card_string = credit_cards.get(name).ok_or_else(|| {
        let msg = format!("No credit card was found for {name}.");
        Report::new(CreditCardError::InvalidInput(msg.clone()))
            .attach_printable(msg.clone());
    })?;

    let card = parse_card(card_string)
                .change_context(CreditCardError::Other)
                .attach_printable(format!(
                    "{name}'s card could not be parsed."
                ))?;

    Ok(card)
}
```

Calling a `get` method on a hashmap return an `Option`, we then call `ok_or_else` to map that optional to a `Result` type.
If the optional is `Some` variant then it gets turned into `Ok` variant and if it's a `None` variant then the closure above get called.

Inside this closure we create `msg` string saying "No credit card was found for {name}", we then return a new `Report` with context set
to `InvalidInput` variant. `InvalidInput` contains the `msg` and we also attach the message to the `Report`. If `parse_card` returns an
error, we want to propagate that error to do that add `?` at the end of the call. We also need to change context because `parse_card` 
returns `ParsePaymentInfoError` as the `Err` variant and our function returns `CreditCardError`. 

Last thing we do is add a printable attachment after we change the context. 

Now last function we need to update is `main`.

```rust
fn main() {
    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27 123"),
        ("Bob", "1234567 12 27 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());

    match result {
        Ok(card) => {
            println!("\nCredit Card Info: {card:?}");
        },
        Err(err) => {
            match err.current_context() {
                CreditCardError::InvalidInput(msg) => println!("\n{msg}"),
                CreditCardError::Other => println!()
            }
        }
    }
}
```
