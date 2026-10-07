# Control Flow
## Conditional statements

```rust
fn main() {
    // if/else
    let a = 5;

    if a > 5 {
        println!("bigger than 5");
    } else if a > 3 {
        println!("bigger than 3");
    } else {
        println!("smaller or equal to 3");
    }
}
```

Conditionals can also be used in let statements.

```rust
let b = if a > 5 { 1 } else { -1 }
```

Note that return type of both the branches must be the same.

## Loops

### loop
First type of loop in rust is simply called `loop`. This type of loop will execute forever unless you explicitly
exit the loop using break.

```rust
fn main() {
    loop {
        println!("loop forever");
    }
}
```

We can also label the loop and break to a label. Label starts with a single quote.

```rust
fn main() {
    'outer: loop {
        println!("loop forever");
        loop {
            break 'outer;
        }
    }
}
```

We can also return values from our loop. E.g.

```rust
fn main() {
    let x = loop {
        break 5;
    };
}
```

You could use this pattern if you have an operation that could fail but you want to keep trying that operation
until it succeeds and then use the returning value for your variable.

### while loop
While loops will continue to execute while a given condition is true.

```rust
fn main() {
    let mut a = 0;

    while a < 5 {
        println!("a is {a}");
        a = a+1;
    }
}
```

### for loops
For loops allow us to loop through collections e.g. below we are looping over array elements:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5, 6];

    for element in a {
        println!("{}", element);
    }
}
```
