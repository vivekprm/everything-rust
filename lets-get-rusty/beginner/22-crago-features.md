# Cargo Features

We have a Cargo project called Draw with a single library crate.

This single library has one public top level function called draw_line. It also has two modules color & shapes.

[package]
name = "draw"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
rgb = { version = "0.8.25", features = ["serde"] }

[features]
color = []
shapes = []

Here we use `serde` to be able to serialize and deserialize the Rectangle struct. Now this library is very small.
But imagine it was lot bigger, we don't want consumers of the library to pay for the behaviour that they don't use.
This is where **Cargo Features** come in handy. **Cargo Features** allow us to do two things:
- Allow us to define part of our code that are conditionally compiled only if a certail feature is turned on.
- Allow us to define optional dependency. 

The advantage of using features is that they reduce compile time and file sizes. To add features to our Cargo project,
we can open up `Cargo.toml` and add a feature section as below:

```toml
[package]
name = "draw"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
rgb = { version = "0.8.25", features = ["serde"] }

[features]
color = []
shapes = []
```

color and shapes are two features, each feature has a name and an associative array where we can specify other features or 
optional dependencies that should be enabled.

E.g. we can say that if the `shapes` feature is enabled then the `color` feature will also be enabled as below:

```toml
[package]
name = "draw"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
rgb = { version = "0.8.25", features = ["serde"] }

[features]
color = []
shapes = ["color"]
```

For features to enable optional dependencies first we have to make our dependencies optional by marking them as such. Lets make
`serde` and `rgb` optional dependencies. Then we can say if color feature is enabled optional RGB dependency should also be enabled.
We'll also enable the serde feature in the RGB dependency. To enable a feature inside a dependency, we specify dependency followed by /
and then the feature name, in this case `rgb?/serde`.

? syntax means only enable the `serde` feature inside the `rgb` dependency if the rgb dependency is already enabled by something else.
In this case the color feature.


```toml
[package]
name = "draw"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"], optional = true }
rgb = { version = "0.8.25", features = ["serde"], optional = true }

[features]
color = ["dep:rgb"]
shapes = ["color", "dep:serde", "rgb?/serde"]
```

Notice that to enabled optional dependencies we we `dep:` syntax. 
So `rgb` will only compile if `color` feature is enabled and `serde` will only compile when `shapes` feature is enabled.

**Features** are disabled by default. However, we can change this by adding a special feature called default.

```toml
[package]
name = "draw"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"], optional = true }
rgb = { version = "0.8.25", features = ["serde"], optional = true }

[features]
default = ["color"]
color = ["dep:rgb"]
shapes = ["color", "dep:serde", "rgb?/serde"]
```

Here we are saying all the feautres are disabled except the `color` feature.

We have seen how to use features to conditionally compile optional dependencies. Now let's see how we can use features
to conditionally include code at compile time.

First we will make it such that the color module is only included if the color feature is enabled.

```rust
pub fn draw_line(x: i32, y: i32) {
    // draw line without color
}

#[cfg(feature = "color")]
pub mod color {
    pub use rgb::RGB;

    pub fn draw_line(x: i32, y: i32, color: &RGB<u16>) {
        println!("{color}");
        // draw line with color
    }
}

#[cfg(feature = "shapes")]
pub mod shapes {
    use serde::{Serialize, Deserialize};
    use rgb::RGB;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rectangle{
        pub color: RGB<u16>,
        pub width: u32,
        pub height: u32,
    }
}
```

Notice that we get the error in shapes module saying that "serde is an unresolved import". This is because we made serde an
optional dependency and it's only enabled if the shapes feature is enabled and the `shapes` feature is disabled by default.
To fix this let's include the shapes module only if shapes feature is enabled.

Now let's see how consumer of this library could use the features we have defined. Will make a new project called `draw_consumer`
with one binary crate. First let's add our draw library as a dependency:

```toml
[package]
name = "draw_consumer"
version = "0.1.0"
edition = 2021

[dependencies]
draw = { path = "../draw" }
```

Then we can use draw library in `main.rs`. Notice that we have only access to the top level `draw_line` function and the color
module. This is because the top level `draw_line` function is not gated behind any features and the `color` module is gated behind
the `color` feature, which is on by default.

```rust
use draw::color;

fn main() {
    draw::draw_line(32, 32);

    let color = color::RGB {
        r: 247,
        g: 76,
        b: 0,
    };

    color::draw_line(32, 32, &color);
}
```

Now our code is working let's go back to `Cargo.toml`. As a consumer of draw library we can disable color feature which is enabled
by default. Specifically we can disable all default features by setting default features to false.

```toml
[package]
name = "draw_consumer"
version = "0.1.0"
edition = 2021

[dependencies]
draw = { path = "../draw", default_features = false }
```

Now we see that we get compilation error "unresolved import `draw::color`". To fix this let's manually enable the color feature. To
enable feature for given dependency we simply add a features field with an array of feature names.

```toml
[package]
name = "draw_consumer"
version = "0.1.0"
edition = 2021

[dependencies]
draw = { path = "../draw", default_features = false, features = ["shapes"] }
```

We can also change it to `shapes` which will also give access to `shapes` and resolve `color` as well.

Let's go ahead and create a shape in main.

```rust
use draw::color;

fn main() {
    draw::draw_line(32, 32);

    let color = color::RGB {
        r: 247,
        g: 76,
        b: 0,
    };

    color::draw_line(32, 32, &color);

    let square = draw::shapes::Rectangle {
        color,
        width: 32,
        height: 32,
    }

    println!("{square:?}");
}
```
