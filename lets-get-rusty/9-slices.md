# Slices
Slices are references to a contiguous sequence of elements in a collection.

Slices are typically used when you want to reference part of a collection, instead of the entire collection.
Let's look at an example:

```rust
fn main() {
    let tweet = String::from(
        "This is my tweet and it's very very long"
    );
    let trimmed_tweet: &str = &tweet[..20]; // string slice
    println!("{trimmed_tweet}");
}
```

String slice is a type represented as &str.

Let's look at both string types `String` and `&str` closely:
- **String**
    - growable, heap allocated string (UTF-8 encoded).
- **str**
    - immutable sequence of UTF-8 bytes somewhere in memory (stack, heap, or static memory).
    - Handle behind a reference (&str) because length of sequence is unknown at compile time.

To better understand the difference between String & str, let's look at the relationship between them:

pic

Bootcamp string is owned by a variable of type String, which contains:
- The pointer to the first index of the string.
- Length of the string 
- Capacity of the string
    - In this case cpacity is 10 bytes, however we are only taking up 8 bytes.

We have also created a string slice which has:
- Pointer to an index in our string 
- Length of the string slice.

So our string slice has immutable view of our string "camp". Which is as substring of string "bootcamp".

**When you need to own the string because you want to mutate it or pass it to other threads then use the String
type. But if you only need immutable view of a string or a subset of a string then use string slice &str**.

If you create a string literal as below, type is going to be a string slice:

```rust
let s = "my string";
```

In Rust, all string literals are string slices and strings themselves are stored in your applications binary.
So in this case 's' is as string slice pointing to a specific location in your program's binary.

# String Slices & Functions
```rust
fn main() {
    let tweet = String::from(
        "This is my tweet and it's very very long"
    );

    let trimmed_tweet = trim_tweet(&tweet);
    println!("{trimmed_tweet}");
}

fn trim_tweet(tweet: &String) -> &str {
    &tweet[..20]
}
```

What would happen if our string was string literal?


```rust
fn main() {
    let tweet = String::from(
        "This is my tweet and it's very very long"
    );

    let trimmed_tweet = trim_tweet(&tweet);
    println!("{trimmed_tweet}");

    let tweet2 = "This is my second tweet and it's also very long.";
    let trim_tweet2 = trim_tweet(tweet2); // error
}

fn trim_tweet(tweet: &String) -> &str {
    &tweet[..20]
}
```

We get an error while calling trim_tweet saying "type mismatch as trim_tweet expects &String but we are 
passing &str. We don't want to create another function to handle string slices. Luckily we don't have to,
just change the type of the time_tweet to `&str`.

```rust
fn trim_tweet(tweet: &str) -> &str {
    &tweet[..20]
}
```

Now our function works both for `&String` and `&str`.

You might we wondering why we are not getting type mismatch for earlier call, as we are passing &String but
our function expects &str?
This works in Rust because of a feature called **Deref Coercion**, due to which Rust automatically coerce &String
to a &str.

Important thing to not is that, **If you have a function that accepts a string and you don't need onwership
of that string then you should use string slices (&str). That way callers of your function could pass references
to String or str.

Slices also work with other collections, such as arrays and vectors.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5, 6];
    let a_slice = &a[..3];
    println!("{:?}", a_slice);
}
```

This syntax of println with ':?' will print a string out with debug formatting.
