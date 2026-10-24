# Deref Coercion
In below example, inside main we have a variable called `s`, which is an owned string inside of a BoxPointer. We also have a function defined
called `print` which takes a string slice and prints it out.

```rust
fn main() {
    let s = Box::new("Let's Get Rusty".to_owned());
    // &Box -> &String -> &str
    print(&s);
}

fn print(s: &str) {
    println!("{s}");
}
```

We see if we call print with `s` as argument our code is compiling, eventhough the type we are passing in is different than the type expected by
the `print` function. This works because of a feature in Rust called **Implicit Deref Coercion**.

**Deref Coercion** allows the Rust compiler to coerce a reference of one type to a reference of another type. For convenience this happens
implicitly when references are passed to functions or methods.

In this case a reference to Box smart pointer is being coerced to a Reference to String which is again coerced to a string slice.

**Deref Coercion** works only on the types that implements the `Deref` & `DerefMut` traits. 

**Deref** trait defines how the dereference operator `*` behaves (https://doc.rust-lang.org/std/ops/trait.Deref.html) for immutable references.
**DerefMut** trait defines how the dereference operator `*` behaves (https://doc.rust-lang.org/std/ops/trait.DerefMut.html) for mutable references.

The **Deref Trait** has only one required method called `deref` which takes in an immutable reference to self and returns immutable reference
to target. `Target` is an associated type. You can think of associated types like generics except that they are defined once per implementation.

```rust
pub trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}
```

In this case `target` can be any type that optionally implements the `Sized` trait (https://doc.rust-lang.org/std/marker/trait.Sized.html).

The `Sized` trait is implemented on types with a constant size known at compile time. All typed parameters are implicitly bound by the `Sized`
trait unless you use thie special syntax `?Sized` which removes the bound.

In `Deref` trait `?Sized` is used to say that `Target` could be any type whose size is known at compile time or not known at compile time.

To get better understanding of how `Deref` and `DerefMut` works, let's implement these traits on our own type.

Also let's implement `Deref` and `DerefMut` traitss on our struct `MySmartPointer`.

For `Deref` trait target is going to be whatever is stored in value. When `deref` is called we simply return reference to self.value.
For `DerefMut` trait when `deref_mut` is called we return a mutable reference to self.value. Also in main wrap s into MySmartPointer.

```rust
use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> MySmartPointer<T> {
        MySmartPointer { value }
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let s = MySmartPointer::new(
        Box::new("Let's Get Rusty".to_owned())
    );    
    print(&s);
}

fn print(s: &str) {
    println!("{s}");
}
```

In this case we create a smart pointer called MySmartPointer which just holds a generic value. 

Notice that our code is still compiling, why is that?
When we create an instance of MySmartPointer, we pass in a Box which holds an owned string. When we pass reference to &s to print 
function, we are passing a reference to MySmartPointer. Because MySmartPointer implements the `Deref` trait, a reference to 
MySmartPointer can be coerced to the reference to a value inside MySmartPointer. In this case that's going to be a reference to a
Boxed Smart Pointer.

We can also do this operation manually:

```rust
fn main() {
    let s = MySmartPointer::new(
        Box::new("Let's Get Rusty".to_owned())
    );
    // &MySmartPointer -> &Box -> &String -> &str
    let s = &(***s);
    print(&s);
}

```

Everytime we use deref operator `*` the `deref` method is called on that type. Luckily for us compiler will generate this sequence of 
deref calls implicitly.

Also the number of calls to the `deref` method is resolved at compile time so there is no runtime cost for using deref coercion.

Note that deref coercion also works by coercing mutable references to mutable references to immutable references. e.g. let's make `s`
a mutable reference.

```rust
fn main() {
    let mut s = MySmartPointer::new(
        Box::new("Let's Get Rusty".to_owned())
    );
    // &MySmartPointer -> &Box -> &String -> &str
    print(&mut s);
}

```

Notice that our code still compiles even though we are passing in a mutable reference but the print function expects an immutable reference.
The opposite is not true however and immutable reference can't be coerced to a mutable reference because of the borrowing rules.

Last thing to note is `Deref` and `DerefMut` traits are designed for Smart Pointers. So they should only be implemented on  
