# Basic Datatypes
Below are Rust primitive data types:
- `bool` for boolean, 
- unsigned integers such as `u8`, `u16`, `u32`, `u64` & `u128` 
- signed integers such as `i8`, `i16`, `i32`, `i64` & `i128`
- floating point numbers such as `f32` and `f64`
- platform specific integers `usize` represents pointer sized unsigned integer and `isize` represents pointer
sized signed integer.
- `char` for characters.
- `&str` for string slices.
- `String` for string.

```rust
// Scalar Data Types
// unsigned integers
let i1: u8 = 1;
let i2: u16 = 1;
let i3: u32 = 1;
let i4: u64 = 1;
let i5: u128 = 1;

// Signed integers
let i6: i8 = 1;
let i7: i16 = 1;
let i8: i32 = 1;
let i9: i64 = 1;
let i10: i128 = 1;

// Floating point numbers
let f1: f32 = 1.0;
let f2: f64 = 1.0;

// platform specific integers
let p1: usize = 1;
let p2: isize = 1;

// characters, &str, and String
let c1: char = 'c';
let s1: &str = "hello";
let s2: String = String::from("hello");

// Compound data types: Stores multiple values
// arrays
let a1: [i32; 5] = [1, 2, 3, 4, 5];
let i1 = a1[4];

// Tuples
let t1: (i32, i32, i32) = (1, 2, 3);
let t1 = (5, 5.0, "5");

// indexing into the tuple.
let s1: &str = t1.2;
let (i1, f1, s1) = t1
```

All string literals are string slices.

Arrays hold multiple values of same type.

Tuples hold multiple values of different types.

We can index into a tuple by specifying the variable name such as 't1', followed by '.' and then index number.
e.g. t1.2.

We can destructure the tuple using following syntax:

```rust
let (i1, f1, s1) = t1
```

Empty tuple is a special type called **unit**. `unit` types are returned implicitly when no other meaningful
value could be returned.

```rust
let unit = ();
```

E.g. the functions that don't return a value, implicitly return a `unit` type.

# Type Aliasing
Type alias is a new name for an existing type.

```rust
type age = u8;
let a1: age = 57;
```

Type aliases are useful because they make our code easier to understand.


