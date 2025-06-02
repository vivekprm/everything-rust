# Installing rustup on Linux or macOS
If you’re using Linux or macOS, open a terminal and enter the following command:

```sh
curl https://sh.rustup.rs -sSf | sh
```

The command downloads a script and starts the installation of the rustup tool, which installs the latest stable version of Rust.

If you prefer, feel free to download the script and inspect it before running it.

The installation script automatically adds Rust to your system PATH after your next login. If you want to start using Rust right 
away instead of restarting your terminal, run the following command in your shell to add Rust to your system PATH manually:
```sh
source $HOME/.cargo/env
```

Alternatively, you can add the following line to your ```~/.bash_profile```:
```sh
export PATH="$HOME/.cargo/bin:$PATH"
```

Additionally, you’ll need a linker of some kind. It’s likely one is already installed, but when you try to compile a Rust program and 
get errors indicating that a linker could not execute, that means a linker isn’t installed on your system and you’ll need to install 
one manually. C compilers usually come with the correct linker. Check your platform’s documentation for how to install a C compiler. 
Also, some common Rust packages depend on C code and will need a C compiler. Therefore, it might be worth installing one now.

# Updating and Uninstalling
After you’ve installed Rust via rustup , updating to the latest version is easy. From your shell, run the following update script:

```sh
rustup update
```

To uninstall Rust and rustup , run the following uninstall script from your shell:
```sh
rustup self uninstall
```

# Troubleshooting
To check whether you have Rust installed correctly, open a shell and enter this line:
```sh
rustc --version
```

For any kind of help #rust IRC channel on irc.mozilla.org, which you can access through Mibbit
at http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust/. 
At that address you can chat with other Rustaceans. Other great resources include the Users forum at 
https://users.rust-lang.org/ and Stack Overflow at http://stackoverflow.com/questions/tagged/rust/.

# Local documentation
The installer also includes a copy of the documentation locally, so you can read it offline. Run ```rustup doc``` to open the local 
documentation in your browser.

```println!``` calls a Rust macro. If it called a function instead, it would be entered as ```println``` (without the ! ).

Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can
run it even without having Rust installed.

```sh
rustc main.rs
```

Just compiling with ```rustc``` is fine for simple program but as your project grows, you'll want to manage all the options and make it 
easy to share your code. Cargo is tool that does exactly that.

# Hello Cargo
Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles 
a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. 
(We call libraries your code needs dependencies.)

## Creating a Project with Cargo
Let’s create a new project using Cargo and look at how it differs from our original Hello, world! project. Navigate back to your 
projects directory (or wherever you decided to store your code). Then, on any operating system, run the following:
```sh 
cargo new hello_cargo --bin
cd hello_cargo
```

The first command creates a new binary executable called hello_cargo. The ```--bin``` argument passed to cargo new makes an executable 
application (often just called a binary) as opposed to a library. We’ve named our project hello_cargo, and Cargo creates its files in 
a directory of the same name.

Go into the hello_cargo directory and list the files. You’ll see that Cargo has generated two files and one directory for us: a 
```Cargo.toml``` file and a src directory with a ```main.rs``` file inside. It has also initialized a new Git repository
along with a ```.gitignore``` file.

Cargo also provides a command called ```cargo check```. This command quickly checks your code to make sure it compiles but doesn’t 
produce an executable.

Why would you not want an executable? Often, cargo check is much faster than cargo build , because it skips the step of producing an 
executable. If you’re continually checking your work while writing the code, using ```cargo check``` will speed up the process!

## Building for Release
When your project is finally ready for release, you can use ```cargo build --release``` to compile it with optimizations.
This command will create an executable in target/release instead of target/debug. The optimizations make your Rust code run faster, but 
turning them on lengthens the time it takes for your program to compile. 

This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building 
the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. 
If you’re bench-marking your code’s running time, be sure to run ```cargo build --release``` and benchmark with the executable in 
target/release.

For more information about Cargo, check out its documentation at https://doc.rust-lang.org/cargo/.

# Guessing Game
Let's setup a new project.

```sh
cargo new guessing_game --bin
```

The first part of the guessing game program will aks for user input, process that input and check that the input is in the expected
form. To start we will allow the player to input a guess.

```rs
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {}", guess);
}
```

To obtain user input and then print the result as output, we need to bring the ```io``` library into scope. The ```io``` library comes 
from the standard library (which is known as ```std```).

```rust
use std::io
```

By default, Rust brings only a few types into the scope of every program in the *prelude*. If a type you want to use isn’t in the 
prelude, you have to bring that type into scope explicitly with a use statement. Using the ```std::io``` library provides you with a 
number of useful features, including the ability to accept user input.

## Storing Values with Variables
Next, we’ll create a place to store the user input, like this: 
```sh
let mut guess = String::new();
```

Notice that this is a let statement, which is used to create a variable. Here’s another example:
```sh
let foo = bar;
```

This line creates a new variable named ```foo``` and binds it to the value ```bar```.
In Rust, variables are immutable by default. The following example shows how to use ```mut``` before the variable name to make a 
variable mutable:

```rust
let foo = 5; // immutable
let mut bar = 5; // mutable
```

Let’s return to the guessing game program. You now know that let mut guess will introduce a mutable variable named guess . On the 
other side of the equal sign ( = ) is the value that guess is bound to, which is the result of calling ```String::new```, a function 
that returns a new instance of a String. String is a string type provided by the standard library that is a growable, UTF-8
encoded bit of text.

The :: syntax in the ::new line indicates that **```new``` is an associated function of the String type. **An associated function is 
implemented on a type, in this case String, rather than on a particular instance of a String**. Some languages call this a static method.

This new function creates a new, empty string. You’ll find a new function on many types, because it’s a common name for a function that 
makes a new value of some kind.

To summarize, the ```let mut guess = String::new();``` line has created a mutable variable that is currently bound to a new, empty 
instance of a String.

Recall that we included the input/output functionality from the standard library with use ```std::io;``` on the first line of the 
program. Now we’ll call an associated function, stdin, on io:
```sh 
io::stdin().read_line(&mut guess)
 .expect("Failed to read line");
```

If we hadn’t listed the use ```std::io``` line at the beginning of the program, we could have written this function call as 
```std::io::stdin```. The stdin function returns an instance of ```std::io::Stdin```, which is a type that represents a handle to the 
standard input for your terminal.

The next part of the code, ```.read_line(&mut guess)```, calls the read_line method on the standard input handle to get input from the 
user. We’re also passing one argument to ```read_line : &mut guess```.

The job of read_line is to take whatever the user types into standard input and place that into a string, so it takes that string as 
an argument.
The string argument needs to be mutable so the method can change the string’s content by adding the user input.

The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data 
without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is 
how safe and easy it is to use references. You don’t need to know a lot of those details to finish this program.

For now, all you need to know is that **like variables, references are immutable by default**. Hence, you need to write &mut guess 
rather than &guess to make it mutable.

## Handling Potential Failure with the Result type
The second part is:

```sh
.expect("Failed to read line");
```

When you call a method with the ```.foo()``` syntax, it's often wise to introduce a newline and other whitespace to help break up long lines.
We could have written this code as:
```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

However, one long line is difficult to read, so it’s best to divide it: two lines for two method calls. Now let’s discuss what this line does.

As mentioned earlier, read_line puts what the user types into the string we’re passing it, but it also returns a value—in this case, an 
```io::Result```. Rust has a number of types named Result in its standard library: a generic Result as well as specific versions for 
submodules, such as ```io::Result```.

The Result types are enumerations, often referred to as enums. An enumeration is a type that can have a fixed set of values, and those 
values are called the enum’s variants.
For Result, the variants are ```Ok``` or ```Err```. The Ok variant indicates the operation was successful, and inside Ok is the 
successfully generated value. The Err variant means the operation failed, and Err contains information about how or why the operation 
failed.

The purpose of these ```Result``` types is to encode error-handling information. Values of the Result type, like values of any type, 
have methods defined on them. An instance of ```io::Result``` has an expect method that you can call.
If this instance of ```io::Result``` is an Err value, expect will cause the program to crash and display the message that you passed 
as an argument to expect. If the read_line method returns an Err, it would likely be the result of an errorcoming from the underlying 
operating system. 

If this instance of ```io::Result``` is an Ok value, expect will take the return value that Ok is holding and 
return just that value to you so you can use it. In this case, that value is the number of bytes in what the user entered into 
standard input. If you don’t call expect, the program will compile, but you’ll get a warning:

```sh
cargo build
Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `std::result::Result` which must be used
--> src/main.rs:10:5
|
10 |
io::stdin().read_line(&mut guess);
|
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
|
= note: #[warn(unused_must_use)] on by default
```

Rust warns that you haven’t used the Result value returned from read_line, indicating that the program hasn’t handled a possible error.
The right way to suppress the warning is to actually write error handling, but because you just want to crash this program when a 
problem occurs, you can use expect.

## Printing Values with println! Placeholders
Aside from the closing curly brackets, there’s only one more line to discuss in the code added so far, which is the following:
```rust
println!("You guessed: {}", guess);
```

This line prints the string we saved the user’s input in. The set of curly brackets, {} , is a placeholder: think of {} as little crab 
pincers that hold a value in place. You can print more than one value using curly brackets: the first set of curly brackets holds the 
first value listed after the format string, the second set holds the second value, and so on. Printing multiple values in one call to 
println! would look like this:

```rust
let x = 5;
let y = 10;
println!("x = {} and y = {}", x, y);
```

This code would print x = 5 and y = 10 .

# Generating a Secret number

