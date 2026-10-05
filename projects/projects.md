# Create our own coding harness. Automate team workflow or your daily workflow.
# Desktop app e.g like whisprflow, Murmur, chess.
# Build a game with Bevy engine
# AI Powered embedded edge device. Microcontroller that runs tiny ML model. Gesture detection, sensor anomaly detection.
# Build a realtime loadbalancer.
# Build your own programming language.
 - First read raw source text.
 - Lexer: Break it into tokens.
 - Parser: Organize the tokens into AST to represent your program's structure.
 - Execute the nodes:
    - Either by running the node directly, which is known as **tree-walking interpreter** and is the easiest place to start.
    - Or translating the tree into compact instructions and running them on a mini virtual machine that you create which is knows as a **bytecode VM** and takes more effort to build.
    - One great thing about Rust is that it's enums and exhaustive pattern matching make it pretty well suited to parse ASTs.
    - This skill allows us to build many other things such as DSLs, linters, formatters, static analysis tools, query builders, ORMs, templating engines, Serializers and protocol parsers.
- Build your own async runtime such as Tokio crate.
    - Will learn about Futures, State Machines, Wakers, the Pin smart pointer.
    - https://jacko.io/async_intro.html
- Build a mini serverless platform. Think of it as your own mini version of AWS Lambda or Cloudflare workers.
    - Problem that you are trying to solve is, how do you take someone else's untrusted code and safely run it.
    - Web assembly is modern answer to this. It's a portable compiled format that runs in a sandbox. The guest code can only do what you explicitly allow. By default there is no access to file system, network or outside memory access unless you explicitly allow it.
    - So this project will involve building a host program that takes a WASM file that somebody else wrote, gives it a small controlled API to call into, passes data back and forth and enforces limits.
    - Rust is perfect for this because it has first class Web Assembly support.
- Build your own storage engine. The heart of any database is it's storage engine, the part that actually puts bytes on disk and retrieves them.
    - This will involve building an embedded storage engine. Which is essentially a library that you compile into your app instead of a separate server that you connect to over the network.
    - You can start by building a key value store. Data shouldn't be lost if the program crashes.
    - It will teach concepts like durability, crash consistency and performance tradeoffs. 
- Build a Linux observability tool with eBPF. Normally the linux kernel is off limit to your code, but eBPF changes that. eBPF is revolutionary linux kernel technology that lets you run your own mini programs inside the linux kernel.
    - You can attach your program to a hook. e.g. run this everytime any process makes a syscall, sends a network packet or enters or exits a function.
    - This allows you to get extreme level of visibility into the operating system with almost no overhead.
    - That's why eBPF is used in observability for e.g. tracing, profiling and metrics, security spotting suspicious behavior the moment it happens and networking things like load balancing, firewalls and routing between containers. 
    - The great thing about Rust is, we have the IO library which allows you to write eBPF based applications.
- The tool you build for yourself

# How Terminals Work
