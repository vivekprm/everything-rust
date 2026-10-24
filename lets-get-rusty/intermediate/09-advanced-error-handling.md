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
