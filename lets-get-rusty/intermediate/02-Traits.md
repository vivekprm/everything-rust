# Traits
Let's look at the following program:

```rust
struct Car {
    make: String,
    model: String,
    year: u16
}

struct Truck {
    make: String,
    model: String,
    year: u16
}

impl Truck {
    fn unload(&self) {
        println!("unloading truck.");
    }
}
```

Let's say we want to add more functionalities to Car and Truck e.g. ability to park. We could add park implementation
method for Truck as well as for Car. However, this is not ideal for two reasons:
- It introduces code duplication
- We want the interface of the Park method to be exactly the same for Car and Truck. So we can take advantage of **polymorphsim**.

**Polymorphism** allow us to call methods on an interface without worrying about the concrete types that implement that interface.

If you are coming from other OOPS languages, you might say in this case use **inheritance**. We can create another struct called,
**Vehicle** which also has make, model, year fields and a Park method and then have Car and Truck inherit from Vehicle. However,
**Rust doesn't support classical inheritance**. 

Instead in Rust, in order to share functionality and provide a common iterface we use **Traits**. Which are similar to interfaces
in Java.

Let's create our first Trait called Park, which will define a Park method. To create a **Trait** in Rust, we use the `trait` keyword.

```rust
trait Park {
    fn Park(&self);
}
```

Notice that our method doesn't have a body. We are simply declaring interface that needs to be implemented. Now that our trait is
defined we can implement our trait for Car and Truck. To do so we will use an impl block.

```rust
impl Park for Car {
    fn park(&self) {
        println!("parking car!");
    }   
}
```

Then we can do the same for truck:

```rust
impl Park for Truck {
    fn park(&self) {
        println!("parking truck!");
    }
}
```

Now Car and Truck share a similar interface.

One thing to note here is as in case of Classical Inheritance where data and functionality can be shared, with Traits only
functionality can be shared.

We could have improved the way data is modeled in `Car` and `Truck` by extracting the three fields make, model & year out into
their own struct. We can do that by creating a struct called `VehicleInfo`.

```rust
struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}

struct Car {
    info: VehicleInfo,
}

struct Truck {
    info: VehicleInfo,
}
```

Lastly let's implement another Trait called `Paint`, which will have one method `paint`, that allows our Car and Truck to be painted
in different color.

```rust
trait Paint {
    fn paint(&self, color: String) {
        println!("painting object: {}", color);
    }
}
```

Notice that this time our method does have a body, this is known as a default implementation. **Default Implementations** allow us to
provide some default behaviour that could be overwritten.

Lets implement `paint` for Car and Truck.

```rust
impl Paint for Car {}
impl Paint for Truck {}
```

Notice that this time eventhough our impl block is empty, we don't get any errors and that's because only method that needs to be
implemented has a default implementation.

The `Paint` trait could be implemented to many different types of objects. Park makes sense for Vehicle. E.g. lets make another
struct called house and implement Paint trait for it.

```rust
struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting house: {}", color);
    }
}
```

# Trait Bound
Let's add a function called `paint_red`.

```rust
fn main() {}

fn paint_red<T>(object: T) {
    object.paint("red".to_owned());
}
```

We get an error saying "No method named `paint` found for reference &T in the current scope". This makes sense because T could
be any concrete type. So we don't know if `object` implements the `paint` method. What we really want to say is `object` can be
any type as long as that type implements `Paint` trait. We can do that by introducing a **TraitBound**.

There are three ways to specify **TraitBound**:
- First:
```rust
fn paint_red<T: Paint>(object: T) {
    object.paint("red".to_owned());
}

```

- Second:
```rust
fn paint_red(object: &impl Paint) {
    object.paint("red".to_owned());
}
```

- Third:
```rust
fn paint_red<T>(object: &T) where T: Paint {
    object.paint("red".to_owned());
}
```

Using the where clause could be useful if we have multiple trait bounds and you want your function signature to be easy to read.
E.g. let's change this function to `paint_vehicle_red`.

```rust
fn paint_vehicle_red<T>(object: &T) where T: Paint + Park {
    object.paint("red".to_owned());
}
```

Here we are saying T must be any type that implements the both Paint and Park trait.

**TraitBounds** can also be used as return types. E.g. let create a new function called `create_paintable_object`.

```rust
fn create_paintable_object() -> impl Paint {
    House{}
}
```

In this case return type is something that implements `Paint` trait. In this case we are returning an instance of `House`.
Note that it only works if we are returning one concrete type. If we were to return different concrete type based on the
parameter passed in then we need to use a `trait` object which we will explore later.

Now that we have our functions defined let's use them in main:

```rust
fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 1995
        }
    };

    let house = House{};

    let object = create_paintable_object();

    paint_red(&car);
    paint_red(&house);
    paint_red(&object);

    paint_vehicle_red(&car);
    paint_vehicle_red(&house); // error
    paint_vehicle_red(&object); // error
}
```

We don't get any error in paint_red for all the three objects but we get error when passing house and object to paint_vehicle_red
function, becaue house and object don't implement the Park trait.

# Supertraits
In Rust, a **trait** can rely on other **traits** being implemented. The **traits** relied on are called **Supertraits**.

E.g. Let's make `Park` trait more generic by renaming it to `Vehicle`. Then we will make `Paint` a **supertrait** of `Vehicle`.

```rust
trait Vehicle: Paint {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String) {
        println!("painting object: {}", color);
    }
}
```

This simply means that any type implementing the `Vehicle` trait must also implement the `Paint` trait. In this case `Vehicle` is
relying on only one **Supertrait**. However we can add more **Supertraits** if we like using the `+` symbol.

```rust
trait Vehicle: Paint + AnotherTrait {
    fn park(&self);
}
```

Now that the `Paint` is supertrait of `Vehicle`, we can change `paint_vehicle_red` as below:

```rust
fn paint_vehicle_red<T>(object: &T) where T: Vehicle {
    object.paint("red".to_owned())
}
```

Anything that implements `Vehicle` also implements `Paint` which is why we can call paint inside of this function.

**Supertraits** are very useful when you are implementing a trait that relies on functionality from another trait.
So far we have only defined traits with methods meaning functions where first argument is `self`. However, traits 
can also contain associated function. E.g. let's create an associated function on the `Vehicle` trait called
`get_default_color`.

```rust
trait Vehicle: Paint {
    fn park(&self);
    fn get_default_color() -> String {
        "black".to_owned()
    }
}
```

This associated function has default implementation which simply returns the color black.

# Trait Objects
Here we have code from the last lesson:

```rust
trait Vehicle: Paint {
    fn park(&self);
    fn get_default_color() -> String {
        "black".to_owned()
    }
}

trait Paint {
    fn paint(&self, color: String) {
        println!("painting object: {}", color);
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16
}

struct Car {
    info: VehicleInfo
}

impl Vehicle for Car {
    fn park(&self) {
        println!("parking car!");
    }
}

impl Paint for Car {}

struct Truck {
    info: VehicleInfo
}

impl Truck {
    fn unload(&self) {
        println!("unloading truck.")
    }
}

impl Vehicle for Truck {
    fn park(&self) {
        println!("parking truck!");
    }
}

impl Paint for Truck {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting house: {}", color);
    }
}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 1995
        }
    }

    let house = House {};
    let object = create_paintable_object();

    paint_red(&car);
    paint_red(&house);
    paint_red(&object);
    
    paint_vehicle_red(&car);
}

fn paint_red<T: Paint>(object: &T) {
    object.paint("red".to_owned());
}

fn paint_vehicle_red<T>(object: &T) where T: Vehicle {
    object.paint("red".to_owned());
}

fn create_paintable_object() -> impl Paint {
    House{}
}
```

We already mentioned, we can only use **TraitBounds** as a return type if we are returning one concrete type. Let's see what
happens if we try to return different concrete types based on some condition. We will pass new parameter called `vehicle` which
is going to be a `boolean`. Then inside the function body if vehicle is true then return a `Car` otherwise will return a `House`.

```rust
fn create_paintable_object(vehicle: bool) -> impl Paint {
    if vehicle {
        Car {
            info: VehicleInfo {
                make: "Honda".to_owned(),
                model: "Civic".to_owned(),
                year: 1995
            }
        }
    } else {
        House{} // error
    }
}
```

We get "if and else have incompatible type error", there is a suggestion of returning a **Boxed trait object**.

When using a generic as a return type that generic must be substituted with one concrete type at compile time and in this case we have
two concrete types which is why we are getting the error. To fix this error we can follow the suggestion and return a `TraitObject`.

```rust
fn create_paintable_object(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {
        Box::new(Car {
            info: VehicleInfo {
                make: "Honda".to_owned(),
                model: "Civic".to_owned(),
                year: 1995
            }
        })
    } else {
        Box::new(House{})
    }
}

```

**TraitObjects** allow us to define a type which implements a **Trait** without knowing what that type is at compile time.
**TraitObjects** are defined with `dyn` keyword, which stands for **dynamic dispatch** and must be behind some type of pointer.

In this case, we are using **Box pointer**, which points to something allocated on the heap. Let's take a moment to discuss the
difference between **static dispatch** versus **dynamic dispatch**.

# Difference between Static Dispatch versus Dynamic Dispatch
**Static Dispatch** is when the compiler knows which concrete method to call at compile time. E.g. in the previous version of
`create_paintable_object` function when we were returning a generic type `impl Paint`, that generic type would be replaced with
one concrete type at compile time.

Because Generics are substituted with Concrete Types at compile time compiler knows which `paint` method to call at compile time.

The opposite of `static dispatch` is `dynamic dispatch` where compiler can't figure out which concrete method to call at compile
time. So instead it inserts a little bit of code to figure that out at runtime.

E.g. this function now returns a trait object `Box<dyn Paint>`. If we called the `paint` method on the returned object, we don't
know whether we are calling the `paint` method implemented on `Car` or on `House` at compile time. Instead, that would be figured
out at runtime.

The advantage of dynamic dispatch is flexibility e.g. in one case we can return `Car` and in other case we can return `House`.
Disadvantage is runtime performance cost.

Now since we have updated `create_paintable_object` to return trait object let's fix the error in main:

```rust

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 1995
        }
    }

    let house = House {};
    let object = create_paintable_object(true);

    paint_red(&car);
    paint_red(&house);
    paint_red(&object.as_ref());
    
    paint_vehicle_red(&car);
}

fn paint_red<T: Paint>(object: &dyn Paint) {
    object.paint("red".to_owned());
}

```

Also we need to modify the paramter type in `paint_red` now it accepts reference to a type that implements the `Paint` trait.

In summary, Generics with **TraitBounds** can be used if the compiler knows the concrete types that will substitute the Generics
at compile timw. **TraitObjects** are used when the compiler can't tell which concrete types will be used at compile time.

Another situation where **TraitObjects** are used when creating a collection of types that implement a certain Trait.

E.g. Lets create a vector of types that implement `Paint` trait.

```rust

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 1995
        }
    }

    let house = House {};
    let object = create_paintable_object(true);

    let paintable_object = vec![car, house];    // error

    paint_red(&car);
    paint_red(&house);
    paint_red(&object.as_ref());
    
    paint_vehicle_red(&car);
}
```

Inside our vector we have `Car` and `House` and here we get `mismatched types error`. All elements in the Vector must be of the
same type. To fix this let's explicitly mention `paintable_object` is of type that implements the `Paint` trait.

```rust
let paintable_object: Vec<&dyn Paint> = vec![&car, &house];
```

The code now compiles, each element in the vector is a reference to some type which implements the `Paint` trait.

In summary, if compiler can't tell which concrete types are used at compile time, or if you want a collection of different
concrete types that implement the same trait then you can use a **Trait Object**.

# Deriving Traits
In the following program we have one struct called `Point` which represents a point on a Graph. Point has two fields `x` and `y`
which are both integers. In main we have created three points p1, p2 & p3.

Let's go ahead and print out p1 in debug mode:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 3, y: 1 };
    let p2 = Point { x: 3, y: 1 };
    let p3 = Point { x: 5, y: 5 };

    println("{:?}", p1);    // error
}
```

The `:?` syntax is used to print out some type with the debug formatting. However, this only works if the type implements
the `debug` trait. We get error here because `Point` doesn't implement debug trait. We can fix this either by using `derive`
attribute or implementing `debug` manually.

The **derive** attribute lets us implement a trait for a given type by providing us with a basic implementation such that we
don't have to implement the trait manually. Let's use `derive` attribute to implement `Debug` trait for `Point`.


```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 3, y: 1 };
    let p2 = Point { x: 3, y: 1 };
    let p3 = Point { x: 5, y: 5 };

    println!("{:?}", p1);
}
```

`Debug` is a trait implemented in Standard Library and it's brought into scope automatically.

Next let's compare p1 to p2 and p3.


```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 3, y: 1 };
    let p2 = Point { x: 3, y: 1 };
    let p3 = Point { x: 5, y: 5 };

    println!("{:?}", p1);
    println!("{}", p1 == p2); // error
    println!("{}", p1 == p3); // error
}
```

Here we get couple of more errors saying: "binary operation can not be applied to type `Point`", because `PartialEq` trait
has not been implemented for `Point`. Luckily this trait could also be derived.

Not all traits can be derived, only `traits` with an associated `derive macro` can be used in the derive attribute, which we
will look at later.

# The Orphan Rule
In this example, we have created a package called `OrphanRule` with one Library Crate and one Binary Crate. In the Library crate
there is one public struct called `Point` and inside the binary crate, we are bringing `Point` into scope. Inside the binary crate
lets try to implement partial equality `PartialEq` trait for `Point`.

lib.rs
```rust

```

main.rs
```rust
use orphan_rule::Point;

impl PartialEq for Point {  // error
    fn eq(&self, other: &self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
```

Here we get error "only traits defined in the current Crate can be implemented for arbitrary types, define and implement a trait or
new type instead."

The reason we get this error is because we are breaking the **Orphan Rule**. The **Orphan Rule** states that in order to implement a
trait on a given type either the trait or the type must be defined within the current crate. **Without this rule two crates could
implement the same trait on the same type and then Rust would not know which implementation to use**.

In this case, the `PartialEq` is defined in Rust standard library and the `Point` struct is defined in our library crait. So trait
and type not defined within our binary crate. There is however a way to get around this rule by creating a wrapper type.

E.g. Let's create a new struct called `PointWrapper`. 

```rust
struct PointWrapper(Point);
```

`PointWrapper` is a tuple struct which simply wraps the `Point` struct. Because `PointWrapper` is defined within our crate we can
implement the `PartialEq` trait for `PointWrapper`.

```rust
use orphan_rule::Point;

struct PointWrapper(Point);

impl PartialEq for PointWrapper {
    fn eq(&self, other: &self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}
```

Let's use `PointWrapper` inside main.

```rust
fn main() {
    let p1 = PointWrapper(Point{ x: 1, y: 2 });
    let p2 = PointWrapper(Point{ x: 1, y: 2 });

    println!("{}", p1 == p2);
}
```

Then we can run our program using `cargo run`. As expected, result is true.
