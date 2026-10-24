So far when using structs we have always stored owned types. E.g. below we have a struct called Tweet with a field called content,
which stores an owned string.

```rust
struct Tweet {
    content: String,
}

fn main() {
    let tweet = Tweet {
        content: "example".to_owned(),
    }
}
```

Let's say that instead of an owned string, we wanted to store string slice. 

```rust
struct Tweet {
    content: &str,  // error
}
```

Now we get an error stating "missing lifetime specifier". When storing references in structs we must add a generic lifetime annotation.
Just like with generics will specify lifetime annotation inside angle bracket after the name of the struct.

```rust
struct Tweet<'a> {
    content: &'a str,
}

fn main() {
    let tweet = Tweet {
        content: "example",
    }
}
```

next let's create an implementation block for Tweet. We get an error saying that "implicit elided lifetime not allowed here". 

Just like with Generics, we must include Generic Lifetime annotation inside the definition of our impl block.

```rust
impl<'a> Tweet<'a> struct {
    fn replace_content(&mut self, content: &str) -> &str {
        let old_content = self.content;
        self.content = content; // error
        old_content
    }
}
```

We get an error stating that "explicit lifetime required in the type of `content`". Specifically, the `'a` lifetime.

This makes sense, tweet has a field called `content` with a lifetime of `'a`. So the content passed into this function that would
replace the old content should have a compatible lifetime. To fix this we simply add `'a` in content lifetime as well.

```rust
impl<'a> Tweet<'a> struct {
    fn replace_content(&mut self, content: &'a str) -> &str {
        let old_content = self.content;
        self.content = content;
        old_content
    }
}

fn main() {
    let tweet = Tweet {
        content: "example"
    };

    let old_content = tweet.replace_content("replace_example"); // error
}
```

Here we get an error stating that: "can not borrow tweet as mutable because it's not declared as mutable".

Remember that replace content takes a mutable reference to self, so we have to make our tweet variable as mutable.

```rust
fn main() {
    let mut tweet = Tweet {
        content: "example"
    };

    let old_content = tweet.replace_content("replace_example");
}
```

Now let's print out old content and tweet content.

```rust
fn main() {
    let mut tweet = Tweet {
        content: "example"
    };

    let old_content = tweet.replace_content("replace_example");
    println!("{old_content}");
    println!("{}", tweet.content);
}
```

Next, let's talk about lifetime ellison, first let's create a function called `take_and_return_content`.

```rust
fn take_and_return_content(content: &str) -> &str {
    content
}
```

Our function simply takes a parameter called `content` which is a string slice and then returns it. Notice that, eventhough we are
taking in a reference and returning a reference we don't need to add lieftime annotations, why is that?
This is becasue the Rust compiler follows three lifetime elision rules, after applying the three rules if lifetime is still ambiguous
an error will be thrown requiring a explicit lifetime annotation. 

Inorder to understand the elision rules, first we need to understand **input lifetime** and **output lifetime**.

Lifetimes of function or method parameters are called **input lifetime**.
Lifetimes of return values are called **output lifetime**.

Here are the three lifetime elision rules:
- Each parameter that is a reference gets it's own lifetime parameter. So let's add a lifetime to `content` in above function. 
- If there is exactly one **input lifetime** parameter, that lifetime is assigned to all **output lifetime** parameters.
    - In this case we do have only one input parameter so we can assign that lifetime to our return value.
    - Just by using these two rules rust compiler was able to determine the lifetimes of this function. This is why anytime we have
    exactly one reference as input parameter we don't need to explicitly annotate lifetimes.
- If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to
all output lifetime parameters.

```rust
fn take_and_return_content<'a>(content: &'a str) -> &'a str {
    content
}
```

Let's see what would happen if we add another reference `content2` as **input parameter**. Following the first rule let's add a lifetime
to content2.

```rust
fn take_and_return_content<'a, 'b>(content: &'a str, content2: &'b str) -> & str { // error
    content
}
```

Second rule only applies when there is only one reference, in this case we have two reference so we can't assign a lifetime to return value.
The third rule, only applies to methods and here we are using a function so the third rule is not applicable. Notice we get an error 
stating that "we are missing a lifetime specifier". The **Rust Compiler** apllied all the 3 lifetime elision rules but was not able to
determine the lifetime of return value. This means we have to explicitly add a lifetime annotation. In this case we know that we are always
goint to return `content` which has a lifetime of `a`. So let's assign `a` to return value.

```rust
fn take_and_return_content<'a, 'b>(content: &'a str, content2: &'b str) -> &'a str {
    content
}
```

With the current function signature, we are saying that the lifetime of the return value is tied to `content`. We are completely ignoring
`content2`.

But remember lifetime annotations are not concrete lifetimes, they simply describe the relationship between the lifetimes of references.
So we can change these relationships e.g. instead of giving `content2` it's own lifetime we can say that `content`, `content2` and 
return type all have a lifetime `a`.

```rust
fn take_and_return_content<'a>(content: &'a str, content2: &'a str) -> &'a str {
    content
}
```

This new relationship states that lifetime of the return value will be equal to the shortest lifetime passed in.

Now let's see how third rule is used, by looking at our trait implementation.

```rust
impl<'a> Tweet<'a> struct {
    fn replace_content(&mut self, content: &'a str) -> &str {
        let old_content = self.content;
        self.content = content;
        old_content
    }
}

```

Third rule states that "If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of `self` 
is assigned to all output lifetime parameters."

This is why in `replace_content` function we didn't have to explicitly annotate the lifetime of the return value. The lifetime of return
value is equal to lifetime of `self` as per the third rule.
