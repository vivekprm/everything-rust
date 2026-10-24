In the following example, we have a trait called `UIComponent` with one method called `render`, that has a default implementation.

We also have a struct called `Button` that implements `UIComponent`.

```rust
trait UIComponent {
    fn render(&self) {
        println!("Rendering component...");
    }
}

struct Button {
    text: String
}

impl UIComponent for Button {}

fn main() {
    let button_a = Button {
        text: "button a".to_owned(),
    };
}
```

When we create a button instance this way, the button is stored on the **stack**. If we want the button to be stored on heap we can use
the **Box smart pointer**. Let's create a new button stored on heap, called button_b.

```rust
fn main() {
    let button_a = Button {
        text: "button a".to_owned(),
    };
    let button_b = Box::new(Button{ text: "button b".to_owned() });
}
```

**Box smart pointer** is included in **Rust Prelude** so we have access to it without needing a `use` statement.

If you are coming from C++, Box smart pointer is similar to **Unique Smart Pointer** in C++.

It gives you single ownership of something stored on the heap. There are a few usecases for **Box Smart Pointer**:
- Avoid copying large amount of data when transferring ownership.

Let's create two more `button_c` and `button_d`, which will be set equal to `button_c` and `button_d`

```rust
fn main() {
    let button_a = Button {
        text: "button a".to_owned(),
    };
    let button_b = Box::new(Button{ text: "button b".to_owned() });
    let button_c = button_a;
    let button_d - button_b;
}
```

Ownership of the first button `button_a` is now transferred to `button_c` and ownership of `button_d` is transferred to `button_b`.

When transferring ownership, data is copied around on the stack. So in case of `button_a` entire Button will be copied because entire
button is on the stack. However, in the case of `button_b` only the **Box smart pointer** will be copied, which is smart amount of data,
actual button will be stored on the heap. 

In this case, it doesn't matter much because button only stores one field which is a string. Imagine if button stored a lot of information
this would be more impactful.

- Another usecase for **Box smart pointer** is in combination with **Trait objects**.

E.g. let's say we wanted to create a vector of types that implement the **UIComponent** trait, we will store vector in a variable called,
components.

```rust
fn main() {
    let button_a = Button {
        text: "button a".to_owned(),
    };
    let button_b = Box::new(Button{ text: "button b".to_owned() });
    let button_c = button_a;
    let button_d - button_b;

    let components = vec![
        Box::new(button_c),
        button_d,
    ];
}
```
Rust store our vector stored boxed in buttons, but we actually want to store any type that implements **UIComponent** trait, so we will have
to add an explicit type annotation.

```rust
fn main() {
    let button_a = Button {
        text: "button a".to_owned(),
    };
    let button_b = Box::new(Button{ text: "button b".to_owned() });
    let button_c = button_a;
    let button_d - button_b;

    let components: Vec<Box<dyn UIComponent>> = vec![
        Box::new(button_c),
        button_d,
    ];
}
```
Remember that trait objects need to be stored behind some kind of pointer, in this case we are using the **Box smart pointer**.

The last usecase for the **Box smart pointer** which we will go over is, when we have a type of unknown size and we want to use it 
in a context where the exact size is required. One example of this is recursive types. Let's create a new UIComponent called container.
Container can hold other Containers.

```rust
struct Container { // error
    name: String,
    child: Container
}

impl UIComponet for Container{}
```

We get an error which states that "Container is a `recursive type` that has infinite size". We also get a hint which says "insert some
indirection using a Box Smart Pointer, a Rc smart pointer or a reference".

Rust needs to know the size of our types at compile time. However, in this case we can't tell the size of Container. To know the size of
Container, we need to know the size of the fields stored within Container. `name` field is `String` type which has a known size. However, 
the child field is a Container. So the solution is to put child behind some kind of pointer. In this case we can use the Box smart pointer.

```rust
struct Container { // error
    name: String,
    child: Box<Container>
}

impl UIComponet for Container{}
```


