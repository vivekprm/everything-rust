# Ownership
Ownership is a strategy for managing memory (and other resources) through a set of rules checked at compile time.

There are only three rules:
- Each value in Rust has a variable that's called it's owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

# Problem Ownership Solves
- Prevents memory and resource leaks.
- Double free errors
- Use after free errors

Let's see it in action using a very simple program:

```rust
fn main() {
    let s1 = String::from("Rust");  // heap allocated string

    println!("s1 is: {s1}");
}
```

Let's look t visual diagram of how s1 is laid out in memory:

pic

s1 has two components:
- First one is pointer stored on stack. Most specifically, the stack frame for the main function.
- Second component is actual string, which is allocated on the heap.

Following the ownership rule, s1 is the owner of this data stored on the heap. So when s1 goes out of scope
data on the heap will be cleaned up.

s1 lives within the main function scope. At the end of the main function s1 will be dropped.

Let's see what happens if we create inner scope and move s1 into innerscope.

```rust
fn main() {
    {
        let s1 = String::from("Rust");
    }

    println!("s1 is: {s1}"); // compilation error
}
```

Now we get an error when we try to print s1 within the main function as s1 can't be found in the current scope.
Because now s1 is created in inner scope, it's no longer dropped at the end of the main function instead it's 
dropped at the end of the inner scope. Because of this s1 is not valid outside of this inner scope.

# Moving Ownership
```rust
fn main() {
    let s1 = String::from("Rust");
    let s2 = s1;

    println!("s1 is {s1}"); // compilation error
}
```

Now we get error at the println line stating "s1 can't be borrowed after it's already been moved."

In Rust, values are moved by default, when we assigned s1 to s2, value in s1 moved into s2 and because the
ownership rule states that we can only have one owner at a time, s2 is now the owner of the string "Rust" 
allocated on the heap and s1 is invalidated. This is why we get error on the last line.

To make it more clear, let's look at our visual diagram once again:

pic

But what happens when you want to clone the value instead of moving it?
In Rust we can do that by calling the clone method.

```rust
fn main() {
    let s1 = String::from("Rust");
    let s2 = s1.clone();

    println!("s1 is: {s1}");
}
```

Now s2 has it's own copy of "Rust" string. Now if we want to print out value of s1, we no longer get error.

Let's visualize this code change:

pic

Now s1 is pointing to it's own string on heap and s2 is pointing to it's own string on heap.

When we assign s1 to s2, we said value is moved to s2. However, that's not true for some primitive types.

```rust
fn main() {
    let x = 10;
    let y = x;
    println!("x is: {x}");
}
```

You can see in this case our code is compiling, and the reason is in this case value of x is cloned into y, 
instead of value being moved to y.

This is because in Rust, primitive which are entirely stored on stack such as integers, floating point numbers,
booleans or characters are cloned by default. These types are cheap to clone, so there is no material difference
between cloning and moving the values.

# Ownership in context of function
```rust
fn main() {
    let s1 = String::from("Rust");
    print_string(s1);

    println!("s1 is: {s1}");
}

fn print_string(p1: String) {
    println!("p1 is: {p1}");
}
```

in main function at println line we get similar compiled time error stating "s1 can't be borrowed after it has
already been moved".

Passing a variable into a function has the same effect as assigning one variable to another. When we called 
print_string, ownership of string "Rust" moved into p1. p1 lives withing the print_string function scope, so 
at the end of the print_string function p1 is dropped and string on the heap is cleaned up.

Next let's look at how ownership can be moved out of a function.

```rust
fn main() {
    let s3 = generate_string();

    println!("s3 is: {s3}");
}

fn generate_string() -> String {
    String::from("vivek")
}
```

In this case, generate_string is creating a new string with value "vivek". When the string is returned from the 
function, ownership of that string is transferred to s3. s3 will be dropped at the end of main and the string on
the heap will be cleaned up.

Let's see how functions can take ownership and give it back.

```rust
fn main() {
    let s1 = String::from("Rust");
    let s2 = s1.clone();
    let s4 = add_to_string(s2);

    println!("s4 is: {s4}");
}

fn add_to_string(mut p1: String) -> String {
    p1.push_str(" is awesome!");
    p1
}
```

we need mut keyword before p1 parameter as we are going to mutate it. add_to_string mutate the p1 string and
appends a string and returns ownership.

Now let's look at stack only datatype:

```rust
fn main() {
    let x = 10;
    let y = x;
    print_integer(x);
    println!("x is: {x}");
}

fn print_integer(i: i32) {
    println!("i is: {i}");
}
```

It works because value of x is cloned into y and when we call print_integer value in x got cloned into i.
