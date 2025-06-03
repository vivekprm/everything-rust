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


