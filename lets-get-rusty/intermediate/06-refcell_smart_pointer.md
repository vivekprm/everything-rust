# Refcell Smart Pointer
Here is the code from previous lesson, let's add a new field to `Database` struct called `max_connections`.

```rust
use std:rc:Rc;

struct Database {
    max_connections: u32
}

struct AuthService {
    db: Rc<Database>
}

struct ContentService {
    db: Rc<Database>
}

fn main() {
    let db = Rc::new(Database{ max_connections: 100 });
    let auth_service = AuthService{ db: Rc::clone(&db) };
    let content_service = ContentService{ db: Rc::clone(&db) };
}
```

Let's say sometimes during our program execution we would like to increases max_connections to our database to handle more load for example.
Let's change max_connections from 100 to 200 after we instantiate auth_service & content_service.

```rust
fn main() {
    let db = Rc::new(Database{ max_connections: 100 });
    let auth_service = AuthService{ db: Rc::clone(&db) };
    let content_service = ContentService{ db: Rc::clone(&db) };
    db.max_connections = 200; // error
}
```

We get an error stating that "can not assign data in an Rc smart pointer", The **Rc Smart Pointer** only allows immutable shared ownership of
a value. Luckily, we can get around this using **RefCell smart pointer**. First just like **Rc Smart Pointer** we need to import the **RefCell
Smart Pointer**.

We need to call `borrow_mut` method for modifying connections. `borrow_mut` allows us to mutably borrow the value wrapped in `RefCell`, in this
case a database instance.

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Database {
    max_connections: u32
}

struct AuthService {
    db: Rc<RefCell<Database>>
}

struct ContentService {
    db: Rc<RefCell<Database>>
}

fn main() {
    let db = Rc::new(RefCell::new(Database{
        max_connections: 100
    }));

    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = ContentService { db: Rc::clone(&db) };
    db.borrow_mut().max_connections = 200;
}
```

`RefCell` uses the **interior mutability design pattern**. This pattern allows us to mutably borrow data even when there is immutable reference
to that data. **This breaks the Rust compile-time borrowing rules**. So in order to implement this pattern unsafe code must be used and then that
unsafe code is wrapped in a safe API.

The advantage of using this pattern is that it allows certain memory-safe scenarios which would otherwise be disallowed.

Rust ownership and borrowing rules are conservative by nature. Sometimes you as a developer know that something is memory safe but can't prove
it to the compiler. This is why patterns like **interior mutability** exist to get around those limitations when needed.

One of the usecases for **RefCell Smart Pointer** is to be used in combination with **Rc Smart Pointer** to get shared ownership and mutability
at the sametime.

One thing to note is that **RefCell** uses unsafe code to get around **Rust Borrowing Rules** at compile time. However, those borrowing rules
should still be enforced at runtime or else your program will panic. E.g. let's run our program in it's current state.

Notice that our program ran successfully. 

Let's see what would happen if we created two mutable references to the `db` at the same time.

```rust
fn main() {
    let db = Rc::new(RefCell::new(Database{
        max_connections: 100
    }));

    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = ContentService { db: Rc::clone(&db) };

    let mut r1 = db.borrow_mut();
    let r2 = db.borrow_mut();

    r1.max_connections = 200;
}
```

Notice that eventhough we have two mutable references to the same value at the sametime, we don't get any compiletime errors.

Let's see what will happen if we run our program again. 

We get a message saying that "thread `main` panicked at already borrowed at line 24."

It panics because we are attempting to get a mutable reference to a value when a mutable reference already exists.

`RefCell` should be used with caution because the responsibility of following the ownership rules is on the programmer.
