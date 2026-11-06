# Unsafe Basics
There are two versions of Rust:
- Safe Rust
- Unsafe Rust

`Safe Rust` is the default. The Rust we know and love where memory safety is checked at compile time. The problem with this is that memory safety
is checked with static analysis which is conservative by nature. This means valid code might be rejected by the compiler if memory safety can't
be proven. There are also things that you might want to do which are inherently unsafe such as directly interacting with hardware or operating
system. For these situations we have unsafe Rust which allow us to do several things that are not checked for memory safety at compile time.

It is important to know that unsafe Rust code is not inherently bad, however because memory safety can't be checked at compile team the developers
must be thorough in making sure that unsafe code is memory safe by auditing the code. 

It's also important to know that when writing unsafe code Rust still provides some level of safety e.g. if you use references in unsafe code
they will still be checked by the borrow checker.

To write unsafe code in Rust we must use the `unsafe` keyword. e.g. below we have created unsafe code block:

```rust
fn main() {
    unsafe {
        // 1. Dereference a raw pointer.
        // 2. Call an unsafe function.
        // 3. Implement an unsafe trait.
        // 4. Access/modify a mutable static variable
        // 5. Access fields of a union
    }
}
```

In this unsafe code block we can do 5 things that we can't do in safe Rust.
- Dereference a raw pointer
- Call an unsafe function
- Implement an unsafe trait
- Access/modify a mutable static variable
- Access fields of a union

# Dereferncing a Raw Pointer
`Raw pointers` are different from references and smart pointers because they are not bound by the borrowing rules. This means `raw pointer`
allow you to have immutable and mutable borrows to the same memory location at the same time.

`Raw pointers` also allow us to have multiple mutable borrows of the same memory location at the same time. In addition, `raw pointer` could
be null or point to invalid memory and they are not automatically cleaned up. We can create `raw pointers` by casting references or casting
arbitrary memory addresses.

First let's create two raw pointers by casting references which point to the heap allocated string `s`.

```rust
fn main() {
    let mut s = "Let's Get Rusty".to_owned();

    let raw1 = &s as *const String;
    let raw2 = &mut s as *mut String;
}
```
Here we have created two raw pointers:
- `raw1` an immutable raw pointer from an immutable reference.
- `raw2` a mutable raw pointer from mutable reference.

`immutable raw pointers` are denoted as `*const` followed by the type they point to. `immutable` in this case means that the pointer can't be
assigned after it has been dereferenced.
`mutable raw pointers` are denoted with `*mut` followed by the type they point to. `as` keyword is used for casting in this case we are casting
references to raw pointers.

Because we are casting from references, we know that these raw pointers point to valid memory.

Next let's create a raw pointer from an arbitrary memory address.

```rust
fn main() {
    let mut s = "Let's Get Rusty".to_owned();

    let raw1 = &s as *const String;
    let raw2 = &mut s as *mut String;

    let address = 0x012345usize;
    let raw3 = address as *const String;
}
```

Notice that creating raw pointer even if it points to invalid memory is safe. However, it's unsafe to dereference a Raw Pointer.
In order to dereference the smart pointers we need to create an unsafe block.

```rust
fn main() {
    let mut s = "Let's Get Rusty".to_owned();

    let raw1 = &s as *const String;
    let raw2 = &mut s as *mut String;

    let address = 0x012345usize;
    let raw3 = address as *const String;

    unsafe {
        (*raw2).push_str("!!!");
        println!("raw1 is: {}", *raw1);
    }
}
```

If we run this as expected "Let's Get Rusty!!!" is printed out.

`Raw Pointers` are useful for several reasons. e.g. to achieve greater performance for the ability to interface with other languages such as 
C. For the ability to build safe abstractions that the borrow checker doesn't understand and for the ability to interface with hardware.

# Calling an unsafe function
In below example, we have a function called `my_function`, which we are calling in `main`.

```rust
fn main() {
    my_function();
}

fn my_function() {
    println!("calling my function!");
}
```

We can make it unsafe by prefixing it with `unsafe` keyword.

```rust
fn main() {
    unsafe {
        my_function();
    }
}

unsafe fn my_function() {
    println!("calling my function!");
}
```

Marking the function `unsafe` means the function has invariance we need to uphold, when calling the function. As `invariant` is a rule or
requirement that can not be enforced by the compiler or the type system, so must be enforced or respected by the developer.

By calling `my_function`, we are saying that we read the documentation and are upholding the function's invariance. Specifically, when calling
an unsafe function, we need to be careful about the arguments passed in. 

One thing to note is that the body of the unsafe function is treated like an unsafe block. So you are allowed to perform unsafe operations.
However, a proposal has been accepted to change this behaviour in the future.

`Methods` can also be marked unsafe. One example in Rust standard library is `split_at_unchecked` method on `slices`.

# Implementing an Unsafe Trait
In below example, we have a trait defined `MyTrait` with one method called `some_function`. The trait is implemented on the `String` type. 
Which means we can call `some_function` on `String` instances.

```rust
unsafe trait MyTrait {
    fn some_function(&self);
}

unsafe impl MyTrait for String {
    fn some_function(&self) {
        // ...
    }
}

fn main() {
    let s = "some string".to_owned();
    s.some_function();
}
```

Let's make `MyTrait` as unsafe. When a `Trait` is marked unsafe, we also need to mark all implementations as `unsafe`.

A Trait is marked `unsafe` to warn implementers that certain invariance must be respected or memory safety will be violated. By implementing
an unsafe trait for a given type we are promising to uphold those invariance. You won't see unsafe trait often, however one example is 
`GlobalAlloc` trait.

`GlobalAlloc` allows implemnters to define a memory allocator that could be registered with standard library. When implementing `GlobalAlloc`
you must adhere to certain contracts defined in the safety section of the documentation.

# Mutable Static Variables
In below example we have a mutable static variable defined called `COUNTER` which is `u32` integer set to `0`. Accessing immutable variable
is safe, however, accessing/modifying a mutable static variable is unsafe. This is because if two threads are accessing the same mutable
global variable it could lead to a data race. Let's write an `increment_counter` function which increments the `COUNTER` using an unsafe
block.

```rust
static mut COUNTER: u32 = 0;

fn increment_counter() {
    unsafe {
        COUNTER += 1;
    }
}

fn main() {
    for _ in 0..10 {
        increment_counter();
    }

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

The general best practice is, you should keep unsafe block small and wrap them in a safe API. e.g. in our case unsafe increment counter is
wrapped in unsafe block in `increment_counter` function which could be called from regular rust code. Because unsafe Rust code is annotated
with `unsafe` keyword, if your program is experiencing the memory safety issues, you will know to look at `unsafe` blocks. 

# Inline Assembly
Ussafe Rust allows us to write inline assembly.

```rust
fn add(x: u64, y: u64) -> u64 {
    todo!()
}

fn main() {}
```

For example, here we have a function called `add` which takes two integers and returns the sum. Let's implement this function using inline
assembly. First we need to import the `assembly macro`. The `assembly macro` can only be used in `unsafe` code.

```rust
use std::arch::asm;

fn add(x: u64, y: u64) -> u64 {
    let result: u64;

    unsafe {
        // x86/x86-64 assembly
        asm!("add {0} {1}", inout(reg) x => result, in(reg) y);
    }

    result
}

fn main() {}
```

Above assmebly is for x86 architecture so might not work everywhere.

Inline assembly can increase performance, however it decreases portability and stability. In most cases the assembly the compiler generates
is better than the assembly we can write by hand. However, this option is available for special usecases.

# FFI C from Rust
Using unsafe Rust, we have ability to call external code via FFI (Foreign Function Interface). A FFI allows us to define function which could
be called from other programming languages. In this example we will be calling C code from Rust.

Let's create a binary project and create a folder to host c files. Add a file `adder.c` with below code. `adder` is a library containing one
function called `add` which adds two integers.

```c
int add(int a, int b) {
    return a + b;
}
```

Let's switch over to `main.rs` and see how we can call this function from Rust. First we will add `extern` block with the signature of `add` 
function. The `extern` keyword is used to declare function interfaces of external code which Rust can call.

Note that the Rust compiler can't check that the types we give these arguments are correct. Therefore, it's our job as developers to make
sure that the argument types are correct. There are crates such as `cbindgen` or `cxx` that generate these interfaces automatically but as 
of now we will do it manually. String "C" after extern defines which **application binary interface** or ABI the external function uses.

An **ABI** is similar to an **API** except instead of defining an interface between code, it defines interface between binary program.
The `C` ABI is the most common ABI.

```rust
#[link(name = "adder", kind = "static")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    let x: i32;

    unsafe {
        x = add(1, 2);
    }

    println!("x is: {x}");
}
```

Note that, calling foreign function is always unsafe because Rust can't enforce that the other language upholds it's memory safety guarantees.

Before we run our program, we need to tell Rust to link against our `adder` library, so that the `fn add(a: i32, b: i32) -> i32` symbol is
resolved. We can do that using `link` attribute. Above we are saying to link against library `adder` which is a static library.

Let's run our code. First we need to to turn our c code into c static library using `clang` or `gcc`.

```sh
clang -c adder/adder.c -o adder/adder.o
```

Then we will use `ar` utility to turn the object file into a library called `libadder.a`.

```sh
ar -rc ./adder/libadder.a ./adder/adder.o
```

Finally we will set the Rust flags environment variable to link against our library and then run our Rust program.

```sh
RUSTFLAGS='-L ./adder' cargo run
```

# FFI Rust from C
In this example, we are going to call `Rust` code from `C`. Let's create a Rust project with one library crate, containing one public function
called `add`.

```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

In the source directory, we also have a c file called, `call_rust.c`.

```c
#include <stdio.h>

extern int add(int a, int b);

int main() {
    int x = add(1, 2);
    printf("x is: %d", x);
    return 0;
}
```

This `C` program links against the external function called `add`.

In order to make Rust function available to `C` code, we need to add `extern` keyword and specify the ABI to use.
We also need to annotate our function with `no_mangle`.

`no_mangle` will prevent the compiler from changing the name of our function during compilation. Finally we need to open the `Cargo.toml`
and change the type of our library crate to `cdylib`. This tells cargo to compile our code as a shared library which our `c` program
can call. Note that we can also specify static library but that makes linking a little bit more difficult.

```toml
[package]
...

[lib]
crate-type = ['cdylib']
```

First build our code with `cargo build`. Then we will use `clang` to build our `C` code and link against our Rust library.

```sh
clang src/call_rust.c -o call_rust -lrust_from_c -L ./target/debug
```

This will generate an executable called `call_rust`, linking against a library called `rust_from_c` located in `./target/debug` directory.

# FFI Rust From Python
We need python3 installed. Let's create a project directory called `word_counter`.

```sh
mkdir word_counter
cd word_conter
```

Create a new python environment.

```sh
python -m venv .env
source .env/bin/activate
```

Now that we have our python enviroments is setup, we will need a tool called `maturin` which will help us publish our Rust binaries as python
packages.

```sh
pip install maturin
```

Next we need to run `maturin init` and select `pyo3`. `pyo3` is a Rust library which provides Rust bindings for python. After running
`maturin init`, we have Rust project with python bindings.

We can see some of the code is already written in `src/lib.rs`. With this default code, we have defined one module called `word_counter`
that has one function called `sum_as_string`. Let's try to use this module in our python code. First we will create a new direcotry in the
root of our project called `examples`. Inside this `examples` directory create a file called `example.py`.

Inside this python file will import `word_counter` and then call `sum_as_string`.

```python
import word_counter

result = word_counter.sum_as_string(1, 2)
print(result)
```

Next let's open our terminal and call `maturin develop` which will build our Rust code into a `python library` and install it in our virtual
python environment, we have setup. Now let's run the `example.py` program. It should print 3.

Now that we know our infrastructure is working. Let's change the function `sum_as_string` to `count_words`. Then instead of taking in two 
arguments, take one argument of type string.

```rust
use pyo3::prelude::*;

#[pyfunction]
fn count_words(s: String) -> Py<PyAny> {
    todo!()
}
```

Instead of `PyResult<String>`, we will return `Py<PyAny>`. `Py` represents a reference to an object allocated on python's heap and `PyAny`
represents any python object. In this case we want to return a python dictionary containing the words in the string `s` and their counts.

To do that we will start with creating a `HashMap`.


```rust
use std::collections::HashMap;
use pyo3::prelude::*;

#[pyfunction]
fn count_words(s: String) -> Py<PyAny> {
    let mut hm = HashMap::new();
    for sub_str in s.split(' ') {
        let count = hm.entry(sub_str)
            .or_insert(0);
        *count += 1;
    }

    return Python::with_gil(|py| {
        hm.to_object(py)
    });
}
```

`with_gil` acquires python's **global interpreter lock** which we need to convert the hashmap into a python object. Let's use it in our
python code. But first call `maturin develop` again.
