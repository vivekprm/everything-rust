One of the memory safety guarantee in Rust is that there is no dangling references. To accomplish this borrow checker tracks
the lifetime of values and checks to make sure that references are valid at compile time. So what are lifetimes?

Well there are two types:
- Concrete Lifetimes
- Generic Lifetimes

# Concrete Lifetimes
A concrete lifetime is the time during which a value exists at a particular memory location. A lifetime starts when a value is created
or moved into a particular memory location and ends when a value is dropped or moved out of a memory location. Let's look at following
example:

```rust
fn main() {
    let s1 = String::from("Let's Get Rusty!");
    println!("s1: {s1}");
}
```

We define s1 at line 1 which is a heap allocated string and then we print s1 on next line. Lifetime of s1 begins at line 2 when the string
is created and ends on line 4 when s1 goes out of scope.

Let's introduce and inner scope:

```rust
fn main() {
    {
        let s1 = String::from("Let's Get Rusty!");
    }
    println!("s1: {s1}");   // error
}
```

Now lifetime of s1 starts on line 3 and ends on line 4 at the end of the inner scope. This is why we get error at line 5 as s1 is no longer
valid.

Let's create a new variable called s2, assign it to s1 and try to print out s1 again.

```rust
fn main() {
    let s1 = String::from("Let's Get Rusty!");
    println!("s1: {s1}");

    let s2 = s1;
    println!("s1: {s1}");   // error
}
```

In this example lifetime of s1 starts at line 2 and ends on line 4 when value in s1 is moved into s2. At this point s2 contains the string
and s1 is invalid, which is why we get error at line 5 when we try to print s1 again.

So far we have been looking lifetime of owned values, next let's look at the example using references.

```rust
fn main() {
    let r1;
    {
        let s1 = String::from("Let's Get Rusty!");
        r1 = &s1; // error
    }

    println!("r1: {r1}");
}
```

On line 5 we get an error stating "s1 deosn't live long enough borrowed value doesn't live long enough."

s1 is dropped at line 6, yet a borrowed s1 is used on line 8. In order to ensure that there are no dangling references the borrow checker
checks at compile time, that the lifetime of a reference is contained within the lifetime of the value that it's borrowing.

In above example we are breaking that rule, r1 is our reference. It's defined on line 2 and it's lifetime ends at the end of main function
on line 9. However, the value that it borrows has a shorter lifetime. Lifetime of s1 starts at line 4 and ends on line 6 at the end on the
inner scope. At the end of the innerscope s1 will be dropped and this string will be cleaned up. Which means on line 8, r1 is referencing
invalid memory. This is why error states "s1 doesn't live long enough for r1 to be used on line 8".

We can fix this by moving the line 8 inside the inner scope.

Let's take another example using mutable and immutable references.

```rust
fn main() {
    let mut s1 = String::from("Let's Get Rusty!");
    let r1 = &s1;
    println!("r1: {r1}");
    let r2 = &mut s1;
    r2.push_str("!");
}
```

On line 2 we declare a mutable variable called s1, which is a heap allocated string. On line 3 we declare immutable reference called r1
and then print r1 on line 4. One line 5 we we declare a mutable reference to s1 called r2 and then use the `push_str` method to append 
an exclaimation mark to the string.

If you recall that borrowing rule states that there could be either one mutable reference at a given time or multiple immutable references
at a given time. In our example, we have one mutable reference and one immutable reference seemingly at the same time. So why is our code
compiling?

In this case Rust is smart enough to realise that r1 and r2 don't live for the entire duration of the scope. r1 is declared at line 3 and
it's last usage is at line 4. So it's lifetime is from line 3 to line 4. r2 is declared at line 5 and it's last usage is at line 6. So the
lifetime of r2 is from line 5 to line 6.

The technical term for this is **non-lexical lifetimes**, which means lifetimes that are not tied to scope. Let's see what happens if we
move print statement at line 4 to line 7?

```rust
fn main() {
    let mut s1 = String::from("Let's Get Rusty!");
    let r1 = &s1;
    let r2 = &mut s1; //
    r2.push_str("!");
    println!("r1: {r1}");
}
```

Now we get an error on line 4 which states that "we can not borrow s1 as mutable because it's also borrowed as immutable".
The lifetime of r1 is from line 3 to line 6. Lifetime of r2 is from line 4 to line 5. Because the lifetimes overlap we are now using 
immutable reference and a mutable reference at the sametime, which is against the borrowing rules.

One of the ways to fix is by moving the declaration of r1 down two lines.

```rust
fn main() {
    let mut s1 = String::from("Let's Get Rusty!");
    let r2 = &mut s1; //
    r2.push_str("!");
    let r1 = &s1;
    println!("r1: {r1}");
}
```

Now the lifetime of our mutable reference and liftime of our immutable reference don't overlap.

# Generic Lifetimes
In below example we are simulating a game with two players and check which player goes first and print it out.

```rust
fn main() {
    let player1 = String::from("player1");
    let player2 = String::from("player2");

    let result = first_turn(player1.as_str(), player2.as_str());

    println!("Player going first is: {}", result);
}

fn first_turn(p1: &str, p2: &str) -> &str {
    if rand::random() {
        p1
    } else {
        p2
    }
}
```

`first_turn` function accepts two parameters p1 & p2 which are both string slices and also returns a string slice.

From the perspective of borrow checker how do we know that result is not a dangling reference when we print it out?
To figure that out borrow checker will look at the lifetime of result. result is going to be the return value of `first_turn` and return
value of `first_turn` is going to be a reference to specifically a string slice. So what's the time of returned reference?

That's ambiguous as returned reference could either be p1 or p2 and both could have different lifetime because of this ambiguity, borrow 
checker is not able to analyze our code resulting in an error. The error states that we are "missing a lifetime specifier".

**Lifetime Specifier** otherwise known as **Generic Lifetime annotation** are a way to describe the relationship between lifetime of
references. They are not concrete lifetimes but rather describe the relationship between concrete lifetimes.

Let's fix this error by introducing a **generic lifetime annotation**. We define **generic lifetime annotation** just like regular generics
we define them inside angle brackets after the function name along with any other generics.

The way we distinguish lifetime annotations from regular generics is by using an `'` at the start of the name. Below we have defined a
lifetime called `a`. Also add it to parameters and return values.


```rust
fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random() {
        p1
    } else {
        p2
    }
}
```

In this case we are saying p1, p2 and return value have the same lifetime?
Well not exactly! remember that a is not a concrete lifetime rather it describes the relationship between lifetimes. In this case we are
saying that there is a relationship between p1, p2 and the return value. The relationship is this:

**The lifetime of the retrun value is going to be equal to the shortest lifetime passed in**.

Now we don't see any error, because borrow checker knows that lifetime of result is smallest between the parameter passed in, p1 and p2
will be valid and we can see there lifetime is at the end of main. So borrow checker knows lifetime of result is till the end of `main`.

In the above example player1 and player2 have exactly the same lifetime. Now let's look at an example where both have different lifetime.

```rust
fn main() {
    let player1 = String::from("player1"); 
    {
        let player2 = String::from("player2");

        let result = first_turn(player1.as_str(), player2.as_str());

        println!("Player going first is: {}", result);
    }
}

fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random() {
        p1
    } else {
        p2
    }
}
```

In this example, player1 has lifetime that ends at line 8 while player 2 is lifetime ends at line 7. This means that the lifetime of
result is going to be till line 7 because that's the shortest lifetime passed in. Our code is still valid because we are printing the
result at line 7.

If we print the result after the innerscope ends, we will get compilation error stating "player2 doesn't live long enough borrowed value
doesn't live long enough".

```rust
fn main() {
    let player1 = String::from("player1");
    let result;
    {
        let player2 = String::from("player2");

        result = first_turn(player1.as_str(), player2.as_str());
    }
    println!("Player going first is: {}", result);
}

fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random() {
        p1
    } else {
        p2
    }
}

```

Imagine if `first_turn` returned a reference to player2, however lifetime of player2 ends at the end of the innerscope and hence result
will have a dangling reference at line 8.

We could specify different lifetime relationships depending upon what our function is doing. E.g. let's say our function always returned p1.


```rust
fn main() {
    let player1 = String::from("player1");
    let result;
    {
        let player2 = String::from("player2");

        result = first_turn(player1.as_str(), player2.as_str());
    }
    println!("Player going first is: {}", result);
}

fn first_turn<'a>(p1: &'a str, p2: str) -> &'a str {
    p1
}

```

Notice now we don't have any error and that's because lifetime of result is same as lifetime of first parameter passed in i.e. reference to
player1.

One thing to note is, in general lifetime of a return value must be tied to the lifetime of the input parameters. This is because if a
function returns a reference that reference has to point something passed in, otherwise if a function returns something created inside
the function that reference will be invalid. E.g. let's create an owned string and pass back a reference to it.

```rust
fn main() {
    let player1 = String::from("player1");
    let result;
    {
        let player2 = String::from("player2");

        result = first_turn(player1.as_str(), player2.as_str());
    }
    println!("Player going first is: {}", result);
}

fn first_turn<'a>(p1: &'a str, p2: str) -> &'a str {
    let s = "example".to_owned();
    s.as_ref(); // error
}
```

Now we get an error stating "can not return reference to local variable `s` returns a reference to data owned by the current function".

The return value is referencing `s`. `s` is created inside of this function so at the end of this function, `s` will go out of scope and
it's data will be cleaned up meaning that the reference returned will be invalid.

One exception to this is using static lifetime. **Static Lifetime** is special lifetime that defines a reference that can live for the entire
duration of the program. One example where **static lifetime** is used are **string slices**.

E.g. Let's create a new string slice, we can specify lifetimes as below:

```rust
fn first_turn(p1: &str, p2: str) -> &'static str {
    let s &'static str = "Let's Get Rusty!";
    s;
}
```

String slices have **static lifetimes** because they live in the program's binary. Meanings that they are valid for the enitre duration of 
the program. This means we can return string slices created inside of a function 
