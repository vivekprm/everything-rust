To understand strings first we are going to talk about binary. A computer's main memory consists of a bunch of
transistors that could either have a high voltage level or a low voltage level, 1 or 0. So at the end of the 
day computers can only understand 1s and 0s. That's not very useful for us but luckily we can transform these
ones and zeros into numbers using binary number system.

E.g. here we can take a byte and represent number 65:

    0   1   0   0   0   0   0   1 = 65

Let's say what we really want is, we want to represent some text. So how can we get from integers to text.
What we encoded these integers with meaning? What if we said every integer maps to specific character?

Then we can string these characters together to form words and sentences. This brings us to ASCII.

# ASCII
        72      101     108     108     111     33
        H       e       l       l       o       !

1 character = 7 bits

ASCII stands for **American Standard Code for Information Interchange**. This is an encoding which maps integer
to characters.

First version of ASCII represented each characters with 7 bits. Which means we had only a total of 128 characters.
Now that's very limited number. If you go to ASCII wiki page you can see all the values we could represent.

We have limited number of characters but it's enough to encode english alphabets, integers and special symbols
but what about all the other languages?

Once world wide web was invented it allowed people all over the world to communicate with each other but this
created a problem, as everyone was using different text encodings, how we were suppose to share information?

To solve this problem a bunch of poeople got together and formed a group called **Unicode Consortium** and the
way they solved this problem is by creating a standard that encompassed all the different characters out there.

One standard that was widely used called **UTF-8**. UTF stands for Unicode Transformation Format and 8 is for
8 bits. 

UTF-8 is a variable width character encoding. With ASCII you can represnt a character in one byte. However, in 
UTF-8 a character can be in range on 1 byte to 4 byte. This means that UTF-8 standard can encode 1,112,064 chars.

What's even better is that UTF-8 is backward compatible with ASCII. 

# Let's look at how UTF-8 encodes text
The first byte tells us how many bytes a character takes up. E.g. 

If a character takes up one byte then that byte will start with 0. 

0xxxxxxx

If a character takes 2 bytes then first byte will start with 110.

110xxxxx    10xxxxxx

If a character take 3 bytes then the first byte will start with 1110.

1110xxxx 10xxxxxx 10xxxxxx

If a character takes 4 bytes then first byte will start with 11110

11110xxx 10xxxxxx 10xxxxxx 10xxxxxx

Remember that ASCII characters are 7 bits, so we can store all our ASCII characters in one byte.

For characters that takeup multiple bytes, first byte will tell how many bytes that character takes up and then
the preceeding bytes will start with 10.

This format makes it easy to tell weather we are at the starting byte of a character or if we were in the middle
of a character.

Now because UTF-8 can represent over a million characters, not only it encompasses all the languages in the 
world but it also encodes things like emojis. e.g. crab emoji encoded in utf-8 is:

11110000    10011111    10100110    10000000

So this character takes up 4 bytes because it starts with 11110.

UTF-8 is the encoding used in Rust.

Now let's talk about Rust specific concepts.

There are two different types of strings in Rust:

**str**
pic

str is a view into a sequence of utf-8 encoded bytes of dynamic length. These bytes could either be stored in
the application's binary on the stack or on the heap. The amount of byte is dynamic meaning we can't know the
amount of byte at compile time, you might say that if the bytes are in binary then we would know at compile time
but if bytes are for example on the heap then those bytes can be allocated at runtime thus we don't know the
size at compile time.

Because of this we don't use `str` type directly we use the borrowed form which is `&str`. This is commonly
referred to as string slice. Now Rust compiler needs to know the size of the types that we use at compile
time. We can't know the size of `str` because it points to a dynamic length of bytes but we do know the size
of the string slice `&str` because it just stores an address pointer pointing to first byte of the string and
the length of the string.

To summarize **string slice is view in the sequence of utf-8 encoded bytes**. A string slice doesn't own the
underlying data. So if you just need an immutable view of a string or part of a string then use a string slice.

**String**
pic

String type is on the other hand provided by Rust standard library which is a growable, mutable, owned utf-8
encoded string. **With the String type underlying string is always going to be allocated on the heap**.

Unlike a string slice we are going to keep track of three values:
- An address pointing to the first byte of the string.
- Length of the string
- Capacity of the string

String has more overhead than &str. However, the advantage is we own the data so we can manipulate the string
however we like.

Now let's jump into the code and see some code examples:

```rust
fn main() {
    let s1 = "привіт світ! 🦀";

    let s2 = String::from("привіт світ!");
    let s3 = "привіт світ!".to_string();
    let s4 = "привіт світ!".to_owned();

    let s5 = &s4[..];

    println!("{}", s5);
}
```

In Rust, string literals are string slices that are stored in the application's binary.

# String manipulation
Now let's talk about manipulating string:

```rust
fn main() {
    let mut s = String::from("foo");
    s.push_str(" bar");
    println!("{}", s);

    s.replace_range(.., "baz");
    println!("{}", s);
}
```

# Concatenation
Let's concatenate strings:

```rust
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;

    println!("{}", s1); // error
}
```

This will move s1 into s3 and copy the contents of s2 and append it to s3. Now because s1 has been moved, if 
we try to print out s1 we will get an error "You can't borrow s1 as it has already moved."

Another way to concatenate string is using `format` macro.

```rust
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}
```

format macro is going to be less efficient than the plus opertor we saw above because it's going to copy contents
of each of these strings. 

Also know that format macro could take String type as well as string slices.

Below are some of the other ways to concatenate strings:

```rust
fn main() {
    let s1 = ["first", "second"].concat();
    let s2 = format!("{}{}", "first", "second");
    let s3 = concat!("first", "second");

    let s4 = String::from("test");
    let s5 = s4 + "okok";   // String type must be first.
} 
```

# Indexing into a String
```rust
fn main() {
    let s1 = "🦀🦀🦀🦀🦀";
    let s2 = s1[0]; // error
}
```

We get an error saying, "we can't intex into a string using integer". Why this error?
String is just a collection of bytes so s1[0] is going to give us first byte of our string. But as we mentioned
before in UTF-8, a character could be beween 1-4 bytes. crab is 4 byte long. You might expect s1[0] wil give 
first crab but s[0] just gives first byte, which doesn't mean anything. 

To prevent errors, where people might index into a string and don't get the expected output, Rust doesn't let us
index into string using an integer. Rust however, does allow us to create a string slice over a specific set of
bytes. 

```rust
fn main() {
    let s1 = "🦀🦀🦀🦀🦀";
    let s2 = &s1[0..4];
    println!("{}", s2);
}
```

Prints the first crab. But here we need to be very careful because we have to know exactly how many bytes a 
character is. So if instead we give &s1[0..3] we get thread panic saying "byte index 3 is not a char boundary".

Rust make sure that a string or string slice is valid utf-8 here &s1[0..3] is not a valida utf-8. So main thread
panics.

Because in utf-8 each character can be different amount of bytes, if you want to find a particular character 
within a string it's not going to take constant time, it's actually going to take linear time and that's 
because you have to iterate through each character to find the one you are looking for.

# Iterating Over String
There are few ways to iterate over a string.

## Iterating Over the Bytes of a String

```rust
fn main() {
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
```

## Iterating Over the Characters of a String

```rust
fn main() {
    for c in "नमस्ते".bchars() {
        println!("{}", c);
    }
}
```

This is little confusing if you look at the characters being printed out because you might have expected chars
to be an iterator over the user perceived characters of the string but in fact it's an iterator over the 
unicode scalar value of the string. scalar values are a basic unit in unicode and some user percieved characters
are madeup of multiple scalar values.

In unicode user perceived characters are known as grapheme clusters and in order to iterate over grapheme clusters
we actually have to import a crate **UnicodeSegmentation**.

```rust
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    for g in "नमस्ते"".graphemes(true) {
        println!("{}", g);
    }
}
```

With this we get user perceived characters.

# Strings & Functions
You would frequently see a function that takes a string slice and returns a String. Advantage of taking
string slice is that we can pass string slices as well as String to our function.

```rust
fn main() {
    let s1 = "Hello World!";
    let s2 = String::from("Hello World!");
    my_function(s1);
    my_function(&s2);
}

fn my_function(a: &str) -> String {
    return format!("{}", a);
}
```

Since each character in string can be from 1 to 4 bytes it's linear time operation to search for a character.
What if all the characters were 4 bytes then we could search a character in constant time. Something like this
is not implemented in Rust but Rune type in Go does exactly that.
