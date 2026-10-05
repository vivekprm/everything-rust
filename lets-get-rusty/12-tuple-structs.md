# Tuple vs Struct
Lets talk about difference between tuple and struct. Like structs, tuple group together data of different types.

However, a tuple doesn't define a new type:

```rust
fn main() {
    // tuples
    let color1 = (255, 106, 0);
    let color2 = (0, 58, 100, 0);
}
```

In example above color1 is a tuple with 3 integers and color2 is a tuple with 4 integers. However, it's hard to
tell what these numbers mean. So let's change the name to make it more obvious.

```rust
fn main() {
    // tuples
    let rgb_color = (255, 106, 0);
    let cmyk_color = (0, 58, 100, 0);
}
```

So first color represent color encoded in rgb and second color is a color encoded in cmyk. Changing the name of
variable is helpful however this code is still problematic, if we were to pass these values around the name of
the variables could change. Also nothing is enforcing the number of elements in each tuple or the type of the 
elements.

Tuple structs can help us in these situations. Let's redefine both variables as tuple struct.

```rust
fn main() {
    // tuples
    let rgb_color = (255, 106, 0);
    let cmyk_color = (0, 58, 100, 0);

    // tuple structs
    struct RGB(i32, i32, i32);
    struct CMYK(i32, i32, i32, i32);

    let color1 = RGB(255, 106, 0);
    let color2 = CMYK(0, 58, 100, 0);
}
```

Tuple struct are similar to Regular structs, excepts instead of using {} we use () and fields are not named.

Now that the color type are encoded in the type system, we exactly know which color type we are working with
and rust will enforce the type has correct values.

Besides tuple struct there is on other type of struct a unitlike struct.

```rust
struct MyStruct;
```

**Unit Structs** are structs without any fields and are rarely used. One situation in which they are used is when
you have a trait that you need to implement on something but you don't need to store any data inside of it.
