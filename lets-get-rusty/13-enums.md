# Enums
Here is the product that we created in previous lesson. 

```rust
struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

impl Product {
    fn new(name: String, price: f32) -> Product {
        Product {
            name: name,
            price: price,
            in_stock: true
        }
    }

    fn calculate_sales_tax(&self) -> f32 {
        self.price * 0.1
    }
    fn set_price(&mut self, price: f32) {
        self.price = price;
    }
    fn buy(self) -> i32 {
        let name = self.name 
        println!("{name} was bought!");
        123
    }
}

fn main() {
    let book = Product::new(String::from("Book"), 30.0));
}
```

Let's say we want to add a new field in our product called Category. Now we could make category a string, however
that's not ideal as there is limited set of categories, we don't want random strings also people can mis-spell a
category.

To get around these problems, instead of using a string, we could use an **enumeration** also known as **Enum**.

**Enum** allows us to define a type by enumerating it's variance. To define an enum we use enum keyword:

```rust
enum ProductCategory {
    Books,
    Clothings,
    Electronics,
}

struct Product {
    name: String,
    category: ProductCategory,
    price: f32,
    in_stock: bool
}

fn main() {
    let category = ProductCategory::Electronics;
    let product = Product {
        name: String::from("TV"),
        category: category,
        price: 200.98,
        in_stock: true
    }
}
```

Let's look at more complicated example. Imagine we are creating a text editor something similar to MS Word.
In our code we want to use the Command Pattern to describe actions that could be taken. Let's create a enum
called Command which lists all possible commands as variance.

In Rust, enums are powerful and variance can have data associated with them e.g. AddText which can hold string. It can also have struct like variance as Replace.

```rust
enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String,
    }
}

fn main() {
    let cmd = Command::Undo;
    let cmd = Command::AddText(String::from("test"));
    let cmd = Command::MoveCursor(22, 0);
    let cmd = Command::Replace{
        from: String::from("a"),
        to: String::from("b"),
    }
}
```

Just like structs we can add methods and associated functions with enums using impl blocks.

```rust
enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String,
    }
}

impl Command {
    fn serialize(&self) -> String {
        String::from("JSON String")
    }
}

fn main() {
    let cmd = Command::Undo;

    let json_string = cmd.serialize();
}
```
