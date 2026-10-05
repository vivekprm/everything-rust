# Borrowing
Now we will be looking at counterpart of **ownership** which is **borrowing**.

Borrowing is:
- The act of creating a reference to a value.
    - References are pointers with rules/restrictions.
    - References do not take ownership of values.

This is why creating references is called borrowing. You are borrowing the value instead of taking ownership
of it.

There are two reasons why you might want to borrow a value:
- Performance
    - Let's say you have a function where parameter was a string and you simply wanted to read the string. It
    would be cheaper to pass a reference to that string rather than cloning the string.
    - This is good idea for types that take up lot of memory.
- When ownership not needed or desired.
    - Going back to the example where we have function where parameter is string and you simply print it out. 
    You don't want that function being responsible for deciding when that string gets cleaned-up. So in that
    case you just borrow it, instead of moving the ownership to the function.

# Borrowing Rules
There are actually only two rules that references must follow:
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

These rules prevent two memory safety problems:
- Data races
    - First rule prevents data races, which happens when two threads try to read and write to the same memory
    location and the results are non-deterministic
- Dangling References
    - Second rule prevents dangling references, which is when a reference is pointing to invalid memory.

Let's look at below example:

```rust
fn main() {
    let s1 = String::from("Rust");  // heap allocated string
    print_string(s1);
}

fn print_string(p1: String) {
    println!("{p1}");
}
```

This code compiles but let's see what happens when we try to use s1 after calling print_string:


```rust
fn main() {
    let s1 = String::from("Rust");  // heap allocated string
    print_string(s1);
    println!("s1 is: {s1}"); // error
}

fn print_string(p1: String) {
    println!("{p1}");
}
```

We get the error which states that "we are borrowing s1 after it has already been moved."

In order to print s1, we need to borrow s1, however at this point s1 is invalid, that's because when we called
print_string, ownership of our string is transferred from s1 into p1, so s1 is no longer valid after that call.

Previously we fixed this error by cloning s1. This works, however cloning s1 is not very efficient. Also
print_string doesn't need to take ownership of the string, because it's simply printing out the string. It 
doesn't care about managing when the string gets cleaned up.

So instead of cloning s1 let's pass reference.


```rust
fn main() {
    let s1 = String::from("Rust");  // heap allocated string
    let r1 = &s1;
    print_string(r1);
    println!("s1 is: {s1}");
}

fn print_string(p1: &String) {
    println!("{p1}");
}
```

So instead of taking ownership of the string, our print_string is now borrowing the string. So s1 is still valid.

Let's add another function which mutates the string.


```rust
fn main() {
    let s1 = String::from("Rust");  // heap allocated string
    let r1 = &s1
    print_string(r1);
    add_to_string(s1);
    println!("s1 is: {s1}");
}

fn print_string(p1: String) {
    println!("{p1}");
}

fn add_to_string(p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}
```

add_to_string took one parameter that's a mutable string, mutated the string and then finally returned the
string. If p1 was not returned then p1 would be dropped at the end of this function and the string would be
cleaned up.

add_to_string takes s1 now we see same error as before at println!, s1 is invalid as it has already moved
into add_to_string. add_to_string returns the ownership, so we can fix it by shadowing.

```rust
fn main() {
    let s1 = String::from("Rust");  // heap allocated string
    let r1 = &s1
    print_string(r1);
    let s1 = add_to_string(s1);
    println!("s1 is: {s1}");
}

fn print_string(p1: String) {
    println!("{p1}");
}

fn add_to_string(p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}
```

This technically works but code is strange and not efficient. There is no reason why add_to_string needs to take
ownership of the string. Let's fix it by using references.

```rust
fn main() {
    let s1 = String::from("Rust");  // heap allocated string
    let r1 = &s1
    print_string(r1);
    let r2 = &mut s1;   // error
    add_to_string(r2);
    println!("s1 is: {s1}");
}

fn print_string(p1: String) {
    println!("{p1}");
}

fn add_to_string(p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}
```

We can make another reference r2 which is a mutable reference to s1. In Rust references are immutable by default.
To create a mutable refereces we simply add mut keyword after the &. With this we get an error stating that "we
can't borrow s1 as mutable because it's not declared as mutable". 

In order for us to declare a reference mutable, the variable itself has to be mutable. So let's make s1 mutable.

```rust
fn main() {
    let mut s1 = String::from("Rust");  // heap allocated string
    let r1 = &s1;
    let r2 = &mut s1;   // error
    print_string(r1);
    add_to_string(s1);
    println!("s1 is: {s1}");
}

fn print_string(p1: String) {
    println!("{p1}");
}

fn add_to_string(p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}
```

Now we get another error which states that 

"we can not borrow s1 as mutable because it's already borrowed as immutable"

Recall the borrow rule that **there could be only one mutable reference or many immutable references at the
same time.**

In this case, we have one immutable reference r1 and one mutable reference r2 at the same time, which is a 
violation of the rule.

We can fix this by moving print_string before defining r2. 

```rust
fn main() {
    let mut s1 = String::from("Rust");  // heap allocated string
    let r1 = &s1;
    print_string(r1);
    let r2 = &mut s1;   // error
    add_to_string(r2);
    println!("s1 is: {s1}");
}

fn print_string(p1: String) {
    println!("{p1}");
}

fn add_to_string(p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}
```
This works because Rust smart enough to know that r1 is used upto print_string and then never used again within 
the scope. So when r2 is defined, r1 is not being used. This is a feature in Rust called non lexical lifetime.

We need to change the method signature to use mutable reference instead of a mutable string. We also need to get
rid of return value as we no longer need to pass the ownership.

```rust
fn add_to_string(p1: &mut String) {
    p1.push_str(" is awesome!");
}
```

One thing you might be wondering how we are able to call push_str on p1 eventhough it's a reference. This works
because Rust has a feature called **automatic dereferencing**. This means we don't need to explicitly 
dereference p1. However, if we wanted to it would look like:

```rust
fn add_to_string(p1: &mut String) {
    (*p1).push_str(" is awesome!");
}
```

'*' is dereference operator.

Let's bring in one more function from ownership lesson:

```rust
fn main() {
    let mut s1 = String::from("Rust");  // heap allocated string
    let r1 = &s1;
    print_string(r1);
    let r2 = &mut s1;   // error
    add_to_string(r2);
    println!("s1 is: {s1}");
    let s2 = generate_string();
}

fn print_string(p1: String) {
    println!("{p1}");
}

fn add_to_string(p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}

fn generate_string() -> String {
    String::from("Ferris")
}
```

It simply creates a string and moves ownership out of the function by returning the string. 

What would happen if instead of returning the string we returned reference to a string?

```rust
fn generate_string() -> &String {
    let s = String::from("Ferris");
    &s
}
```

We will get error "missing lifetime specifier, this functions returned type contains a borrowed value but there
is no value for it to be borrowed from".

We are creating error because we are creating a dangling reference, inside the function we create a variable s
which has ownership of the string, then we return a reference to s, however at the end of this function s will
be dropped and the string will be cleaned up. Which means the reference this function returns is a dangling 
reference. It's pointing to invalid memory.

As a general rule, **if you create an owned value inside a function and you want to return the value, you have
to move ownership, you can't return a reference because the owned value will be cleaned up at the end of the 
function, so the reference will be invalid.**
