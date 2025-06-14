Enums allow you to define a type by enumerating its possible values. 

# Defining an Enum
Let’s look at a situation we might want to express in code and see why enums are useful and more appropriate than structs in this case. Say we need to work with IP addresses. Currently, two major standards are used for IP addresses: version four and version six. These 
are the only possibilities for an IP address that our program will come across: we can enumerate all possible values, which is where 
enumeration gets its name.

Any IP address can be either a version four or a version six address, but not both at the same time. That property of IP addresses 
makes the enum data structure appropriate, because enum values can only be one of the variants. Both version four and version six 
addresses are still fundamentally IP addresses, so they should be treated as the same type when the code is handling situations that 
apply to any kind of IP address.

We can express this concept in code by defining an IpAddrKind enumeration and listing the possible kinds an IP address can be, V4 
and V6. These are known as the variants of the enum:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

IpAddrKind is now a custom data type that we can use elsewhere in our code.

# Enum Values
We can create instances of each of the two variants of IpAddrKind like this:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two. The reason 
this is useful is that now both values ```IpAddrKind::V4``` and ```IpAddrKind::V6``` are of the same type: ```IpAddrKind```. We can 
then, for instance, define a function that takes any ```IpAddrKind```:

```rust
fn route(ip_type: IpAddrKind) { }
```

And we can call this function with either variant:

```rust
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

Using enums has even more advantages. Thinking more about our IP address type, at the moment we don’t have a way to store the actual 
IP address data; we only know what kind it is. Given that we just learned about structs, we might tackle this problem as shown below.

```rust
enum IpAddrKind {   // 1
    V4,
    V6,
}

struct IpAddr { // 2
    kind: IpAddrKind,   // 3
    address: String,    // 4
}

let home = IpAddr {     // 5
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr { // 6
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

Here, we’ve defined a struct IpAddr (2) that has two fields: a kind field (3) that is of type IpAddrKind (the enum we defined 
previously (1)) and an address field (4) of type String. We have two instances of this struct. The first, home (5), has the value 
```IpAddrKind::V4``` as its kind with associated address data of 127.0.0.1. The second instance, loopback (6), has the other variant 
of IpAddrKind as its kind value, V6, and has address ::1 associated with it. We’ve used a struct to bundle the kind and address values 
together, so now the variant is associated with the value.

We can represent the same concept in a more concise way using just an enum, rather than an enum inside a struct, by putting data 
directly into each enum variant. This new definition of the IpAddr enum says that both V4 and V6 variants will have associated String 
values:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

We attach data to each variant of the enum directly, so there is no need for an extra struct.

There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data. 
Version four type IP addresses will always have four numeric components that will have values between 0 and 255. If we wanted to store
V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t be able to with a struct. Enums handle 
this case with ease.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

We’ve shown several different ways to define data structures to store version four and version six IP addresses. However, as it 
turns out, wanting to store IP addresses and encode which kind they are is so common that the standard library has a definition 
we can use! 

Let’s look at how the standard library defines IpAddr: it has the exact enum and variants that we’ve defined and used, but it embeds 
the address data inside the variants in the form of two different structs, which are defined differently for each variant:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

This code illustrates that you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example. *You 
can even include another enum*! Also, standard library types are often not much more complicated than what you might come up with.

Note that even though the standard library contains a definition for IpAddr, we can still create and use our own definition without 
conflict because we haven’t brought the standard library’s definition into our scope. We’ll talk more about bringing types into scope
later.
Let’s look at another example of an enum in below code: this one has a wide variety of types embedded in its variants.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

This enum has four variants with different types:
- ```Quit``` has no data associated with it at all.
- ```Move``` includes an anonymous struct inside it.
- ```Write``` includes a single String.
- ```ChangeColor``` includes three i32 values.

Defining an enum with variants such as the ones in above is similar to defining different kinds of struct definitions, except the 
enum doesn’t use the struct keyword and all the variants are grouped together under the ```Message``` type. The following structs 
could hold the same data that the preceding enum variants hold:

```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

But if we used the different structs, which each have their own type, we couldn’t as easily define a function to take any of these 
kinds of messages as we could with the Message enum defined earlier, which is a single type.

There is one more similarity between enums and structs: just as we’re able to define methods on structs using impl, we’re also able 
to define methods on enums. Here’s a method named call that we could define on our Message enum:

```rust
impl Message {
    fn call(&self) {
        // (1) method body would be defined here
    }
}
let m = Message::Write(String::from("hello"));  // (2)
m.call();
```

The body of the method would use self to get the value that we called the method on. In this example, we’ve created a variable m (2) 
that has the value ```Message::Write(String::from("hello"))```, and that is what self will be in the body of the call method (2) when 
```m.call()``` runs.

Let’s look at another enum in the standard library that is very common and useful: ```Option```.

## The Option Enum and Its Advantages over Null Values 
The ```Option``` type is used in many places because it encodes the very common scenario in which a value could be something or it 
could be nothing. Expressing this concept in terms of the type system means the compiler can check whether you’ve handled all the 
cases you should be handling; this functionality can prevent bugs that are extremely common in other programming languages.

Programming language design is often thought of in terms of which features you include, but the features you exclude are important 
too. *Rust doesn’t have the null feature that many other languages have*. Null is a value that means there is no value there. In 
languages with null, variables can always be in one of two states: null or not-null.

In his 2009 presentation “Null References: The Billion Dollar Mistake,” Tony Hoare, the inventor of null, has this to say:

*"I call it my billion dollar mistake. At that time, I was designing the first comprehensive type system for references in an 
object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed
automatically by the compiler. But I couldn’t resist the temptation to put in a null reference, simply because it was so easy to 
implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of
pain and damage in the last forty years."*

The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind. Because 
this null or not-null property is pervasive, it’s extremely easy to make this kind of error.

However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for 
some reason.

The problem isn’t really with the concept but with the particular implementation. As such, Rust does not have nulls, but it does 
have an enum that can encode the concept of a value being present or absent. This enum is ```Option<T>```, and it is defined by the 
standard library as follows:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

The ```Option<T>``` enum is so useful that *it’s even included in the prelude*; you don’t need to bring it into scope explicitly. 
In addition, so are its variants: you can use ```Some``` and ```None``` directly without the ```Option::``` prefix. The ```Option<T>```
enum is still just a regular enum, and ```Some(T)``` and ```None``` are still variants of type ```Option<T>```.

The <T> syntax is a feature of Rust we haven’t talked about yet. It’s a generic type parameter, and we’ll cover generics in more 
detail later.

For now, all you need to know is that <T> means the Some variant of the Option enum can hold one piece of data of any type. Here 
are some examples of using Option values to hold number types and string types:

```rust
let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;
```

If we use None rather than Some, we need to tell Rust what type of ```Option<T>``` we have, because the compiler can’t infer the 
type that the Some variant will hold by looking only at a ```None``` value.

When we have a ```Some``` value, we know that a value is present and the value is held within the Some. When we have a ```None``` 
value, in some sense, it means the same thing as null: we don’t have a valid value. So why is having Option<T> any better than 
having null?

In short, because ```Option<T>``` and T (where T can be any type) are different types, the compiler won’t let us use an ```Option<T>```
value as if it were definitely a valid value. For example, this code won’t compile because it’s trying to add an i8 to an 
```Option<i8>```:

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y;
```

If we run this code, we get an error message like this:
```
error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
not satisfied
-->
|
5 | |
let sum = x + y;
| ^ no implementation for `i8 + std::option::Option<i8>`
```

Intense! In effect, this error message means that Rust doesn’t understand how to add an ```i8``` and an ```Option<i8>```, because 
they’re different types.

When we have a value of a type like i8 in Rust, the compiler will ensure that we always have a valid value. We can proceed confidently
without having to check for null before using that value. Only when we have an ```Option<i8>``` (or whatever type of value we’re 
working with) do we have to worry about possibly not having a value, and the compiler will make sure we handle that case before using 
the value.

In other words, you have to convert an ```Option<T>``` to a T before you can perform T operations with it. Generally, this helps 
catch one of the most common issues with null: assuming that something isn’t null when it actually is.

Not having to worry about incorrectly assuming a not-null value helps you to be more confident in your code. In order to have a value 
that can possibly be null, you must explicitly opt in by making the type of that value ```Option<T>```. Then, when you use that 
value, you are required to explicitly handle the case when the value is null. 

Everywhere that a value has a type that isn’t an ```Option<T>```, you can safely assume that the value isn’t null. This was a 
deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.

So, how do you get the T value out of a Some variant when you have a value of type ```Option<T>``` so you can use that value? 
The ```Option<T>``` enum has a large number of methods that are useful in a variety of situations; you can check them out in its 
documentation. Becoming familiar with the methods on ```Option<T>``` will be extremely useful in your journey with Rust.

In general, in order to use an ```Option<T>``` value, you want to have code that will handle each variant. You want some code that 
will run only when you have a ```Some(T)``` value, and this code is allowed to use the inner T. You want some other code to run if 
you have a None value, and that code doesn’t have a T value available. The match expression is a control flow construct that does
just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the 
data inside the matching value.

## The match Control Flow Operator
Rust has an extremely powerful control flow operator called ```match``` that allows you to compare a value against a series of 
patterns and then execute code based on which pattern matches. Patterns can be made up of *literal values*, *variable names*, 
*wildcards*, and many other things; later we'll cover all the different kinds of patterns and what they do. The power of match comes 
from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.

Think of a match expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it, and 
each coin falls through the first hole it encounters that it fits into. In the same way, values go through each pattern in a match, 
and at the first pattern the value “fits,” the value falls into the associated code block to be used during execution.

Because we just mentioned coins, let’s use them as an example of using match! We can write a function that can take an unknown 
United States coin and, in a similar way as the counting machine, determine which coin it is and return its value in cents, as shown 
here in below code.

```rust
enum Coin { // 1
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {    // 2
        Coin::Penny => 1,   // 3
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Let’s break down the match in the value_in_cents function. First, we list the match keyword followed by an expression, which in this 
case is the value coin (2). This seems very similar to an expression used with if, but there’s a big difference: with if, the 
expression needs to return a Boolean value, but here, it can be any type. The type of coin in this example is the Coin enum that we
defined at (1).

Next are the match arms. An arm has two parts: a pattern and some code. The first arm here has a pattern that is the value 
```Coin::Penny``` and then the ```=>``` operator that separates the pattern and the code to run (3). The code in this case is just the 
value 1. Each arm is separated from the next with a comma.

When the match expression executes, it compares the resulting value against the pattern of each arm, in order. If a pattern matches 
the value, the code associated with that pattern is executed. If that pattern doesn’t match the value, execution continues to the 
next arm, much as in a coin-sorting machine. We can have as many arms as we need: in below code, our match has four arms.

The code associated with each arm is an expression, and the resulting value of the expression in the matching arm is the value that 
gets returned for the entire match expression.

Curly brackets typically aren’t used if the match arm code is short, as it is in below code where each arm just returns a value. If 
you want to run multiple lines of code in a match arm, you can use curly brackets. For example, the following code would print 
“Lucky penny!” every time the method was called with a ```Coin::Penny``` but would still return the last value of the block, 1:

```rust
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## Patterns That Bind to Values
Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can 
extract values out of enum variants.

As an example, let’s change one of our enum variants to hold data inside it. From 1999 through 2008, the United States minted 
quarters with different designs for each of the 50 states on one side. No other coins got state designs, so only quarters have 
this extra value. We can add this information to our enum by changing the Quarter variant to include a UsState value stored inside 
it, which we’ve done here in below code.

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

Let’s imagine that a friend of ours is trying to collect all 50 state quarters. While we sort our loose change by coin type, we’ll 
also call out the name of the state associated with each quarter so if it’s one our friend doesn’t have, they can add it to their 
collection.

In the match expression for this code, we add a variable called state to the pattern that matches values of the variant 
```Coin::Quarter```. When a ```Coin::Quarter``` matches, the state variable will bind to the value of that quarter’s state. Then 
we can use state in the code for that arm, like so:

```rust
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

If we were to call ```value_in_cents(Coin::Quarter(UsState::Alaska))```, coin would be ```Coin::Quarter(UsState::Alaska)```. When we 
compare that value with each of the match arms, none of them match until we reach ```Coin::Quarter(state)```. At that point, the 
binding for state will be the value ```UsState::Alaska```. We can then use that binding in the ```println!``` expression, thus 
getting the inner state value out of the Coin enum variant for Quarter.

## Matching with Option<T>
In the previous section, we wanted to get the inner T value out of the Some case when using ```Option<T>```; we can also handle 
```Option<T>``` using match as we did with the ```Coin``` enum! Instead of comparing coins, we’ll compare the variants of 
```Option<T>```, but the way that the match expression works remains the same.

Let’s say we want to write a function that takes an ```Option<i32>``` and, if there’s a value inside, adds 1 to that value. If there 
isn’t a value inside, the function should return the None value and not attempt to perform any operations.

This function is very easy to write, thanks to match, and will look like below code.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,   // 1
        Some(i) => Some(i + 1), // 2
    }
}
let five = Some(5);
let six = plus_one(five);   // 3
let none = plus_one(None);  // 4
```

Let’s examine the first execution of plus_one in more detail. When we call ```plus_one(five)``` (3), the variable x in the body of 
```plus_one``` will have the value ```Some(5)```. We then compare that against each match arm.

The ```Some(5)``` value doesn’t match the pattern None (1), so we continue to the next arm. Does ```Some(5)``` match ```Some(i)``` (2)?
Why yes it does! We have the same variant. The i binds to the value contained in Some, so i takes the value 5. The code in the match 
arm is then executed, so we add 1 to the value of i and create a new Some value with our total 6 inside.

Now let’s consider the second call of ```plus_one``` in code above, where x is None (4). We enter the match and compare to the first 
arm (1). It matches! There’s no value to add to, so the program stops and returns the None value on the right side of =>. Because 
the first arm matched, no other arms are compared.

Combining match and enums is useful in many situations. You’ll see this pattern a lot in Rust code: match against an enum, bind a 
variable to the data inside, and then execute code based on it. It’s a bit tricky at first, but once you get used to it, you’ll wish 
you had it in all languages. It’s consistently a user favorite.

### Matches Are Exhaustive
There’s one other aspect of match we need to discuss. Consider this version of our plus_one function that has a bug and won’t 
compile:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

We didn’t handle the None case, so this code will cause a bug. Luckily, it’s a bug Rust knows how to catch. If we try to compile 
this code, we’ll get this error:

```
error[E0004]: non-exhaustive patterns: `None` not covered
-->
|
6 | match x {
| ^ pattern `None` not covered
```

Rust knows that we didn’t cover every possible case and even knows which pattern we forgot! Matches in Rust are exhaustive: we must 
exhaust every last possibility in order for the code to be valid. Especially in the case of ```Option<T>```, when Rust prevents us 
from forgetting to explicitly handle the None case, it protects us from assuming that we have a value when we might have null, thus 
making the billion-dollar mistake discussed earlier.

## The Placeholder _
Rust also has a pattern we can use when we don’t want to list all possible values. For example, a u8 can have valid values of 0 
through 255. If we only care about the values 1, 3, 5, and 7, we don’t want to have to list out 0, 2, 4, 6, 8, 9 all the way up 
to 255. Fortunately, we don’t have to: we can use the special pattern _ instead:

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

The _ pattern will match any value. By putting it after our other arms, _ will match all the possible cases that aren’t specified 
before it. The ```()``` is just the unit value, so nothing will happen in the ```_``` case. As a result, we can say that we want 
to do nothing for all the possible values that we don’t list before the _ placeholder.

However, the match expression can be a bit wordy in a situation in which we care about only *one* of the cases. For this situation, 
Rust provides if let.

# Concise Control Flow with if let
The if ```let``` syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring
the rest. Consider the program in below code that matches on an ```Option<u8>``` value but only wants to execute code if the value 
is 3.

We want to do something with the ```Some(3)``` match but do nothing with any other ```Some<u8>``` value or the ```None``` value. 
To satisfy the match expression, we have to add ```_ => ()``` after processing just one variant, which is a lot of boilerplate code 
to add.

Instead, we could write this in a shorter way using if ```let```. The following code behaves the same as the match in code above:

```rust
if let Some(3) = some_u8_value {
    println!("three");
}
```

The syntax if let takes a pattern and an expression separated by an equal sign. It works the same way as a match, where the 
expression is given to the match and the pattern is its first arm. Using if let means less typing, less indentation, and less boiler­
plate code. However, you lose the exhaustive checking that match enforces.

Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an 
appropriate trade-off for losing exhaustive checking.

In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then 
ignores all other values.

We can include an else with an if let. The block of code that goes with the else is the same as the block of code that would go with 
the _ case in the match expression that is equivalent to the if let and else. Recall the ```Coin``` enum definition in below code, 
where the Quarter variant also held a ```UsState``` value. If we wanted to count all non-quarter coins we see while also announcing 
the state of the quarters, we could do that with a match expression like this:

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

Or we could use an if let and else expression like this:

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

If you have a situation in which your program has logic that is too verbose to express using a match, remember that if let is in 
your Rust toolbox as well.

# Summary
We’ve now covered how to use enums to create custom types that can be one of a set of enumerated values. We’ve shown how the standard 
library’s ```Option<T>``` type helps you use the type system to prevent errors. When enum values have data inside them, you can use 
```match``` or ```if let``` to extract and use those values, depending on how many cases you need to handle.

Your Rust programs can now express concepts in your domain using structs and enums. Creating custom types to use in your API ensures 
type safety: the compiler will make certain your functions get only values of the type each function expects.

In order to provide a well-organized API to your users that is straight-forward to use and only exposes exactly what your users will 
need, let’s now turn to Rust’s modules.


