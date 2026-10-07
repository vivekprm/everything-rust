# Functions
Functions are defined using the `fn` keyword, followed by the name of the function.

Naming convention for function name is snake case.

```rust
fn my_function(x: i32) -> i32 {
    println!("my_function called with: {}", x);

    let y = 10;
    y
}
```

We use '->' to specify the return type. In rust the final expression in a function will be used as a return value.
Expressions are things that evaluate to a returning value, whereas statements are instructions that doesn't 
return any value.

Note: to use last expression as return value in a function, we have to omit ';'.
