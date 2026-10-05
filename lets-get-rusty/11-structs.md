# Structs
Structs allow us to group related data together. Let's create a struct to represent products in our online store.

```rust
struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

fn main() {
    let book = Product {
        name: String::from("Book"),
        price: 28.85,
        in_stock: true
    }

    let price: f32 = book.price;
    book.in_stock = false; // error as book is immutable
}
```

# Adding Functionality to a Struct
Suppose we want ability to calculate sales tax for a product. Let's create a function for that:

```rust
fn main() {
    let book = Product {
        name: String::from("Book"),
        price: 28.85,
        in_stock: true
    }
    let sales_tax = calculate_sales_tax(book);
    println!("sales tax: {}", sales_tax);
}

fn calculate_sales_tax(product: &Product) -> f32 {
    product.price * 0.1
}
```

# Implementation Blocks
Our code above is working but it can be improved. calculate_sales_tax function is completely separate from the
product type, even though calculating sales taxes is linked with products. Ideally calculate_sales_tax should
be defined on product type itself. We can do exactly that with implementation blocks.

Implementation type allow us to implement functionality for a given type. E.g. we can create a new implementation
block for our product struct by using the `impl` keyword.

```rust
struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

impl Product {
    fn calculate_sales_tax(&self) -> f32 {
        self.price * 0.1
    }
}

fn main() {
    let book = Product {
        name: String::from("Book"),
        price: 28.85,
        in_stock: true
    }
    let sales_tax = book.calculate_sales_tax();
    println!("sales tax: {}", sales_tax);
}
```

Methods are declared by having the first parameter in a function be self. However there are 3 forms of self
that a method could take. 
- First form is immutable borrow to self as above.
    - This form is used when you want to reference a field of self without modifying anything.
- Second form is a mutable borrow to self. e.g. let's say we wanted to create a method called set price.

```rust

struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

impl Product {
    fn calculate_sales_tax(&self) -> f32 {
        self.price * 0.1
    }

    fn set_price(&mut self, price: f32) {
        self.price = price;
    }
}
```

- Last form of self that a method could take is owned form of self.
    - This is usually done when you want to transform one type to another type while also preventing the caller
    from using the original instance. e.g. let's create a new method `buy`, which takes the owned form of self.

```rust
struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

impl Product {
    fn calculate_sales_tax(&self) -> f32 {
        self.price * 0.1
    }
    fn buy(self) -> i32 {
        let name = self.name 
        println!("{name} was bought!");
        123
    }
}
```

Buy takes owned form of self, which means ownership of instance will be passed to this method. Inside method body
we are getting name from self and not doing anyting else, so at the end of the method self will be dropped and
instance will no longer be valid. Let's call all these methods inside main function.

```rust
struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

impl Product {
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
    let mut book = Product {
        name: String::from("Book"),
        price: 28.85,
        in_stock: true
    }
    let sales_tax = book.calculate_sales_tax();
    println!("sales tax: {}", sales_tax);

    book.set_price(1.0);
    book.buy();
}
```

Let's see what happens if we want to change the price again after we already bought the book.

```rust
fn main() {
    let mut book = Product {
        name: String::from("Book"),
        price: 28.85,
        in_stock: true
    }
    let sales_tax = book.calculate_sales_tax();
    println!("sales tax: {}", sales_tax);

    book.set_price(1.0);
    book.buy();
    book.set_price(2.0); // error
}
```

We get an error stating that we are attempting to borrow a value that has already been moved. The `buy` method
takes the owned form of self which means instance of book will be moved into the buy method and then dropped.
So after we call buy, book instance is no longer valid.

## Associated Functions
Associated functions are sometimes called static methods or static functions in other languages. Associated
functions are associated with a type, however they don't work on instances of that type. e.g. let's create
an associated function called get_default_sales_tax.

```rust
impl Product {
    fn get_default_sales_tax() -> f32 {
        0.1
    }

    fn calculate_sales_tax(&self) -> f32 {
        self.price * Product::get_default_sales_tax()
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
```

As you can see only difference is associated functions don't take `self` as parameter.
Another difference is, while calling associated functions we don't use dot syntax. We use 
type::associated_function e.g. in this case Product::get_default_sales_tax().

A common pattern you will see in Rust is for types to have an associated function called `new`, which acts
as a constructor. Let's create one for Product.

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

    let sales_tax = book.calculate_sales_tax();
    println!("sales tax: {}", sales_tax);

    book.set_price(1.0);
    book.buy();
}
```

Using the constructor function new is much cleaner than creating the Product directly.


