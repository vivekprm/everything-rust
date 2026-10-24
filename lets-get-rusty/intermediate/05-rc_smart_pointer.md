# RC Smart Pointer
In the following example we have three structs, a `Database` struct and two services `AuthService` & `ContentService`. Both services need
to perform database operations so they each have a `db` field which is of type `Database`.

Let's create instances of these in main.

```rust
struct Database {}

struct AuthService {
    db: Database
}

struct ContentService {
    db: Database
}

fn main() {
    let db = Database{};
    let auth_service = AuthService { db: db };
    let content_service = ContentService { db: db }; // error
}
```

We get an error while defining ContentService instance, stating "use of moved value `db`". When we passed db into AuthService instance
we are transferring ownership from db into the db field in `AuthService`.

Remember that the **Rust Ownership** rule states that there could only be one owner of a value at a given time. However, in some
situations, we may want to use Shared Ownership. E.g. in this case we want `AuthService` and `ContentService` to both share ownership
of the same database instance. **Rust allows us to do that with RC Smart Pointers**, which stands for **Reference Counting**.

Unlike the **Box Smart Pointer** the **RC Smart Pointer** is not included in the Rust prelude. So we have to import it.

```rust
use std:rc:Rc;

struct Database {}

struct AuthService {
    db: Rc<Database>
}

struct ContentService {
    db: Rc<Database>
}

fn main() {
    let db = Rc::new(Database{});
    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = ContentService { db: Rc::clone(&db) };
}
```

`clone` function on `Rc` will not actually clone our database instance instead it will simply increment the reference count. When we create 
an instance of db reference count is 1, then when we create auth_service instance, reference count is incremented to 2 and when we create
instance of content_service, reference count becomes 3. At the end of the main function all three variables will be dropped and reference
count becomes 0 and Rc pointer will drop the the value it's holding.

`Rc Smart Pointer` is similar to `Shared Smart Pointer` in C++.

One last thing to know about `Rc Smart Pointer` is that it can only be used in **Single Threaded Applications**. For multi-threaded applications
you must use the **atomically reference counted smart pointer** which will discuss later.
