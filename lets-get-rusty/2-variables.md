# Create a variable

```rust
let a = 5;
```

rust infers the type of variable based on the value provided e.g. in this case rust infers it as i32, a 32 bit 
integer.

However, we can also explicitly specify the type:

```rust
let a: i16 = 5;
```

# Mutability
Let's say we have another variable assigned a value 5 and then we change it to 10.

```rust
let b = 5;
b = 10;
```

In this case we get compile time error saying "we can't assign twice to an immutable variable".

In Rust variables are immutable by default. So we need to change it to mutable variable if we want to reassign.

```rust
let mut b = 5;
b = 10;
```

# Shadowing

```rust
let c = 5;
let c = 10;
```

Second statement shadows the first assignment statement. So value of c is 10.
We can confirm it using println! macro.

```rust
let c = 5;
let c = 10;
println!("c is : {c}")
```

Difference is, with mutability you are modifying a single variable, while with shadowing you are creating 
two separate variables and one of them is shadowing the other.

# Scope
In Rust variable live in a scope, area between opening '{' and closing '}'. E.g. variable 'd' below lives 
in main function scope.

```rust
fn main() {
    let d = 5;
    println!("d is {d}");
}
```

Let's create another inner scope:

```rust
fn main() {
    let d = 5;
    {
        let d = 12;
        println!("inner d is: {d}");
    }
    println!("d is: {d}")
}
```

inner scope has access to variable in outerscope. d inside innerscope will shadow the d in outer scope
and print 12.
