# Closures
In below example, we have a `Credentials` struct which contains two fields: `username` and `password`. Imagine that this struct represents
credentials that are filled out on a webform. We want to validate that the `username` and `password` meet certain criteria e.g. `username`
and `password` can not be empty. Let's create a function that will do this for us called `validate_credentials`.

```rust
struct Credentials {
    username: String,
    password: String,
}

fn main() {
    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password".to_owned()
    }

    println!("{}", validate_credentials(&creds.username, &creds.password));
}

fn validate_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}
```

Another alternative is to have validation logic in a `closure`.

`Closures` are anonymous functions which you can store in variables or pass as arguments to other functions. e.g. lets create a variable called
`validator` which will store a closure.

```rust
struct Credentials {
    username: String,
    password: String,
}

fn main() {
    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password".to_owned()
    }

    let validator = | username: &str, password: &str | -> bool {
        !username.is_empty() && !password.is_empty()
    }

    println!("{}", validator(&creds.username, &creds.password));
}

fn validate_credentials(username: &str, password: &str) -> bool {
    !username.is_empty() && !password.is_empty()
}
```

One benefit of closure is that we can store them and pass them around. We can also store closures in the `Structs`. E.g. let's say we
wanted to add a field to `Credential` struct named `validator`. What should be the type of `validator`?

To know that we need to know the concrete type of a closure. Each closure instance has its own anonymous unique type. So in order to store
closures we need to use `generics` and `trait bounds`. We will make validator a generic type `T` and then define our generics above:

```rust
struct Credentials<T> where T: Fn(&str, &str) -> bool {
    username: String,
    password: String,
    validator: T
}
```

`T` has a trait bound of `Fn`. `Fn` is a `trait` implemented on our closure. There are actually two more traits that could be implemented
on closures, we will talk more about them later on.

The `Fn` trait has special syntax which allows us to define the signature that we expect closure to have. In this case we are saying `T` could
be any closure that accepts two string slices as argument and returns a boolean. Now that we have validator field defined let's change our main.

```rust
fn main() {
    let validator = | username: &str, password: &str | -> bool {
        !username.is_empty() && !password.is_empty()
    }

    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password".to_owned(),
        validator: validator
    }

    println!("{}", validator(&creds.username, &creds.password));
}
```

Next lets make use of our new validator field by creating impl block for `Credentials` and implementing a function called `is_valid`. It takes
immutable reference to self and returns a boolean. Inside the method body we call `self.validator` passing it `self.username` and `self.password`
as the argument. In order to call a closure stored on a field in our struct, we need to wrap it in parantheses. Let's update main to use this
method.

```rust
impl<T> Credentials<T> where T: Fn(&str, &str) -> bool {
    fn is_valid(&self) -> bool {
        (self.validator)(&self.username, &self.password)
    }
}

fn main() {
    let validator = | username: &str, password: &str | -> bool {
        !username.is_empty() && !password.is_empty()
    }

    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password".to_owned(),
        validator: validator
    }

    println!("{}", creds.is_valid());
}
```

Cool thing about this implementation is that each credential instance can have it's own validator. So far we have one simple validator which
checks if username and password are not empty. Let's create another validator which is little more complex.

```rust
fn main() {
    let validatori2 = | username: &str, password: &str | -> bool {
        !username.is_empty() && 
        !password.is_empty() &&
        password.len() > 8 &&
        password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
    };

    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password".to_owned(),
        validator: validator2
    }

    println!("{}", creds.is_valid());
}
```

One thing that's special about `Closures` is that they could capture variables in the scope which they are defined. E.g. let's create
a variable called weak password. And add a check in validator2 to see if the password passed in is not a weak password.

```rust
fn main() {
    let validator = | username: &str, password: &str | -> bool {
        !username.is_empty() && !password.is_empty()
    }

    let weak_password = "password123!"

    let validatori2 = | username: &str, password: &str | -> bool {
        !username.is_empty() && 
        !password.is_empty() &&
        password.len() > 8 &&
        password.contains(['!', '@', '#', '$', '%', '^', '&', '*']) &&
        password != weak_password
    };

    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password123!".to_owned(),
        validator: validator2
    }

    println!("{}", creds.is_valid());
}
```

Now our validation fails because we are using a weak password. Even though this closure (validator2) gets passed into the `Credential` instance,
when it's executed by calling the is_valid method it still has access to `weak_password` variable defined in main. In this case, `validator2`
immutably borrows the `weak_password`. 

`Closures` can also take mutable references to variables or take ownership of the variables. These three ways of capturing variables, mpas to
the three Closure functions.

```
Fn - Immutably borrow variables in environment.
FnMut - Mutably borrow variables in environment. These Closures can change the environment in which they were defined in.
FnOnce - Take ownership of variables in environment. Can only be called once because you can't take ownership of same variable twice.
```

Rust will automatically infer which trait a closure implements based on how variables in it's environment are used. It is also worth noting
that every closure implements the `FnOnce` trait. Because every closure can be called atleast once.

We can change our closure to take ownership of `weak_password` variable using the `move` keyword. `move` keyword can be used right before 
our closure definition.

```rust
fn main() {
    let validator = | username: &str, password: &str | -> bool {
        !username.is_empty() && !password.is_empty()
    }

    let weak_password = "password123!"

    let validatori2 = move | username: &str, password: &str | -> bool {
        !username.is_empty() && 
        !password.is_empty() &&
        password.len() > 8 &&
        password.contains(['!', '@', '#', '$', '%', '^', '&', '*']) &&
        password != weak_password
    };

    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password123!".to_owned(),
        validator: validator2
    }

    println!("{}", creds.is_valid());

    printl!("{weak_password}")
}
```

This will force closure to take ownership of `weak_password`. Now we will get error at last line if we try to print `weak_password` as we are
attempting to borrow moved value.

One usecase you will see of `move` keyword is, passing a closure to a newly created thread. In this case, we don't need to take ownership
of `weak_password`, so let's get rid of `move`.

In summary, **Closures are anonymous functions which you can store in variables and pass in as arguments to other functions. They allow you
to `define an operation`, `save it` and then `execute it` later in a different context. Closures can also capture variables from the scope 
or environment in which they were defined.**

You can take advantage of closure for code reuse and customization. In above example we were able to customize the validation logic for each
instance of the credential struct.

# Closure as Function Argument
We will implement a function called `get_default_creds` which will take a validation closure as an argument and returns a credential struct.

```rust
fn get_default_creds<T>(f: T) -> Credentials<T> where T: Fn(&str, &str) -> bool {
    Credentials {
        username: "guest".to_owned(),
        password: "password123!".to_owned(),
        validator: f
    }
}
```

Our one argument is of type `T` which is a generic with a trait bound. `T` could be any closure which takes two string slices as arguments
and returns a boolean. We use this argument to create a `Credential` struct with a default username and password but a custom validator.
In main let's create a new variable called default creds.

```rust
fn main() {
    let validator = | username: &str, password: &str | -> bool {
        !username.is_empty() && !password.is_empty()
    }

    let weak_password = "password123!"

    let validatori2 = move | username: &str, password: &str | -> bool {
        !username.is_empty() && 
        !password.is_empty() &&
        password.len() > 8 &&
        password.contains(['!', '@', '#', '$', '%', '^', '&', '*']) &&
        password != weak_password
    };

    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password123!".to_owned(),
        validator: validator2
    }

    println!("{}", creds.is_valid());

    let default_creds = get_default_creds(validator2);
}
```

# Returning Closure From a Function
We will create a function called `get_password_validator`, which will return a closure that only validates the password string.

```rust
fn get_password_validator(min_len: usize) -> impl Fn(&str, &str) -> bool {
    move |_: &str, password: &str| !password.len() >= min_len    // error
}
```

This function has one argument called `min_len` and returns a closure which validates that password has a length greater than or equal to 
`min_len`. We have to specify return type for this function and in this case we can use `impl trait syntax`. Here we are saying 
`get_password_validator` returns some type which implements the `Fn` trait with the above signature. 

If we save, we get error saying "`min_len` doesn't live long enough". This closure captures `min_len`by reference, however closure is
returned from the function while `min_len` is dropped at the end of the function. To get around this we can use the `move` keyword to
force the closure to take ownership of `min_len`.

Using the `impl trait syntax` is nice because we can take advantage of **static dispatch**. However, this syntax is only available to
us in simple cases like we have here, where we are returning just one closure.

Let's make this function little more complicated by adding another argument `special_char` which is a boolean. If `special_char` is true
we return a validator which also validates if the password has a special character. Otherwise we will return the original validator.

```rust
fn get_password_validator(min_len: usize, special_char: bool) -> impl Fn(&str, &str) -> bool {
    if special_char {
        move |_: &str, password: &str| {
            !password.len() >= min_len &&
            password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
        }
    } else {
        move |_: &str, password: &str| !password.len() >= min_len // error
    }
}
```

We now get error in else saying "`if` and `else` have incompatible types. No two closures, even if identical, have the same type consider
boxing your closure and/or using it as a trait object". Because now we are returning two different types based on a condition, we can no
longer use `static dispatch`  using `impl trait`. We have to use dynamic dispatch via trait objects. So we will return a Box smart pointer
which will contain any type that implements the `Fn` trait above. And then also Box the closures.

```rust
fn get_password_validator(min_len: usize, special_char: bool) -> Box<Fn(&str, &str) -> bool> {
    if special_char {
        Box::new(move |_: &str, password: &str| {
            !password.len() >= min_len &&
            password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
        })
    } else {
        Box::new(move |_: &str, password: &str| !password.len() >= min_len)
    }
}
```

Now let's use `get_password_validator` in main.

```rust
fn main() {
    let validator = | username: &str, password: &str | -> bool {
        !username.is_empty() && !password.is_empty()
    }

    let weak_password = "password123!"

    let validatori2 = move | username: &str, password: &str | -> bool {
        !username.is_empty() && 
        !password.is_empty() &&
        password.len() > 8 &&
        password.contains(['!', '@', '#', '$', '%', '^', '&', '*']) &&
        password != weak_password
    };

    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password123!".to_owned(),
        validator: validator2
    }

    println!("{}", creds.is_valid());

    let password_validator = get_password_validator(8, true); 

    let default_creds = get_default_creds(password_validator);
}
```

Notice that our code compiles, even though `password_validator` is a boxed smart pointer containing a trait object while `get_default_creds`
accepts a generic that implements the `Fn` trait. This is because if we go to the implementation of Box smart pointer, you will see that Box
implements `Fn`, it also implements `FnMut` and `FnOnce`. This means you can use a Boxed closure trait object where a generic implementing
a closure trait is expected.

# Function Pointers
In the following example, we have a function called `are_both_true` which takes three arguments two closures and one reference to a generic
type. The return value is a boolean. The two closures also take a reference to a generic type and return a boolean.

```rust
fn main() {}

fn are_both_true<T, U, V>(f1: T, f2: U, item: &V) -> bool
    where T: Fn(&V) -> bool, U: Fn(&V) -> bool {
    f1(item) && f2(item)
}
```

This function only returns true if only both f1 & f2 return true. You can think of this function as taking a reference to a value and two 
validators. If both the validators return true then this function returns true. Let's use this function in `main`.

```rust
fn main() {
    let greater_than = |x: &i32| *x > 10;
    let less_than = |x: &i32| *x < 20;

    let result = are_both_true(greater_than, less_than, &15);
    println!({result});
}
```

Up until now we have been discussing closures but Rust also have a notion of **Function Pointers**. 

**Function Pointers** are similar to closures except they don't capture variables in their environment. All functions coerce to a function
pointer. As an example let's convert `less_than` to a function from closure.

```rust
fn main() {
    let greater_than = |x: &i32| *x > 10;

    let result = are_both_true(greater_than, less_than, &15);
    println!({result});
}

fn less_than(x: &i32) -> bool {
    *x < 20
}
```

Notice that even though I got rid of `less_than` closure, code is still compiling. Instead of passing `less_than` closure, we are now passing
`less_than` function. This works because now we are passing a function pointer and **function pointers implement all 3 closure traits**.

This means you can pass regular functions anywhere closures are expected. 

Let's change `are_both_true` to accept `function pointers` instead of `closures`.

```rust
fn are_both_true<V>(f1: fn(&V), f2: fn(&V), item: &V) -> bool {
    f1(item) && f2(item)
}
```

**function pointers** are a concrete types represented with `fn` not to be confused with `Fn` which is a closure trait. Because function
pointers are a concrete type, we don't need to use generics. 

You can see our code is still compiling, eventhough now our function `are_both_true` is expecting two function pointers but we are 
passing in a function pointer and a closure. **Closures** could be coerced to **Function pointers** if they don't capture any variables.

Let's update `greater_than` closure so that it does capture variable in it's env.

```rust
fn main() {
    let ten = 10;

    let greater_than = |x: &i32| *x > ten;

    let result = are_both_true(greater_than, less_than, &15); // error
    println!({result});
}

fn less_than(x: &i32) -> bool {
    *x < 20
}

fn are_both_true<V>(f1: fn(&V), f2: fn(&V), item: &V) -> bool {
    f1(item) && f2(item)
}
```

Now because `greater_than` captures a varaible now in it's env, we get an error "mismatched type". To get around these errors use closures
as function arguments, so that you can pass in both `closures` and `function pointers`.

```rust
fn are_both_true<T, U, V>(f1: T, f2: U, item: &V) -> bool
    where T: Fn(&V) -> bool, U: Fn(&V) -> bool {
    f1(item) && f2(item)
}
```

# Iterator Pattern
Iterator design pattern allows different types to have a common interface for iteration while abstracting away how that iteration is implemented
internally. Iterators are used heavily in idomatic Rust code. So it's important to understand iterator design pattern and more specifically
how it's implemented in Rust.

The Rust standard library has an **Iterator Trait** which any type can implement and it looks like below:

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```
The **Iterator trait** has many methods with default implementations. The only method that required to be implemented manually is the `next`
method. Ther `next` method is called to get the next item in the iteration. It takes the mutable reference to self and returns an `Option` 
containing an Item.

Calling the next method repeatedly, keeps giving you the next item in the iteration and tell there are no more items, in which case `None`
variant is returned.

**Item** is defined above as an associated type. **Associated types** are similar to **generics** except that they are restricted to one 
concrete type per trait implementation. To understand the difference let's change `Item` to be a generic. Then implement **Iterator** on
a struct.

```rust
type Iterator<Item> {
    fn next(&mut self) -> Option<Item>
}

struct MyStruct {}

impl Iterator<String> for MyStruct {
    fn next(&mut self) -> Option<String> {
        None
    }
}

impl Iterator<i32> for MyStruct {
    fn next(&mut self) -> Option<i32> {
        None
    }
}

fn main() {
    let mut m = MyStruct{};
    let item = m.next(); // error
}
```

Because `Item` is a generic we can have another implementation of `Iterator` for `i32`.

We get an error because we have two Implementation of `Iterator` one for `String` and one for `i32`. So rust doesn't know `next` from which
implementation to call. So we need to add explicit type to help Rust compiler in finding the right implementation.

```rust
fn main() {
    let mut m = MyStruct{};
    let item: Option<String> = m.next();
}
```

At this point you might be thinking why would we have two implementations of `Iterator` on a single type? 
When iterating over a given type it makes sense that each item in that iteration is the same concrete type. It wouldn't make sense to iterate
over `MyStruct` and sometimes get a `String` and othertimes get a `i32`.

In a situation when we want to customize the return type of `next`, however we want that return type to be one concrete value. In situations
like these we could use `associated types` instead of `generics`. Let's change `Item` back to an associated type.

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct MyStruct {}

impl Iterator for MyStruct {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl Iterator for MyStruct { // error
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
```

Because now we are not using generics and Item is defined within the implementation. We get an error on second implementation stating
"conflicting implementation of trait `Iterator`for type `MyStruct`". 

Rust doesn't allow us to implement same trait for the same type twice. So we have to remove one of the implementations.

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct MyStruct {}

impl Iterator for MyStruct {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn main() {
    let mut m = MyStruct{};
    let item = m.next();
}
```

So In summary, If you have a generic type and you want to restrict it to one concrete type per implementation then use `associated types`.
Otherise, if you want to allow multiple implementation with different concrete types then use `generics`.

There is one more Iterator trait in the standard library that we should know about, that trait is called `IntoIterator`.

```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
```

`Iterator` is implemented on a type which you can iterate over and `IntoIterator` is implemented on a type which you can turn into an
`Iterator`. E.g. a `Vector` is a collection which you can turn into an `Iterator`.

The `IntoIterator` has one method called `into_iter` which consumes self and returns an `Iterator`.

In below example, we have two structs `Person` with three fields and `PersonIterator` with one field called `values` which is a vector of
strings. We want the ability to iterate over the fields in `Person`  which we can do because all of them are of same type.

First we will need a struct which holds iterator state and then we need to implement the `Iterator` trait on that struct. Now, we could 
include the iterator state inside the Person struct but let's say we wanted to have a separation of concern, we don't want to mix iterator
state with state associated with a Person. So instead, we create a new struct called `PersonIterator` which is responsible for iterator
state.

```rust
struct Person {
    first_name: String,
    last_name: String,
    occupation: String,
}

struct PersonIterator {
    values: Vec<String>
}
```

Next let's implement the `Iterator` trait for `PersonIterator`.

```rust
impl Iterator for PersonIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.values.is_empty() {
            return None;
        }
        Some(self.values.remove(0))
    }
}
```

Now that our iterator struct is complete let's implement `IntoIterator` for the `Person` struct.

```rust
impl IntoIterator for Person {
    type Item = String;
    type IntoIter = PersonIterator;

    fn into_iter(self) -> Self::IntoIter {
        PersonIterator {
            values: vec![
                self.first_name,
                self.last_name,
                self.occupation,
            ]
        }
    }
}
```

`into_iter` method has to return an Iterator, which `PersonIterator` is because it implements the `Iterator` trait. So all we have to so is,
return a new instance of `PersonIterator` where the vector values are set to `first_name`, `last_name` & `occupation`.

Next let's see how we can iterate over a `Person` struct in main.

```rust
fn main() {
    let p = Person {
        first_name: "John".to_owned(),
        last_name: "Doe".to_owned(),
        occupation: "Software Engineer".to_owned()
    }
}
```

First we will iterate over this `Person` instance manually.

```rust
fn main() {
    let p = Person {
        first_name: "John".to_owned(),
        last_name: "Doe".to_owned(),
        occupation: "Software Engineer".to_owned()
    }
}

let mut i = p.into_iter(); // returns an instance of PersonIterator

println!("{:?}", i.next());
println!("{:?}", i.next());
println!("{:?}", i.next());
println!("{:?}", i.next());
```

We get the expected result. However we don't have to call iterator manually, we can use a for loop to automatically iterate through `Person`
struct.

```rust
fn main() {
    let p = Person {
        first_name: "John".to_owned(),
        last_name: "Doe".to_owned(),
        occupation: "Software Engineer".to_owned()
    }
}

for item in p {
    println!("{item}");
}
```

In Rust `for` loops know how to handle iterators. This `for` loop knows that eventhough p itself is not an `Iterator`, it could be turned into
an `Iterator`, producing items of type `String`. When the `None` variant is returned, this `for` loop will automatically end.

One last thing to know is that the `PersonIterator` struct is not necessary, we added it for demonstration. But `vectors` itself can be turned
into `Iterator`. So instead of returning a `PersonIterator` we could return a vector iterator. To do that we simply need to call `into_iter` on
our vector and change the return type to `std::vec::IntoIter`.

```rust
impl IntoIterator for Person {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            self.first_name,
            self.last_name,
            self.occupation,
        ].into_iter()
    }
}
```

# Iterating Over Collections
One common usecase for `Iterators` is to perform operation on elements in a collection. Collections in Rust standard library can be turned
into `Iterator`. We saw above vector can be turned into iterators using `into_iter`. In below example we will turn a `HashMap` into `Iterator`.

There are 3 common methods that make `Iterator` from a `Collection`.
- `iter()`: Gives `Iterator` over immutable references to items in the collection.
- `iter_mut()`: Gives an `Iterator` over mutable references to items in the collection.
- `into_iter()`: Returns an `Iterator` over the owned values in a collection.

In case of hashmap only the values are mutable, keys are immutable because changing the key will messup the internal state of the `HashMap`.

When using collections in a `for` loop, Rust will automatically know which type of iterator you want based on how you reference the collection.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("red team".to_owned(), 2);
    scores.insert("blue team".to_owned(), 8);
    scores.insert("green team".to_owned(), 6);

    let mut scores_iter = scores.iter();
    let first = scores_iter.next()
}
```

Let's iterate over the scores HashMap using immutable references to items inside the HashMap.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("red team".to_owned(), 2);
    scores.insert("blue team".to_owned(), 8);
    scores.insert("green team".to_owned(), 6);

    for (team, score) in &scores {
        println!("{team} got: {score} points.");
    }
}
```

In above for loop, scores is borrowed immutably which is how for loop knows to create an `Iterator` over immutable references to the items
inside of scores. If we borrowed scores mutably, then we will get the `Iterator` with mutable references to items inside scores.

```rust
for (team, score) in &mut scores {
    println!("{team} got: {score} points.");
}
```

Finally, if we used owned form of scores, our for loop will take ownership of items inside scores.

```rust
for (team, score) in scores {
    println!("{team} got: {score} points.");
}
```

# Combinators
In the following example, we have a vector of strings in main which represents students. Each string contains student's name and their GPA.
There is also a `Student` struct containing two fields, `name` which is a `String` and `gpa` which is `f32`.

```rust
#[derive(Debug)]
struct Student {
    name: String,
    gpa: f32,
}

fn main() {
    let students = vec![
        "Bogdan 3.1",
        "Wallace 2.3",
        "Lidiya 3.5",
        "Kyle 3.9",
        "Anatoliy 4.0"
    ];
}
```

Let's say we wanted to convert this vector of `String` into a vector of `Student` structs. Additionally, we want to filter out any student
that got less than 3.5 gpa. One way we can accomplish this is by using a `for` loop.

```rust
fn main() {
    let students = vec![
        "Bogdan 3.1",
        "Wallace 2.3",
        "Lidiya 3.5",
        "Kyle 3.9",
        "Anatoliy 4.0"
    ];
}

let mut good_students = vec![];

for s in students {
    let mut s = s.split(' ');
    let name = s.next();
    let gpa = s.next();

    if let (Some(name), Some(gpa)) = (name, gpa) {
        let name = name.to_owned();
        let gpa = gpa.parse::<f32>();
    }
}
```

Here we need to tell `parse` what we want to parse this string into. To do that we use this syntax `parse::<f32>`, called the **turbo fish**
syntax. The **turbo fish** syntax is used when a function defines a generic, however, it's unclear what concrete type should substitute that
generic. Next thing is to check if gpa is `Ok` variant.

```rust
fn main() {
    let students = vec![
        "Bogdan 3.1",
        "Wallace 2.3",
        "Lidiya 3.5",
        "Kyle 3.9",
        "Anatoliy 4.0"
    ];

    let mut good_students = vec![];

    for s in students {
        let mut s = s.split(' ');
        let name = s.next();
        let gpa = s.next();

        if let (Some(name), Some(gpa)) = (name, gpa) {
            let name = name.to_owned();
            let gpa = gpa.parse::<f32>();

            if let Ok(gpa) = gpa {
                if gpa >= 3.5 {
                    good_students.push(Student{name, gpa});
                }
            }
        }
    }

    for s in good_students {
        println!("{:?}", s);
    }
}
```

Our code works, however, it's not the cleanest code. E.g. we have three nested if statements inside our for loop. Let's see how we can
simplify this code using `iterators` and `combinators`.

```rust
fn main() {
    let students = vec![
        "Bogdan 3.1",
        "Wallace 2.3",
        "Lidiya 3.5",
        "Kyle 3.9",
        "Anatoliy 4.0"
    ];

    let mut good_students = students.iter();
}
```

`iter` is a combinator which will return an `Iterator` over the strings in students vector.

**Combinators** are small pure functions which perform a specific task that could be chain together to perform complex operations.
**Iterators** have lots of useful methods with default implementations e.g. `map` method which is also a combinator.

The `map` method converts items in an `Iterator` from one type to another type. It takes a **closure** which will be executed for
every item in the `Iterator`. The `Closure` accepts one argument which is going to be the item and returns the new type. In this case
we want to map every student string into a `Student` struct.

```rust
fn main() {
    let students = vec![
        "Bogdan 3.1",
        "Wallace 2.3",
        "Lidiya 3.5",
        "Kyle 3.9",
        "Anatoliy 4.0"
    ];

    let mut good_students = students.iter().map(|s| {
        let mut s = s.split(' ');
        let name = s.next()?.to_owned();
        let gpa = s.next()?.parse::<f32>().ok()?;

        Some(Student { name, gpa })
    });
}
```

At the end of the map call above we get an `Iterator` over optional `Student` types. The next thing we want to do is get rid of the `None`
variants. To do that we can call `flatten` combinator. 

`flatten` combinator will create a new iterator that flattens nested structure. In the case of `Option` type, it will extract out `Student` 
from the `Some` varaint and get rid of all the `None` variants.

Finally, we will call the `filter` combinator to filter out all the students with gpa < 3.5.
`filter` creates new `Iterator` with items filtered out. It accepts a closure which will be executed for every item in the `Iterator`.
The `closure` takes one argument which is going to be the item and returns a boolean expression. If the boolean expression evaluates to
`false` then that item would be filtered out.

Last thing to do is, turn this `Iterator` into a `Collection`, we can do that by calling the `collect` combinator. We also need to add
explicit type annotation to `good_students` to specify which collection we want.

```rust
let mut good_students: Vec<Student> = students.iter().map(|s| {
        let mut s = s.split(' ');
        let name = s.next()?.to_owned();
        let gpa = s.next()?.parse::<f32>().ok()?;

        Some(Student { name, gpa })
    })
    .flatten()
    .filter(|s| s.gpa >= 3.5)
    .collect();
```

Now we have same functionalty implemented using the `combinator` and the code is a lot cleaner.

One thing to note is that `Iterator` and methods on `Iterator` such as `map`, `flatten`, `filter` etc. are lazy. This means, they
won't actually do any work until asked to iterate over items by calling the `next` method or the method that internally calls `next.

For example, up until `filter` no work is actually done as soon as we call the `collect` method which internally calls `next`, at that point
all the previous transformations are executed for each item in the `Iterator`.

Another thing to note is, that using iterators and methods on `Iterator` like `map`, `flatten`, `filter` are zero cost abstractions. That means
this code will be just as fast as `for` loop we wrote earlier.

In this example we used several combinators for demonstrations. However, we can remove the call to `flatten` and `filter` and replace `map`
with `filter_map` to make it much simpler.

`filter_map` will create an `Iterator` that both filters and maps. It takes a `closure` that returns `Option`. If `None` variant is returned
then that item will be filtered out. So before returning a `Student` will check if the student's gpa is less than 3.5 and if so, return the
`None` variant.

```rust
let mut good_students: Vec<Student> = students.iter().filter_map(|s| {
        let mut s = s.split(' ');
        let name = s.next()?.to_owned();
        let gpa = s.next()?.parse::<f32>().ok()?;

        if gpa < 3.5 {
            return None
        }
        Some(Student { name, gpa })
    })
    .collect();
```

