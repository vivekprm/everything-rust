# Hardware
rustc compiler is frontend for LLVM, which means it's very easy to cross compile for various core architectures and environments.

Rust compiler has excellent support for microcontrollers that have arm Cortex M or RISC 5 processors. 

We will be using **BBC microbit version2** which has an ARM Cortex M4 based microcontroller, LED matrix, some buttons, a microphone, 
a speaker and an accelerometer to play around with. It can be connected using microusb cable.

# Cross Compilation
If we build a project in rust rightnow, it would run fine on our host machine but to run it on our project board, we need to set compilation 
target. To set target for ```rustc``` is through a target triple. This is composed of a core, and sub architecture, an optional vendor and
operating system and an environment or ABI.

```
<arch><sub>-<vendor>-<sys>-<env>
```

- arch = x86_64, i386, arm, ...
- sub = [ex. arm] v5, v6, v7m, ...
- vendor = [optional] pc, apple, ibm, ...
- sys = none, linux, win32, darwin, ...
- env = eabi, gnu, elf, ...

So what target we need for the above board?
We need to look at [hardware specification](https://tech.microbit.org/hardware/#nrf52-application-processor). It has Arm Cortex M4 processor.
If we go to ARM's [website](https://developer.arm.com/Processors/Cortex-M4), we can see that it has ```Armv7E-M``` architecture and instruction set is ```thumb``` or ```thumb2```.

Next thing we do is, we head to [rust platform support page](https://doc.rust-lang.org/nightly/rustc/platform-support.html) and search for ```Armv7E-M```. We will need ```Bare Armv7E-M, hardfloat```. We can target many systems but only one get installed which is for our host machine.

So we need to manually add support for our board, we will do this through below command.

```sh 
rustup target add thumbv7em-none-eabihf
```

We can check which target are installed using:
```sh
rustup show
```

Now we are ready to write our first baremetal project.

# Bare Metal Rust
 tarting a project in rust is easy. Run below command.

```sh
cargo new myproject
```

rust-analyzer plugin runs ```cargo check``` in the background, that checks our code and dependencies for errors and does this by actually 
compiling the code and produces other artifacts. ```target``` directory is where our compiled artifacts live.

First we need to make change in ```main.rs``` file. By default rust assumes that our application is running in an operating system, so our
main function accepts arguments that we don't want or need. Also there is standard library that is getting included in the prelude ahead of
our main function which is giving us the ability to use this ```println!``` macro to standard out. We can't use that. So we need to tell
the compiler that we don't want to include the standard library or it's main function and we will do this through ```no_std``` and ```no_main``` attributes at the top of the file.

```rust
#![no_std]
#![no_main]
fn main(){
}
```

# Dependency Management
Rust project maintains a registry at https://crates.io and this is where you will find all sorts of libraries that you can import in your
projects.

Let's add our first dependency which answers the question how does our code get properly located in our microcontroller's memory map.
This is addressed in the ```cortex-m-rt``` crate which includes the linker script that will work with any arm cortex mbased devices. Let's look
at the [docs](https://crates.io/crates/cortex-m-rt) of this crate.

Another crazy awesome feature of rust where a certain type of comment that you leave in your source files can generate documentation that looks
like [this](https://docs.rs/cortex-m-rt/0.7.5/cortex_m_rt/) crate document.

This crate will locate our vector table to map interrupt and exception handlers. It'll initialize our static variables and enables the FPU
before our main function is called and it gives us some function attributes to identify our main function and our exception handler.

It will be looking for ```memory.x``` file which minimally needs to contain both the start address and size of the Flash and RAM regions of
memory for our specific microcontroller.

Let's revisit our [product specification](https://tech.microbit.org/hardware/#hardware-description) to get the start addresses we need to 
visit microcontroller's datasheet, available [here](https://docs.nordicsemi.com/bundle/ps_nrf52833/page/keyfeatures_html5.html). Download the latest pdf and search for ```memory map```.

pic

```Flash``` starts at zero and ```RAM``` starts half a gigabyte later. Let's add this as our first dependency using below command:
```sh
cargo add cortex-m-rt
```

Now let's also add ```memory.x``` file that it wants.

```rust
/* Linker script for the STM32F103C8T6 */
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}
```

Now we can also update our main function with the entry attribute that our ```cortex-m-rt``` crate is looking for. And one last note about
rust_analyzer's autocomplete suggestion if it finds what you're looking for within a dependency and you click on it, it'll automatically
import that item into scope for you the entry attribute requires that the function signature never return and then we'll actually make it 
never return.

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}
```


We also need to let the rust compiler know that it needs to run the Linker Script anytime it rebuilds our project and we do this by passing
in some arguments. Passing in both the Target and Linker arguments anytime we want to rebuild the project is going to get tedious very fast
so it's time to do little house keeping.

We can create a cargo configuration file in ```.cargo/config.toml```for our project that includes those targets and linker arguments.

```toml
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
rustflags = ["-C", "link-arg=-Tlink.x"]
```

Now anytime we run ```cargo build``` it'll automatically apply that target and those rust Compiler Flags sometimes you'll find Rust-analyzer 
needs a little manual intervention to refresh update and this leads us to another issue. Remember rust-analyzer compiles in background to 
find out error but it does it based on the host architecutre which is not ideal. We need to tell it to only check our target of interest 
otherwise we're going to get some strange errors. Let's add another configuration file ```.vscode/settings.json```, this one for vscode's 
rust-analyzer plugin.

```json
{
  "rust-analyzer.check.allTargets": false,
  "rust-analyzer.cargo.target": "thumbv7em-none-eabihf"
}
```

Now it's only going to check our baremetal target. Now run ```cargo check``` to see if everything is alright.

We get below errors:

```
error: `#[panic_handler]` function required, but not found

error: could not compile `rust-embedded` (bin "rust-embedded") due to 1 previous error

```

The error is telling that we are missing a panic handler. In Rust for normal recoverable error you just return a Result Type in the error
state from whatever function you're in. So a panic is basically reserved for fatal errors. It's when you've hit some intractable issue 
like failing an assertion and the panic handler is where your code goes to die.

Panic handler is just another function that's marked with the panic handler attribute that has a very specific function signature. It takes
a reference to a panic info argument and it never returns.

```rust
#![no_std]

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    // ...
}
```

This will get called when your code or one of your dependencies calls the panic macro directly or when trying to unwrap a result or 
option in an invalid state and in some cases the compiler will add a branch to panic. For example if you are accessing array elements
by index the compiler will insert runtime bounds check into your code that panics on an out of bounds access. So we need this thing 
even if doesn't get linked in the compiler still wants it defined. The easies option is just to pull in the 
**[panic_halt](https://docs.rs/panic-halt/1.0.0/panic_halt/)** crate, which does the absolute bare minimum. It's got the right function
signature and an infinite loop inside. Definitely not what we'd want when deploying a product but for debugging on the bench this is fine.

So let's add it to our project using:
```sh
cargo add panic_halt
```

And add it to our ```main.rs``` as below:

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    loop {}
}

```

Now rust-analyzer and ```cargo check``` both seems happy. So code should build.

# Build & Flash
To build run below cargo command.

```sh
cargo build
```

Our program is located in the target directory and either the debug or release folder depending on how we built it debugs the default.

We need to add few other tools to install the first one through rustup.

```sh
rustup component add llvm-tools 
```

llvm-tools gives us binutils, llvm-size, llvm-objdump, llvm-objcopy etc. for getting info about the binary and viewing disassembly.
The ergonomics of calling these directly for our target binary aren't great, so we are also going to install a subscommand for cargo, called
```cargo-binutils```, which is just a wrapper around llvm tools that makes them easier to use.

```
cargo install cargo-binutils
```

So yes cargo is extensible, you can create your own applications that can be invoked as cargo subscommands. For ```cargo-binutils``` all we
need to do is specify the llvm-tool we want. Let's try size:

```sh
cargo size -- -Ax
```

We can see the Linker script's handywork. It's correctly locating our code somewhere in the ```.text``` flash section of memory just after our
```.vector_table``` and we don't have any global static data but if we did we can see the ```.bss``` and ```.data``` section are located in 
the RAM region we defined in our ```memory.x``` file.

To actually Flash this on our board, we will install one last cargo subscommand ```cargo-embed```. This is going to both rebuild our code 
and talk with our secondary debug microcontroller. Send it our binary and have it program our primary microcontroller.

```sh
cargo install cargo-embed
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

Now run ```cargo embed``` command.

It's also going to allow us to connect a deugger, so that we can set breakpoints modify memory and step through code. To get ```cargo embed```
setup we minimally need to specify the microcontroller we'll be programming. Let's see what it supports.

```sh
cargo embed --list-chips
```

Let's grep for our microcontroller.

```sh
cargo embed --list-chips | grep nrf52833
```

Now let's flash our microcontroller using below command:

```sh
cargo embed --chip nrf52833_xxAA
```

Now we are going to dump this chipinfo into a configuration file, so we don't have to provide it everytime. ```cargo embed``` subcommnad
is going to be looking for an ```Embed.toml``` file and we will add our chip info as below:

```toml
[default.general]
chip = "nRF52833_xxAA"
```

# Debugging with RTT
When debugging embedded applications it's preferable to use log or println messages over full-blown debugger. If your code is trying to 
maintain communication with one or more connected devices then hitting a breakpoint halts the CPU, which can be massively disruptive to 
the timing of those interfaces and ironically can make Debugging more difficult by introducing new issues. This brings us to our first
debugging option.

- **RTT (real time transfer)**: A way to exchange data between your host computer and your target microcontroller using it's debug 
interface. So for example you want to fire out println messages to the host, you are basically writing to an in-memory ring buffer
that gets read by the debugger. Which means it's very fast. The **[rtt-target](https://docs.rs/rtt-target/latest/rtt_target/)** crate
has what we need. It's pretty simple, we have to call an initialization macro and then a println macro with any data we want to Send
to the host. It does require an implementation for critical section which we don't currently have. But **[cortex-m](https://docs.rs/cortex-m/latest/cortex_m/)** 
crate have us covered. We need to add optional critical section single core feature for my singlecore controller. So let's add both these
to our project.

```sh
cargo add cortex-m --features critical-section-single-core
cargo add rtt-target
```

We are going to enable RTT through our ```Embed.toml``` file.

```toml
[default.general]
chip = "nRF52833_xxAA"

[default.rtt]
enabled = true
```

In our code we will just add the initialization macro. 

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello world!");
    loop {
        rprintln!("Echo...");
    }
}
```

```cortex-m``` crate also has an assembly noop instruction that we can use to create a little bootleg delay.

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello world!");
    loop {
        rprintln!("Echo...");
        for _ in 0..100_100 {
            nop();
        }
    }
}

```

But sometimes we need more hardcore option, something providing standard debugging experience of setting breakpoints, viewing disassembly
and manipulating memory. We can use GDB to do that.

# Debugging with GDB
To debug an arm cortexm based microcontroller, we are going to need the arm toolchain version of GDB. 

## On Ubuntu
```sh
sudo apt install gdb-multiarch
```

## On MAC
```
brew install arm-none-eabi-gdb
```

To connect to an embedded target GDB will need a proxy to send commands to and ```cargo embed``` will do this for us. We just need to 
update our ```Embed.toml``` configuration to enable the GDB server and halt on reset to wait for GDB to connect and we'll switchoff rtt.

```toml
[default.general]
chip = "nRF52833_xxAA"

[default.rtt]
enabled = false

[default.gdb]
enabled = true

[default.reset]
halt_afterward = true

```

Let's modify our main function, get rid of rtt and and just increment local variable we can watch.

```rust
#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut x: usize = 0;
    loop {
        x += 1;
        for _ in 0..x {
            nop();
        }
    }
}

```

Now if we run ```cargo embed```, it's going to rebuild our code and block while running our GDB stub. Now run GDB in another terminal.
Connect to our target binary as:
```sh
(GDB)target remote :1337
```

We get lots of warnings which we can dump at stderr as follows:

```sh
arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/rustymicrobit 2>/dev/null
```

And now we can do all sorts of GDB thing.

```sh
(GDB)target remote :1337
(GDB) info registers
```

Let's check disassembly for this reset function.

```sh
(GDB) disassemble
```

Let's demangle those assembly.

```sh
(GDB) set print asm-demangle on
(GDB) disassemble
```

So first thing our reset function does is calls a preinit function. Let's see where that goes.
```sh
(GDB)stepi
(GDB)disassemble
```

We never defined one so it just returns us to where we came from.

```sh
(GDB)stepi
(GDB)disassemble
```

And the rest of this reset function is just initializing ```.bss``` and ```.data``` section and our FPU before it eventually calls our
main function. Let's skip ahead and set a breakpoint somewhere in main. How about where we increment x.

```sh
(GDB) break main.rs:12
```

Then run until we get there.

```sh
(GDB)continue
```

Then we can check out the local variable value.

```sh
(GDB)info locals
(GDB) continue
```

We can also print the variable and it's address.

```sh
(GDB)print x
(GDB)print &x
```

We can see it's on stack and modify variable value.

```sh
(GDB)set var x=6
(GDB)print x
```

We can also check and remove our breakpoints

```sh
(GDB)info break
(GDB)delete 1
```

And we can reset our microcontroller.

```sh
(GDB)monitor reset
```
