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
Next, we need to generate a secret number that the user will try to guess. The secret number should be different every time so the game 
is fun to play more than once. Let’s use a random number between 1 and 100 so the game isn’t too difficult. 

Rust doesn’t yet include random number functionality in its standard library. However, the Rust team does provide a rand
crate at https://crates.io/crates/rand/.

## Using a Crate to Get More functionality
Remember that a crate is a package of Rust code. The project we’ve been building is a binary crate, which is an executable. The rand crate 
is a *library crate*, which contains code intended to be used in other programs.
Cargo’s use of external crates is where it really shines. Before we can write code that uses rand, we need to modify the Cargo.toml file to 
include the ```rand``` crate as a dependency. Open that file now and add the following line to the bottom beneath the [dependencies] section 
header that Cargo created for you:

```toml
[dependencies]
rand = "0.3.14"
```

Cargo understands Semantic Versioning (sometimes called *SemVer*), which is a standard for writing version numbers.
The number 0.3.14 is actually shorthand for ^0.3.14, which means “any version that has a public API compatible with version 0.3.14."

Now, without changing any of the code, let’s build the project

```sh
cargo build
```

You may see different version numbers (but they will all be compatible with the code, thanks to SemVer!), and the lines may be in a 
different order.
Now that we have an external dependency, Cargo fetches the latest versions of everything from the registry, which is a copy of data from 
https://crates.io/. 

Crates.io is where people in the Rust ecosystem post their opensource Rust projects for others to use.

After updating the registry, Cargo checks the [dependencies] section and downloads any crates you don’t have yet. In this case, although 
we only listed rand as a dependency, Cargo also grabbed a copy of *libc*, because rand depends on libc to work. 

After downloading the crates, Rust compiles them and then compiles the project with the dependencies available. 

If you immediately run cargo build again without making any changes, you won’t get any output aside from the *Finished* line. Cargo knows 
it has already downloaded and compiled the dependencies, and you haven’t changed anything about them in your Cargo.toml file. Cargo also 
knows that you haven’t changed anything about your code, so it doesn’t recompile that either. With nothing to do, it simply exits.

If you open the ```src/main.rs``` file, make a trivial change, and then save it and build again, you’ll only see two lines of output:

These lines show Cargo only updates the build with your tiny change to the ```src/main.rs``` file. Your dependencies haven’t changed, so 
Cargo knows it can reuse what it has already downloaded and compiled for those. It just rebuilds your part of the code.

## Ensuring Reproducible Builds with the Cargo.lock File 
Cargo has a mechanism that ensures you can rebuild the same artifact every time you or anyone else builds your code: Cargo will use only 
the versions of the dependencies you specified until you indicate otherwise. For example, what happens if next week version 0.3.15 of the 
rand crate comes out and contains an important bug fix but also contains a regression that will break your code?

The answer to this problem is the ```Cargo.lock``` file, which was created the first time you ran cargo build and is now in your 
guessing_game directory.
When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes 
them to the ```Cargo.lock``` file. When you build your project in the future, Cargo will see that the ```Cargo.lock``` file exists and use 
the versions specified there rather than doing all the work of figuring out versions again. 

This lets you have a reproducible build automatically. In other words, your project will remain at 0.3.14 until you explicitly upgrade, 
thanks to the Cargo.lock file.

## Updating a Crate to Get a New Version
When you do want to update a crate, Cargo provides another command, ```update```, which will ignore the ```Cargo.lock``` file and figure 
out all the latest versions that fit your specifications in ```Cargo.toml```. If that works, Cargo will write those versions to the 
```Cargo.lock``` file.
But by default, Cargo will only look for versions larger than ```0.3.0``` and smaller than ```0.4.0```. If the rand crate has released two 
new versions, 0.3.15 and 0.4.0, you would see the following if you ran cargo update:

```
cargo update
    Updating registry `https://github.com/rust-lang/crates.io-index`
    Updating rand v0.3.14 -> v0.3.15
```

At this point, you would also notice a change in your Cargo.lock file noting that the version of the rand crate you are now using 
is ```0.3.15```. If you wanted to use rand version ```0.4.0``` or any version in the ```0.4.x``` series, you’d have to update the 
```Cargo.toml``` file to look like this instead:

```toml
[dependencies]
rand = "0.4.0"
```

The next time you run cargo build, Cargo will update the registry of crates available and reevaluate your rand requirements according to 
the new version you have specified.

## Generating a Random Number
Now that you’ve added the rand crate to ```Cargo.toml```, let’s start using rand. The next step is to update ```src/main.rs```, as shown 
below.

```rust
extern crate rand; // 1

use rand::Rng; // 2
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(0..101); // 3
    println!("The secret number is: {}", secret_number); // 4

    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {}", guess);
}
```

First, we add a line that lets Rust know we’ll be using the rand crate as an external dependency. This also does the equivalent of calling 
```use rand```, so now we can call anything in the ```rand crate``` by placing ```rand::``` before it.

Next, we add another use line: ```use rand::Rng```. The ```Rng``` trait defines methods that random number generators implement, and this 
trait must be in scope for us to use those methods. 

Also, we’re adding two more lines in the middle. The ```rand::thread_rng``` function will give us the particular random number generator 
that we’recgoing to use: one that is local to the current thread of execution and seeded by the operating system. Next, we call the 
```gen_range``` method on the random number generator. This method is defined by the ```Rng``` trait that we brought into scope with the 
use ```rand::Rng``` statement. The ```gen_range``` method takes two numbers as arguments and generates a random number between them. 
It’s inclusive on the lower bound but exclusive on the upper bound, so we need to specify 1 and 101 to request a number between 1 and 100.

**Note**: *You won’t just know which traits to use and which functions and methods to call from a crate. Instructions for using a crate are 
in each crate’s documentation. Another neat feature of Cargo is that you can run the ```cargo doc --open``` command, which will build 
documentation provided by all of your dependencies locally and open it in your browser. 
If you’re interested in other functionality in the rand crate, for example, run ```cargo doc --open``` and click ```rand``` in the sidebar 
on the left.*

## Comparing the Guess to the Secret Number 
Now that we have user input and a random number, we can compare them.

```rust
extern crate rand;

use rand::Rng; 
use std::cmp::Ordering; // 1
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(0..101); // 3
    println!("The secret number is: {}", secret_number); // 4

    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {}", guess);

    // 2
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```
The first new bit here is another use statement, bringing a type called ```std::cmp::Ordering``` into scope from the standard library. Like 
```Result```, ```Ordering``` is another enum, but the variants for ```Ordering``` are ```Less```, ```Greater```, and ```Equal```.

These are the three outcomes that are possible when you compare two values.

Then we add five new lines at the bottom that use the ```Ordering``` type. The ```cmp``` method compares two values and can be called on 
anything that can be compared. It takes a reference to whatever you want to compare with: here it’s comparing the guess to the 
```secret_number```. Then it returns a variant of the ```Ordering``` enum we brought into scope with the use statement. We use a ```match```
expression to decide what to do next based on which variant of ```Ordering``` was returned from the call to cmp with the values in guess 
and ```secret_number```.

A ```match``` expression is made up of ```arms```. An arm consists of a ```pattern``` and the code that should be run if the value given to 
the beginning of the match expression fits that arm’s pattern. Rust takes the value given to match and looks through each arm’s pattern in 
turn. The match construct and patterns are powerful features in Rust that let you express a variety of situations your code might encounter 
and make sure that you handle them all.

However, the code above won’t compile yet. Let’s try it:

```sh
cargo build

Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
--> src/main.rs:23:21
|
23 | match guess.cmp(&secret_number) {
| ^^^^^^^^^^^^^^ expected struct `std::string::String`,
found integral variable
|
= note: expected type `&std::string::String`
= note: found type `&{integer}`
error: aborting due to previous error
Could not compile `guessing_game`.
```

The core of the error states that there are mismatched types. Rust has a strong, static type system. However, it also has type inference. 
When we wrote ```let guess = String::new()```, Rust was able to infer that guess should be a String and didn’t make us write the type. The 
```secret_number```, on the other hand, is a number type. 

A few number types can have a value between 1 and 100: i32, a 32-bit number; u32, an unsigned 32-bit number; i64, a 64-bit number; as well
as others. Rust defaults to an i32, which is the type of secret_number unless you add type information elsewhere that would cause Rust to 
infer a different numerical type. The reason for the error here is that Rust cannot compare a string and a number type.

Ultimately, we want to convert the String the program reads as input into a real number type so we can compare it numerically to the guess. 
We can do that by adding the following two lines to the main function body:

```rust
let guess: u32 = guess.trim().parse()
.expect("Please type a number!");
```

We create a variable named ```guess```. But wait, doesn’t the program already have a variable named guess? It does, but Rust allows us to 
shadow the previous value of guess with a new one. This feature is often used in situations in which you want to convert a value from one 
type to another type. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as 
guess_str and guess, for example.

We bind guess to the expression ```guess.trim().parse()```. The guess in the expression refers to the original guess that was a String with 
the input in it.
The trim method on a String instance will eliminate any whitespace at the beginning and end. Although u32 can contain only numerical 
characters, the user must press enter to satisfy read_line. When the user presses enter, a newline character is added to the string. 
For example, if the user types 5 and presses enter, guess looks like this: 5\n. The \n represents “newline,” the result of pressing enter. 
The trim method eliminates \n, resulting in just 5.

The ```parse``` method on strings parses a string into some kind of number. Because this method can parse a variety of number types, we 
need to tell Rust the exact number type we want by using ```let guess: u32```. The colon (:) after guess tells Rust we’ll annotate the 
variable’s type. 
Rust has a few built-in number types; the u32 seen here is an unsigned, 32-bit integer. It’s a good default choice for a small positive 
number. 

The call to parse could easily cause an error. If, for example, the string contained ```A<%```, there would be no way to convert that to a 
number. Because it might fail, the parse method returns a ```Result``` type, much as the ```read_line``` method does. We’ll treat this 
Result the same way by using the expect method again. 

If parse returns an ```Err``` Result variant because it couldn’t create a number from the string, the expect call will crash the game and 
print the message we give it. If parse can successfully convert the string to a number, it will return the ```Ok``` variant of Result, and 
expect will return the number that we want from the Ok value.

## Allowing Multiple Guesses with Looping
The ```loop``` keyword creates an infinite loop. We’ll add that now to give users more chances at guessing the number:

```rust
fn main() {
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(0..101); // 3
    println!("The secret number is: {}", secret_number); // 4

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## Handling invalid input
To further refine the game’s behavior, rather than crashing the program when the user inputs a non-number, let’s make the game ignore a 
non-number so the user can continue guessing. We can do that by altering the line where guess is converted from a String to a u32, as 
shown below:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

Switching from an ```expect``` call to a match expression is how you generally move from crashing on an error to handling the error. 
Remember that parse returns a ```Result``` type and ```Result``` is an enum that has the variants ```Ok``` or ```Err```.
We’re using a match expression here, as we did with the ```Ordering``` result of the ```cmp``` method.

If ```parse``` is able to successfully turn the string into a number, it will return an ```Ok``` value that contains the resulting number. 
That ```Ok``` value will match the first arm’s pattern, and the match expression will just return the num value that parse produced and put 
inside the ```Ok``` value. That number will end up right where we want it in the new ```guess``` variable we’re creating.

If ```parse``` is not able to turn the string into a number, it will return an ```Err``` value that contains more information about the 
error. The ```Err``` value does not match the ```Ok(num)``` pattern in the first match arm, but it does match the ```Err(_)``` pattern in 
the second arm. The underscore, _, is a catchall value; in this example, we’re saying we want to match all ```Err``` values, no matter 
what information they have inside them. So the program will execute the second arm’s code, ```continue```, which tells the program to go to 
the next iteration of the loop and ask for another guess. So effectively, the program ignores all errors that parse might encounter!

Let's try and run the program using ```cargo run```.

# Ch2- Variables & Mutability
By default variables are immutable. This is one of many nudges Rust gives you to write your code in a way that takes advantage of the 
safety and easy concurrency that Rust offers. However, you still have the option to make your variables mutable. Let’s explore how
and why Rust encourages you to favor immutability and why sometimes you might want to opt out.

When a variable is immutable, once a value is bound to a name, you can’t change that value. To illustrate this, let’s generate a new 
project called variables in your projects directory by using ```cargo new --bin variables```. Then, in your new variables directory, 
open ```src/main.rs``` and replace its code with the following code that won’t compile just yet:

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

Save and run the program using ```cargo run```. You should receive an error message, as shown in this output:

```
error[E0384]: cannot assign twice to immutable variable `x`
--> src/main.rs:4:5
|
2 | let x = 5;
| - first assignment to `x`
3 | println!("The value of x is: {}", x);
4 | x = 6;
| ^^^^^ cannot assign twice to immutable variable
```

This example shows how the compiler helps you find errors in your programs. Even though compiler errors can be frustrating, they only mean
your program isn’t safely doing what you want it to do yet; they do not mean that you’re not a good programmer! Experienced Rustaceans still
get compiler errors.

The error message indicates that the cause of the error is that you cannot assign twice to immutable variable x, because you tried to assign 
a second value to the immutable x variable.

It’s important that we get compile-time errors when we attempt to change a value that we previously designated as immutable because this 
very situation can lead to bugs. If one part of our code operates on the assumption that a value will never change and another part of our 
code changes that value, it’s possible that the first part of the code won’t do what it was designed to do.

The cause of this kind of bug can be difficult to track down after the fact, especially when the second piece of code changes the value only 
sometimes. In Rust, the compiler guarantees that when you state that a value won’t change, it really won’t change. That means that when 
you’re reading and writing code, you don’t have to keep track of how and where a value might change. Your code is thus easier to reason 
through.

But mutability can be very useful. Variables are immutable only by default; but you can make them mutable by adding ```mut``` in front of 
the variable name. In addition to allowing this value to change, ```mut``` conveys intent to future readers of the code by indicating that 
other parts of the code will be changing this variable value. For example, let’s change ```src/main.rs``` to the following:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

Now we’re allowed to change the value that x binds to from 5 to 6 when ```mut``` is used. In some cases, you’ll want to make a variable 
mutable because it makes the code more convenient to write than if it had only immutable variables.

There are multiple trade-offs to consider in addition to the prevention of bugs. 

For example, in cases where you’re using large data structures, mutating an instance in place may be faster than copying and returning 
newly allocated instances. With smaller data structures, creating new instances and writing in a more functional programming style may be 
easier to think through, so lower performance might be a worthwhile penalty for gaining that clarity.

# Differences Between Variables and Constants
Being unable to change the value of a variable might have reminded you of another programming concept that most other languages have: 
constants. Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few 
differences between constants and variables.

First, you aren’t allowed to use ```mut``` with constants. Constants aren’t just immutable by default—they’re always immutable.
You declare constants using the ```const``` keyword instead of the ```let``` keyword, and the type of the value must be annotated.

```Constants``` can be declared in any scope, including the global scope, which makes them useful for values that many parts of the code 
need to know about.

The last difference is that constants may be set only to a constant expression, not to the result of a function call or any other value 
that could only be computed at runtime.

Here’s an example of a constant declaration where the constant’s name is ```MAX_POINTS``` and its value is set to 100,000. (Rust’s naming 
convention for constants is to use all uppercase with underscores between words):

```rust
const MAX_POINTS: u32 = 100_000;
```

Constants are valid for the entire time a program runs, within the scope they were declared in, making them a useful choice for values
in your application domain that multiple parts of the program might need to know about, such as the maximum number of points any player 
of a game is allowed to earn or the speed of light.

Naming hardcoded values used throughout your program as constants is useful in conveying the meaning of that value to future maintainers 
of the code. It also helps to have only one place in your code you would need to change if the hardcoded value needed to be updated in 
the future.

# Data Types
Every value in Rust is of a certain ```data type```, which tells Rust what kind of data is being specified so it knows how to work with 
that data. We’ll look at two data type subsets: ```scalar``` and ```compound```.

Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time. The 
compiler can usually infer what type we want to use based on the value and how we use it. In cases when many types are possible, such as 
when we converted a String to a numeric type using parse in “Comparing the Guess to the Secret Number”, we must add a type annotation, 
like this:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

If we don’t add the type annotation here, Rust will display the following error, which means the compiler needs more information from us 
to know which type we want to use:

```
error[E0282]: type annotations needed
--> src/main.rs:2:9
|
2 | let guess = "42".parse().expect("Not a number!");
| ^^^^^
| |
| cannot infer type for `_`
| consider giving `guess` a type
```

## Scalar Types
A scalar type represents a single value. Rust has four primary scalar types: ```integers```, ```floating-point numbers```, ```Booleans```, 
and ```characters```. 
You may recognize these from other programming languages. Let’s jump into how they work in Rust.

### Integer Types
An integer is a number without a fractional component. We used one integer type earlier, the u32 type. This type declaration indicates that 
the value it’s associated with should be an unsigned integer (signed integer types start with i, instead of u) that takes up 32 bits of 
space. Below Table shows the built-in integer types in Rust. Each variant in the Signed and Unsigned columns (for example, i16) can be used 
to declare the type of an integer value.

|   Length | Signed  | Unsigned |
|----------|---------|----------|
| 8-bit    |  i8     |   u8     |
| 16-bit   |  i16    |   u16    |
| 32-bit   |  i32    |   u32    |
| 64-bit   |  i64    |   u64    |
| arch     |  isize  |  usize   |

Signed numbers are stored using two’s complement representation. 

Each signed variant can store numbers from −(2<sup>n−1</sup>) to 2<sup>n−1</sup> − 1 inclusive, where n is the number of bits that variant 
uses. So an i8 can store numbers from −(2<sup>7</sup>) to 2<sup>7</sup> − 1, which equals −128 to 127. Unsigned variants can store numbers 
from 0 to 2<sup>n</sup> − 1, so a u8 can store numbers from 0 to 2<sup>8</sup> − 1, which equals 0 to 255.

Additionally, the ```isize``` and ```usize``` types depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit 
architecture and 32 bits if you’re on a 32-bit architecture.

You can write integer literals in any of the forms shown in Table. Note that all number literals except the byte literal allow a type 
suffix, such as 57u8, and _ as a visual separator, such as 1_000.

|  Number Lierals |   Example     |
|-----------------|---------------|
|  Decimal        |   98_222      |
|  Hex            |   0xff        |
|  Octal          |   0o77        |
|  Binary         |   0b1111_0000 |
|  Byte (u8 only) |   b'A'        |

So how do you know which type of integer to use? If you’re unsure, Rust’s defaults are generally good choices, and integer types default to 
i32: this type is generally the fastest, even on 64-bit systems. The primary situation in which you’d use isize or usize is when indexing 
some sort of collection.

### Floating-Point Types
Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. 
Rust’s floating-point types are ```f32``` and ```f64```, which are 32 bits and 64 bits in size, respectively. The default type is ```f64```
because on modern CPUs it’s roughly the same speed as ```f32``` but is capable of more precision.

Here’s an example that shows floating-point numbers in action:

```rust
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard. The f32 type is a single-precision float, and f64 has 
double precision.

### Numeric Operations
Rust supports the basic mathematical operations you’d expect for all of the number types: addition, subtraction, multiplication, division, 
and remainder.
The following code shows how you’d use each one in a let statement:

```rust
fn main() {
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
}
```

Each expression in these statements uses a mathematical operator and evaluates to a single value, which is then bound to a variable.

### The Character Type
So far we’ve worked only with numbers, but Rust supports letters too. Rust’s ```char``` type is the language’s most primitive alphabetic 
type, and the following code shows one way to use it. (Note that the char type is specified with single quotes, as opposed to strings, 
which use double quotes.)

```rust
fn main() {
    let c = 'z';
    let z = 'Ƶ';
    let heart_eyed_cat = '😻 ';
}
```

Rust’s ```char``` type represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; 
Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust. Unicode Scalar Values range 
from ```U+0000``` to ```U+D7FF``` and ```U+E000``` to ```U+10FFFF``` inclusive. However, a “character” isn’t really a concept in 
Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust.

# Compound Types
Compound types can group multiple values into one type. Rust has two primitive compound types: ```tuples``` and ```arrays```.

## The Tuple Type
A tuple is a general way of grouping together some number of other values with a variety of types into one compound type.

We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of 
the different values in the tuple don’t have to be the same. We’ve added optional type annotations in this example:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

The variable tup binds to the entire tuple, because a tuple is considered a single compound element. To get the individual values out of 
a tuple, we can use pattern matching to destructure a tuple value, like this:

```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}
```

This program first creates a tuple and binds it to the variable tup. It then uses a pattern with let to take tup and turn it into three 
separate variables, x, y, and z. This is called *destructuring*, because it breaks the single tuple into three parts. Finally, the program 
prints the value of y, which is 6.4.

In addition to destructuring through pattern matching, we can access a tuple element directly by using a period (.) followed by the index 
of the value we want to access. For example:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

This program creates a tuple, x, and then makes new variables for each element by using their index. As with most programming languages, the
first index in a tuple is 0.

## The Array Type
Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. 
Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length: once declared, they cannot 
grow or shrink in size.

In Rust, the values going into an array are written as a comma-separated list inside square brackets:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

```Arrays``` are useful when you want your data allocated on the stack rather than the heaps or when you want to ensure you always have a 
fixed number of elements.

An array isn’t as flexible as the vector type, though. A ```vector``` is a similar collection type provided by the standard library that is 
allowed to grow or shrink in size. If you’re unsure whether to use an array or a vector, you should probably use a vector.

An example of when you might want to use an array rather than a vector is in a program that needs to know the names of the months of the
year. It’s very unlikely that such a program will need to add or remove months, so you can use an array because you know it will always 
contain 12 items:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
"August", "September", "October", "November", "December"];
```

### Accessing Array Elements
An array is a single chunk of memory allocated on the stack. You can access elements of an array using indexing, like this:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
```

### Invalid Array Element Access
What happens if you try to access an element of an array that is past the end of the array? Say you change the example to the following 
code, which will compile but exit with an error when it runs:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index];
    println!("The value of element is: {}", element);
}
```

Running this code using cargo run produces the following result:

```
cargo run
    Compiling arrays v0.1.0 (file:///projects/arrays)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50 secs
    Running `target/debug/arrays`
    thread '<main>' panicked at 'index out of bounds: the len is 5 but the index
    is 10', src/main.rs:6
    note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

The compilation didn’t produce any errors, but the program resulted in a runtime error and didn’t exit successfully. When you attempt to 
access an element using indexing, Rust will check that the index you’ve specified is less than the array length. If the index is greater 
than the length, Rust will panic, which is the term Rust uses when a program exits with an error.

This is the first example of Rust’s safety principles in action. In many low-level languages, this kind of check is not done, and when 
you provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting 
instead of allowing the memory access and continuing.

# Functions
Functions are pervasive in Rust code. You’ve already seen one of the most important functions in the language: the main function, which 
is the entry point of many programs. You’ve also seen the ```fn``` keyword, which allows you to declare new functions.

Rust code uses ```snake case``` as the conventional style for function and variable names. In snake case, all letters are lowercase and 
underscores separate words. Here’s a program that contains an example function definition:

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Function definitions in Rust start with fn and have a set of parentheses after the function name. The curly brackets tell the compiler 
where the function body begins and ends.

We can call any function we’ve defined by entering its name followed by a set of parentheses. Because ```another_function``` is defined 
in the program, it can be called from inside the main function. Note that we defined ```another_function``` after the main function in 
the source code; we could have defined it before as well. Rust doesn’t care where you define your functions, only that they’re defined 
somewhere.

Let’s start a new binary project named functions to explore functions further. Place the another_function example in src/main.rs and run 
it. You should see the following output:

```rust
$ cargo run
Compiling functions v0.1.0 (file:///projects/functions)
Finished dev [unoptimized + debuginfo] target(s) in 1.50 secs
Running `target/debug/functions`
Hello, world!
Another function.
```

The lines execute in the order in which they appear in the main function. First, the “Hello, world!” message prints, and then 
another_function is called and its message is printed.

## Function Parameters
Functions can also be defined to have parameters, which are special variables that are part of a function’s signature. When a function 
has parameters, you can provide it with concrete values for those parameters. Technically, the concrete values are called arguments, but 
in casual conversation, people tend to use the words parameter and argument interchangeably for either the variables in a function’s 
definition or the concrete values passed in when you call a function.

The following rewritten version of ```another_function``` shows what parameters look like in Rust:

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

## Statements and Expressions in Function Bodies
Function bodies are made up of a series of statements optionally ending in an expression. So far, we’ve only covered functions without 
an ending expression, but you have seen an expression as part of a statement. Because Rust is an expression-based language, this is an 
important distinction to understand. Other languages don’t have the same distinctions, so let’s look at what statements and expressions 
are and how their differences affect the bodies of functions.

We’ve actually already used statements and expressions. Statements are instructions that perform some action and do not return a value. 
Expressions evaluate to a resulting value. Let’s look at some examples.

Creating a variable and assigning a value to it with the let keyword is a statement. ```let y = 6;``` is a statement.

```rust
fn main() {
    let y = 6;
}
```

Function definitions are also statements; the entire preceding example is a statement in itself.

Statements do not return values. Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll 
get an error:

```rust
fn main() {
    let x = (let y = 6);
}
```

The ```let y = 6``` statement does not return a value, so there isn’t anything for x to bind to. This is different from what happens in 
other languages, such as C and Ruby, where the assignment returns the value of the assignment. In those languages, you can 
write ```x = y = 6``` and have both x and y contain the value 6; that is not the case in Rust.

Expressions evaluate to something and make up most of the rest of the code that you’ll write in Rust. Consider a simple math operation, 
such as 5 + 6, which is an expression that evaluates to the value 11. Expressions can be part of statements: ```let y = 6;``` is an 
expression that evaluates to the value 6. Calling a function is an expression. Calling a macro is an expression. 

The block that we use to create new scopes, {}, is an expression, for example:

```rust
fn main() {
    let x = 5;

    // 1
    let y = { // 2
        let x = 3;
        x + 1 // 3
    };
    println!("The value of y is: {}", y);
}
```

The expression 2 is a block that, in this case, evaluates to 4. That value gets bound to y as part of the let statement. Note the 
line 3 without a semicolon at the end, which is unlike most of the lines you’ve seen so far. **Expressions do not include ending semicolons**. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value. 

Keep this in mind as you explore function return values and expressions next.

## Functions with Return Values
Functions can return values to the code that calls them. We don’t name return values, but we do declare their type after an arrow (->). 
In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. 
You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression 
implicitly. Here’s an example of a function that returns a value:

```rust
fn five() -> i32 {
    5
}
fn main() {
    let x = five();
    println!("The value of x is: {}", x);
}
```

There are no function calls, macros, or even let statements in the five function—just the number 5 by itself. That’s a perfectly valid 
function in Rust. Note that the function’s return type is specified, too, as -> i32. 

The 5 in five is the function’s return value, which is why the return type is i32. Let’s examine this in more detail. There are two 
important bits: first, the line ```let x = five();``` shows that we’re using the return value of a function to initialize a variable. 
Because the function five returns a 5, that line is the same as the following:

```rust
let x = 5;
```

Second, the five function has no parameters and defines the type of the return value, but the body of the function is a lonely 5 with 
no semicolon because it’s an expression whose value we want to return.

Let’s look at another example:

```rust
fn main() {
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Running this code will print The value of x is: 6. But if we place a semicolon at the end of the line containing ```x + 1```, changing 
it from an expression to a statement, we’ll get an error.

Running this code produces an error, as follows:

```
error[E0308]: mismatched types
--> src/main.rs:7:28
|
7 | 8 | | | | 9 | | }
|
fn plus_one(x: i32) -> i32 {
| ____________________________^
x + 1;
= help: consider removing this semicolon
| |_^ expected i32, found ()
= note: expected type `i32`
found type `()`
```

The main error message, “mismatched types,” reveals the core issue with this code. The definition of the function ```plus_one``` says 
that it will return an i32, but statements don’t evaluate to a value, which is expressed by (), the empty tuple. Therefore, nothing 
is returned, which contradicts the function definition and results in an error. In this output, Rust provides a message to possibly 
help rectify this issue: it suggests removing the semicolon, which would fix the error.

# Control Flow 
Deciding whether or not to run some code depending on whether a condition is true and deciding to run some code repeatedly while a 
condition is true are basic building blocks in most programming languages. The most common constructs that let you control the flow 
of execution of Rust code are if expressions and loops.

```rust
fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

It’s also worth noting that the condition in this code must be a ```bool```. If the condition isn’t a bool, we’ll get an error. For example, try running the 
following code:

```rust
fn main() {
    let number = 3;
    if number {
        println!("number was three");
    }
}
```

The if condition evaluates to a value of 3 this time, and Rust throws an error:

```
error[E0308]: mismatched types
--> src/main.rs:4:8
|
4 | if number {
| ^^^^^^ expected bool, found integral variable
|
= note: expected type `bool`
found type `{integer}`
```

The error indicates that Rust expected a bool but got an integer. Unlike languages such as Ruby and JavaScript, Rust will not 
automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide if with a Boolean as its condition. 
If we want the if code block to run only when a number is not equal to 0, for example, we can change the if expression to the following:

```rust
fn main() {
    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }
}
```

Running this code will print number was something other than zero.

## Handling Multiple Conditions with else if
You can have multiple conditions by combining ```if``` and ```else``` in an ```else if``` expression. For example:

```rust
fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code or use
```match``` for these cases.

## Using If in a let statement
Because if is an expression, we can use it on the right side of a let statement.

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
}
```

The ```number``` variable will be bound to a value based on the outcome of the if expression.

The values that have the potential to be results from each arm of the if must be the same type; in code above, the results of both the 
if arm and the else arm were i32 integers. If the types are mismatched, as in the following example, we’ll get an error:

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        "six"
    };
    println!("The value of number is: {}", number);
}
```

When we try to compile this code, we’ll get an error. The if and else arms have value types that are incompatible, and Rust indicates 
exactly where to find the problem in the program:

```
error[E0308]: if and else have incompatible types
--> src/main.rs:4:18
|
4 | let number = if condition {
| __________________^
5 | | 5
6 | | } else {
7 | | "six"
8 | | };
| |_____^ expected integral variable, found &str
|
= note: expected type `{integer}`
found type `&str`
```

The expression in the if block evaluates to an integer, and the expression in the else block evaluates to a string. This won’t work 
because variables must have a single type. Rust needs to know at compile time what type the number variable is, definitively, so it 
can verify at compile time that its type is valid everywhere we use number. 

Rust wouldn’t be able to do that if the type of number was only determined at runtime; the compiler would be more complex and would make 
fewer guarantees about the code if it had to keep track of multiple hypothetical types for any variable.

## Repetition with Loops
It’s often useful to execute a block of code more than once. For this task, Rust provides several ```loops```. A loop runs through the 
code inside the loop body to the end and then starts immediately back at the beginning. To experiment with loops, let’s make a new project 
called loops.

Rust has three kinds of loops: ```loop```, ```while```, and ```for```. Let’s try each one.

### Repeating Code with loop
The ```loop``` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
As an example, change the ```src/main.rs``` file in your loops directory to look like this:

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

When we run this program, we’ll see again! printed over and over continuously until we stop the program manually. Most terminals support 
a keyboard shortcut, ctrl-C, to halt a program that is stuck in a continual loop.

Fortunately, Rust provides another, more reliable way to break out of a loop. You can place the ```break``` keyword within the loop to 
tell the program when to stop executing the loop.

### Conditional Loops with while
It’s often useful for a program to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition 
ceases to be true, the program calls break, stopping the loop. This loop type could be implemented using a combination of loop, if, else, 
and break; you could try that now in a program, if you’d like.

However, this pattern is so common that Rust has a built-in language construct for it, called a ```while``` loop. Below code uses while: 
the program loops three times, counting down each time, and then, after the loop, it prints another message and exits.

```rust
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
}
```

This construct eliminates a lot of nesting that would be necessary if you used ```loop```, ```if```, ```else```, and ```break```, and 
it’s clearer. While a condition holds true, the code runs; otherwise, it exits the loop.

### Looping Through a Collection with for
You could use the ```while``` construct to loop over the elements of a collection, such as an array. For example, let’s look at below code.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
}
```

Here, the code counts up through the elements in the array. It starts at index 0, and then loops until it reaches the final index in the 
array (that is, when index < 5 is no longer true). Running this code will print every element in the array.

But this approach is error prone; we could cause the program to panic if the index length is incorrect. It’s also slow, because the 
compiler adds runtime code to perform the conditional check on every element on every iteration through the loop.

As a more concise alternative, you can use a ```for``` loop and execute some code for each item in a collection.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

We’ve now increased the safety of the code and eliminated the chance of bugs that might result from going beyond the end of the array or 
not going far enough and missing some items. For example, in the code below, if you removed an item from the ```a``` array but forgot to 
update the condition to ```while index < 4```, the code would panic. Using the ```for``` loop, you wouldn’t need to remember to change any 
other code if you changed the number of values in the array.

The safety and conciseness of ```for``` loops make them the most commonly used loop construct in Rust. Even in situations in which you 
want to run some code a certain number of times, as in the countdown example that used a while loop, most Rustaceans would use a for loop. 

The way to do that would be to use a ```Range```, which is a type provided by the standard library that generates all numbers in sequence 
starting from one number and ending before another number.

Here’s what the countdown would look like using a for loop and another method we’ve not yet talked about, ```rev```, to reverse the range:

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

## Exercise
Try building programs to do the following:
- Convert temperatures between Fahrenheit and Celsius.
- Generate the nth Fibonacci number.
- Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
