# Vectors
Like arrays, vectors hold a sequence of elements of the same type. Unlike arrays vectors are growable and always
allocate memory on the heap. Vector type is defined as a struct in standard library.

There are two primary ways to create vector:

## Using new function.
Calling a new function creates an empty vector.

```rust
fn main() {
    let v = Vec::new();
}
```

If we save this code, we get error saying "type annotation needed". Because rust can't determine the type
stored in this vector. We can fix it in two ways:
- Adding an explicit type
```rust
let v: Vec<String> = Vec::new();
```

- Letting rust infer the type by adding elements to the vector.

```rust
fn main() {
    let mut v = Vec::new();
    v.push(String::from("one"));
    v.push(String::from("two"));
    v.push(String::from("three"));
} // v dropped
```

Note that when adding elements to vector, elements are moved into the vector, so the vector has ownership of
the elements. Which means when v is dropped at the end of the main function all of the elements are also dropped.

## Using vec! macro
vec! macro lets us define a vector similar to how we would define an array.

```rust
fn main() {
    let mut v = Vec::new();
    v.push(String::from("one"));
    v.push(String::from("two"));
    v.push(String::from("three"));

    let v2 = vec![1, 2, 3];
} // v dropped
```

## Indexing into vector
We can use either bracket syntax or using get method to access and element in a vector.

```rust
fn main() {
    let mut v = Vec::new();
    v.push(String::from("one"));
    v.push(String::from("two"));
    v.push(String::from("three"));

    let v2 = vec![1, 2, 3];

    let s = &v[0]; // can panic, if we pass invalid index
} // v dropped
```

If we remove the ampersand above we get error "We can't move out of index of Vec<String>". It's not allowed
because it will leave vector in invalid state.

If we want to move an element out of the vector in a safe way, we can use the `remove` method.

```rust
let s = v.remove(0);
```

It will move all the elements after that index to the left.

Second way to index into a vector is using `get` method.

```rust
let s = v.get(0);
```

Using get method is safer, because calling it with invalid index will not panic, instead get method returns an
Option. We can use the if let syntax to check if we got some variant.

```rust
let s = v.get(0);
if let Some(e) = s {
    println!("{e}");
}
```

## Iterating Over a Vector
We can use for loop to iterate over a vector.

```rust
for s in &mut v {
    s.push_str("!");
}

for s in &v {
    println!("{s}");
}
```

Let's talk about a for loop that consumes a vector

```rust
let mut v3 = vec![];

for s in v {
    v3.push(s);
}

let i = v.get(0); // error
```

Notice this time instead of taking v as reference, we are taking v as a value. This allows us to move elements
inside v to the v3 vector. However, note that after this for loop call v is no longer valid. We get error saying
"we are borrowning a value in this case v that has already been moved".

Using v like this is actually syntax sugar for calling `into_iter` on v.

```rust
for s in v.into_iter() {
    v3.push(s);
}
```

`iter()` gives immutable iteration.
`iter_mut()` gives mutable iternatons.
