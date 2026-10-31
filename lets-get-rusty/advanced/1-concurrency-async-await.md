# How programs are executed.

<img width="1697" height="965" alt="Screenshot 2026-10-30 at 10 11 55 AM" src="https://github.com/user-attachments/assets/933a3d71-7cda-46f4-82d5-c08253300b85" />

Your program is stored on local disk and when it's executed, a process is created in RAM. A `process` is an instance of your computer program
being executed. Typically you have one process for your program. Some programs however will have multiple processes. E.g. a web browser might
have a process for every tab you have open. This is called **Multi-processing**.

A `process` is independent in the sense that it has it's own address space or heap in RAM. It also has a copy of your program's code, data and
files. So multi-processing is useful when you want multiple instances of your program to run concurrently without having much interaction between
the instances.

Each process contains one or more threads. A **thread** is a sequence of instructions that can be executed concurrently. By default a process has
one thread. However, a process could span more than one thread or many threads. Spawning multiple threads is called **Multi Threading**.

**Threads** share the same heap but have their own stack. So threads are useful when you want different parts of your program to run concurrently
and communicate with each other.

In the above example, our program has one process and one thread which is the default. A pointer to our thread then put into a table of 
**Thread Pointers** along with all the other threads. Then a program called a **scheduler** which is part of the operating system will select
a thread and pass it to the CPU for execution. In this diagram you also notice that the threads have a **priority** high, medium or low. The
higher the priority the more likely the thread will be selected for execution.

Now the job of the scheduler is to make sure that each thread gets execution time and the way it does that is through **Preemptive Scheduling**
Each thread gets a small amount of time to execute, once that time is over, execution on that thread stops and another thread is scheduled in 
it's place. This process of switching threads is called **Context Switching**.

**Preemptive Scheduling** means that the scheduler can stop executing a thread at any given point. The programmer has no control over it. If the
CPU has only one core then only one thread could execute at a time. This means if there are multiple threads execution have to be interleaved.
Meaning one thread would get some time to execute, then another thread would get some time, then another thread and so on. If CPU has multiple
cores then multiple threads could be executed in parallel. So if the CPU has 4 cores, then 4 threads could be executed in parallel.

# Concurrency
- When different parts of your program execute independently.
    - This could be done through:
        - **Time Slicing**: Execution of these parts in **interleaved** on a single core.
        - **Parallel Execution**: Execution of these parts **happens at the same time** using **multiple cores**.

<img width="1688" height="952" alt="Screenshot 2026-10-30 at 10 55 21 AM" src="https://github.com/user-attachments/assets/92e46bc4-a287-47e4-af7b-8e4ee9c8e8ed" />

On the left we have diagram of time slicing with two threads. As time goes on each thread gets little bit of time to execute. But because
only one thread could execute at a given time execution have to be **interleaved**.

On the right, we have example of parallel execution with two threads. Imagine if a CPU had two cores and each of these threads were executed 
on one of the cores. In that case, as time goes on these threads can be executed in parallel.

So far we have talked about concurrency in context of operating system threads. However, this is just one model of concurrency. There are
also other model of cuncurrency.

**Concurrent programming** is not as mature and standardized as regular sequential programming. So this space is still evolving. Here are 
some popular concurrency models/

# Concurrency Models
- OS Threads
    - Basic concurrency primitives provided by the operating systems. Other concurrency models are **abstraction over threads**.
- Event Driven Programming
    - It's a popular programming model in Javascript with **Callbacks**.
- Actor Model
    - Divides concurrent computation into units called **Actors** that could communicate with each other through message passing.
- Asynchronous Programming
- Coroutines
    - Similar to async programming in the sense that they center around **executing functions concurrently**.
    - Main difference is **Coroutines** hide low level details where as asynschronous programming exposes some of the low level details.

The two primary concurrency models in Rust are **OS Threads** and **Asynchronous Programming**. So we will focus on comparing these two models.

# OS Threads vs Asynchronous Programming
- In OS Threads we use operating system threads managed by operating system where as **Async Tasks** (**Asynchronous Programming** the units of 
execution are often called **Tasks**) are **managed by a language runtime or a library**.
- **OS Threads** are preemptively scheduled, meaning that the programmer has no control over when a thread executes. Where as **Async Tasks**
on the other hand are **cooperatively scheduled**, meaning that the programmer can decide when to pause a task to allow other tasks to execute.
- **OS Threads** have relatively high performance overhead in terms of CPU and memory usage whereas **Async Tasks** have significantly **lower
performance overhead**.
- **OS Threads** are ideal for small amount of CPU bounded workloads. Whereas **Async Tasks** are ideal for large amount of I/O bound workloads.
- **OS Threads** are harder to reason about when you are writing code whereas **Async Tasks** are easier to reason about. That's because when
you are writing asynchronous code using **async-await** syntax, your code looks sequential eventhough it's being run concurrently.

Note that **Async Tasks** are not better than **OS Threads**, they are simply a different concurrency model used for different purposes.
E.g. you may use **OS Threads** to split-up a **CPU Intensive** workload whereas you may use a **Async Task** for **WebServers** and 
**Databases** which have a larger amount of I/O bound workloads.

Switching between using **OS Threads** and the **Async Tasks** typically requires a large amount of refactoring. So it's important to pick
the correct concurrency model at the beginning of the development cycle.

Let's see how both of these models relate to each other through visual diagram

<img width="1413" height="883" alt="Screenshot 2026-10-30 at 12 03 40 PM" src="https://github.com/user-attachments/assets/d0415096-44f5-4eb7-919a-b8260652a002" />

There are different ways an **Async Runtime** could be implemented. This is one common way.

The **Green Boxes** represent many **Async Tasks**, which maps to a few **OS threads** in a **Thread Pool**. In other languages or contexts
**Async Tasks** are referred to as **green threads** or **user threads** or **m-n threads** etc.

The underlying concept is same. Let's now dig deep into, how these **OS Threads** in the **Thread Pool** execute tasks.

# Task Scheduler/Executor
<img width="1725" height="735" alt="Screenshot 2026-10-30 at 12 08 35 PM" src="https://github.com/user-attachments/assets/391b8de8-9083-4977-ad3e-242be9f824d0" />

Inside the Thread we have something called a **Scheduler or Executor**, now implementations may vary but here is the very simple example.

**Tasks** are stored in a **run queue**. **Processor** picks up a task, begins executing it until the task pauses execution. At that point
the task is either complete meaning there is nothing to do or it's waiting for some other operation e.g. it made a database call and now it's
waiting for the results to return. Once the database result return then the Task will notify that it's ready to make more progress. At that
point it will be back on the **run queue**. In the meantime **Processor** can continue executing other tasks.

# Creating Threads
To create a thread in Rust we use `spawn` function. `spawn` takes a closure containing the code which will run in the new thread. 
First lets import `thread` from standard library.

```rust
use std::thread;

fn main() {
    thread::spawn(|| {
        for i in 0..20 {
            println!("Spawned thread: {i}");
        }        
    });
    for i in 0..10 {
        println!("Main thread: {i}");
    }
}
```

If we run it, we see that loop in our main thread executed till completion, however loop in spawn thread did not. We can see 
different result if we run it again. This is because as soon as our main thread ends program is terminated. What we want is, 
main thread to wait till the spawned thread is finished. To do this we can use `JoinHandle`.

If you look at the return value of `spawn` function, it returns `JoinHandle`. A **JoinHandle** is an owned value with a method
called `Join` which when called will block the current thread from executing until the thread represented by `JoinHandle` terminates.

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..20 {
            println!("Spawned thread: {i}");
        }        
    });
    for i in 0..10 {
        println!("Main thread: {i}");
    }

    handle.join().unwrap();
}
```

`join` method returns a `Result` type which could be an error if the associated thread panics. We are calling `unwrap`, so if the
spawned thread panics main thread will panic as well.

This time loop in the main thread finished and loop in the spawned thread finished as well. You might see different result on your
computer. On my computer looks like main thread and then spawned thread executed sequentially. However it's upto the operating 
system how to run them and we might see different outputs.

We can force threads to stop executing for a given amount of time using the `sleep` function.

```rust
use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..20 {
            println!("Spawned thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }        
    });
    for i in 0..10 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

# Moving Values Into Threads
Lets see what happens if we try to print a value defined outside the spawned thread, into the spawn thread.

```rust
use std::thread;

fn main() {
    let s = "Let's Get Rusty!".to_owned();

    let handle = thread::spawn(|| {
        println!("{s}"); // error
    });
}
```

We get an error which states that "this closure may outlive the current function but it borrow `s`, which is owned by the current function
may outlive borrowed value `s`".

The closure above will be executed in a spawned thread. Rust can't tell how long that spawned thread lives. It could be that the spawned
thread lives longer than the main thread. That would be a problem because this closure borrows `s` immutably to be used in the print.

At the end of main `s` will go out of scope and will be dropped which means, reference inside this closure would be pointing to invalid
memory. The way to fix this is to move `s` inside the closure using the `move` keyword.

```rust
use std::thread;

fn main() {
    let s = "Let's Get Rusty!".to_owned();

    let handle = thread::spawn(move || {
        println!("{s}");
    });
}
```

The spawned thread is taking ownership of `s` and our code compiles.

# Message Passing Between Threads
In below example we have an array of strings that are backwards and our goal is to reverse each string.

```rust
fn main() {
    let sentences = [
        "!dlroW olleH".to_owned(),
        ".tsurT eW tsuR nI".to_owned(),
        "!tsuR teG s'teL".to_owned(),
        "!tsuB ro tsuR".to_owned(),
    ];
}
```

To make our code efficient we will spawn a thread for each string that will reverse the string and report the result back to main thread.
To accomplish this we need some way for threads to communicate. We have two options:
- Either pass messages between threads.
- Or share state between threads.

In this case we will use message passing. Will import two modules `std::thread` and `std::sync::mpsc`. `mpsc`stands for multiple producer
and single consumer. Which is a type of channel provided by the standard library. As the name implies this type of channel allows multiple
threads to send messages and one thread to receive messages.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let sentences = [
        "!dlroW olleH".to_owned(),
        ".tsurT eW tsuR nI".to_owned(),
        "!tsuR teG s'teL".to_owned(),
        "!tsuB ro tsuR".to_owned(),
    ];

    for s in sentences {
        let tx_clone = tx.clone(); // we need to clone it as once it's moved inside the spawned thread, it can no longer be used.

        thread::spawn(move || {
            let reversed: String = s.chars().rev().collect(); // this only works with ascii chars as unicode chars are multiple char long.
            tx_clone.send(reversed).unwarp();
        });
    }

    for sentence in rx {
        println!("{sentence}");
    }
}
```

There are two ways to use the receiver in main thread:
- Using `rx.recv()`: If there are no messages being sent this method will block the current thread until all the senders are dropped.
- Use the receiver like an `Iterator`.
    - With this syntax will block the main thread waiting for messages. For each new message, we will print it out.
    - When the channel closes because all the transmitters are dropped this iteration will end.

However, we see that our program is still running even after printing out all the sentences. Why?
`Receiver` end of the channel will stop listening for messages when all `transmitters` are dropped. For each sentence we clone the
transmitter and move it into a closure. At the end of the closure it will be dropped. However, we are not dropping the original
transmitter before listening for messages. 

One way to fix this is, to remove the first sentence from sentences array and then create one of thread which will take ownership of `tx`.
Another way to fix this is, to manually drop `tx` before we start listening for messages.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let sentences = [
        "!dlroW olleH".to_owned(),
        ".tsurT eW tsuR nI".to_owned(),
        "!tsuR teG s'teL".to_owned(),
        "!tsuB ro tsuR".to_owned(),
    ];

    for s in sentences {
        let tx_clone = tx.clone(); // we need to clone it as once it's moved inside the spawned thread, it can no longer be used.

        thread::spawn(move || {
            let reversed: String = s.chars().rev().collect(); // this only works with ascii chars as unicode chars are multiple char long.
            tx_clone.send(reversed).unwarp();
        });
    }

    drop(tx);

    for sentence in rx {
        println!("{sentence}");
    }
}
```

# Sharing State Between Threads
In this example, we have a `Database` struct with one field called connections which is a vector of integers. The `Database` struct has a 
constructor function which sets connections to an empty vector. It also has a `connect` method which pushes a new connection id into the
connections vector.

```rust
#[derive(Debug)]
struct Database {
    connections: Vec<u32>
}

impl Database {
    fn new() -> Database {
        Database { connections: vec![] }
    }

    fn connect(&mut self, id: u32) {
        self.connections.push(id);
    }
}
```

Imagine that we wanted to share an instance of `Database` struct across multiple threads. In this case using shared state would be appropriate.
In order to make sharing state across threads safe, we need to use a `mutex`. Let's import it. Then in main create a database instance wrapped
in Mutex.

In other languages, we need to acquire the mutex and then release it. Rust makes it easy. `Mutex` is a distinct type, you can call method on
the underlying resource without first acquiring the lock.

```rust
use std::sync::Mutex;

#[derive(Debug)]
struct Database {
    connections: Vec<u32>
}

impl Database {
    fn new() -> Database {
        Database { connections: vec![] }
    }

    fn connect(&mut self, id: u32) {
        self.connections.push(id);
    }
}

fn main() {
    let db = Mutex::new(Database::new());

    {
        db.connect();
    }
}
```

you can see the `connect` is not availa able as db is not a database instance, it's a mutex that wraps a database. First we need to acquire
lock to the resource by calling the `lock` method.

```rust
fn main() {
    let db = Mutex::new(Database::new());

    {
        let mut db_lock = db.lock().unwrap();
        db_lock.connect(1);
    }
}
```

`db_lock` is of type `MutexGuard` that implements the `Deref` trait to point to the inner resource. This means we can now call connect.
After acquiring the lock and using the resource we have to drop the lock. But in Rust this is done automatically for us. That's because
`MutexGuard` implements the `Drop` trait and will automatically release the lock when it goes out of scope. That mean when db_lock goes
out of scope at the end of the curly brace scope the lock will be dropped.

So far our database instance is only being used in main thread. Let's spawn few more threads that connect to this database instance.
First we will create a vector that holds thread handles. In each thread we will acquire a lock to the database and call `connect`

```rust
fn main() {
    let db = Mutex::new(Database::new());
    let mut handles = vec![];

    for i in 0..10 {
        let handle = thread::spawn(move || {    // error
            let mut db_lock = db.lock().unwrap();
            db_lock.connect(i);
        });

        handles.push(handle);
    }
}
```

We get an error stating that "we are using a variable `db` that has already been moved". In the first iteration of this for loop `db` will be
moved into the closure, meaning that it can't be used again. We don't want to clone `db` because we want each thread to use the same instance.

This is the case where we want shared ownership of the database instance. Recall that we could achieve shared ownership with the **reference
counting smart pointers**. Let's wrap database instance into `Rc Smart Pointer`.

```rust
fn main() {
    let db = Rc::new(Mutex::new(Database::new()));
    let mut handles = vec![];

    for i in 0..10 {
        let db_clone = Rc::clone(&db);
        let handle = thread::spawn(move || { // error 
            let mut db_lock = db.lock().unwrap();
            db_lock.connect(i);
        });

        handles.push(handle);
    }
}
```

Now we get an error stating that "Rc can't be sent between threads safely". The `Rc pointer` is not thread safe. In order to get thread safe
shared ownership we need to use `Arc Smart Pointers`, stands for `Atomically Reference Counted Smart Pointer`.

Next let's join our thread handles in main. And lastly we will acquire a lock to db in main and print out the contents of the connection vector. 

```rust
fn main() {
    let db = Arc::new(Mutex::new(Database::new()));
    let mut handles = vec![];

    for i in 0..10 {
        let db_clone = Arc::clone(&db);
        let handle = thread::spawn(move || { // error 
            let mut db_lock = db.lock().unwrap();
            db_lock.connect(i);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let db_lock = db.lock().unwrap();
}
```

# Send & Sync Traits
There are two traits related to concurrency that are important to know, the `Send` trait and the `Sync` trait. Both of these traits are
marker traits meaning that they actually don't do anything, they are just a label on types to let the compiler know that the certain properties
are enforced by the developer. 

Both of these traits are marked unsafe because the compiler can't verify whether these properties are enforced. It's the developer's 
responsibility and if they fail to enforce these properties, it could lead to memory safety issues. We will learn more about unsafe
traits later on.

The `Send` trait is implemented on types whose ownership could be transferred between threads. Almost all Rust primitive types implement `Send`
And if a type is composed of entirely of the types that implements `Send` then it is automatically `Send` as well. One exception to this
is **Rc Smart Pointer**, because it doesn't implement the **atomic operations** to increment the reference count. This means if two threads
try to increment the reference count at the same time, it leads to undefined behaviour. That's why `Arc Smart Pointer` exists. It is marked
with `Send` because it uses **atomic operations**.

If a type implements a `Send` trait, the developer is promising the compiler, that type is safe to send across threads.

The `Sync` trait is implemented for types for which it is safe to share references between threads. More specifically if a reference to a type
implements `Send` then that type is marked as `Sync`. Like `Send`, `Sync` is implemented automatically for almost all Rust primitives.

And if a type is composed of entirely the types that implement `Sync` then that type is also `Sync`. Examples of types that are not `Sync` 
are the `Rc Smart Pointer` or the `Refcel Smart Pointer`. On the other hand `Mutex Smart Pointer` is `Sync`.

It's unlikely that you have to implement the `Send` or `Sync` trait manually. However, they show up in function signatures when dealing with
concurrent code in Rust. So they are important to know.

# Async Await Basics
In Rust, `Async Await` is special syntax which allows us to write functions, closures and blocks that can pause execution and yield control
back to the runtime, allowing other code to make progress and then pick back up where they left off. Typically those pauses are for wait for
IO.

One advantage of `Async/Await` syntax is that it allows us to write asynchronous code which looks like synchronous code.

In this example, we have two functions: a `main` function and `my_function`. Let's make `my_function` async by adding the `async` keyword.

```rust
fn main() {

}

async fn my_function() {
    println!("I'm an async function!");
}
```

This async keyword is actually syntax sugar for the following code.

```rust
fn my_function() -> impl Future<Output = ()> {
    println!("I'm an async function!");
}
```

Async functions are simply functions which simply returns something that implements trait `Future`. `Output` is an associated type which
represent the return type of the function. In this case this function is not returning anything so output is set to unit type `()`.

Let's see what the simplified version of `Future` trait looks like.

```rust
trait Future {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
```

A `Future` is a simple state machine which could be polled to check if it's ready to return a value. The `poll` method returns an enum
with two variants, either the `Future` is ready with value or the `Future` is pending because it's not done executing.

`poll` method also accepts a callback function called `wake`. If calling the `poll` method returns `Pending`, then the `Future` will 
continue making progress until it's ready to get polled again. When it's ready to get polled again the `Future` will call the `wake`
callback to notify it's executor.

If you are familiar with JavaScript `Future` are similar to `Promise`, except in `Rust` `Future` is lazy meaning that they won't do
anything unless driven to completion by being polled. This is what allows `Future` to be a zero cost abstraction.

`Future` could be driven to completion by either awaiting the `Future` or giving it to an `Executor`.

Let's see an example of how we will await a `Future`. Imagine that `read_from_database` performs some asynchronous I/O by querying a
database and eventually 

```rust
fn main() {
    my_function();
}

async fn my_function() {
    println!("I'm an async function!");

    let s1 = read_from_database().await;
    println!("First result: {s1}");
    let s2 = read_from_database().await;
    println!("Second result: {s2}");
}

async fn read_from_database() -> String {
    "DB Result".to_owned()
}
```

If we try to call `my_function` in main like regulr function we will get a warning stating that "Futures do nothing unless you `await` or poll
them". So let's try to await.

```rust
fn main() {
    my_function().await; // error
}
```

We get an error satating that "`await` keyword is only allowed inside `async` functions and blocks". So let's make main async.

```rust
async fn main() {
    my_function().await; // error
}
```

This time we get another error stating that "`main` function is not allowed to be `async`". So how do we fix this problem?
`Future` could be driven to completion in two ways:
- Either calling `await` on the `Future`.
    - Using `await` works on `Future` when it's inside other `Future` or `async` function. However for top most futures we need some code
    which will manually poll them to completion. That code is called a `runtime` or an `executor`.
    - A `runtime` is responsible for polling the top level `Future` and running them itll completion. It's also responsible for running
    multiple futures in parallel.
    - Languages such as JavaScript and C# have an async runtime built in. However, one difference with Rust is that the standard library
    doesn't provide an async runtime.
    - Sin in order to run our async code, we need a community built async runtime and the most popular one is called `tokio`.
- Or manually `polling` the `Future` until it's complete.

Let's add `tokio` as dependency in `Cargo.toml`. We will also enable `full` feature to make all of the `tokio` se capabilities available to us.

```toml
...
[dependencies]
tokio = { version = "1.17", features = ["full"] }
```

Then go to main function and annotate it with `tokio::main`. This is an `attribute` macro that allows our main function to be async 

```rust
#[tokio::main]
async fn main() {
    my_function().await;
}
```

We can see that my_function is lazy. It doesn't do anything unless we call await.

```rust
#[tokio::main]
async fn main() {
    let f = my_function();
    println!("Let's Get Rusty!");
    f.await;
}
```

Another benifit of `Future` being lazy apart from it being zero cost abstraction is, it's easy to cancel. In order to cancel the `Future` stop
polling the `Future`.

# Tokio Tasks
So far we are not taking advantage of our async code and that's because everything is running serially. To make our async code run
concurrently we can use `Tokio Tasks`.

A `Task` is a light-weight non-blocking unit of execution. A `Task` is a green thread, similar to a `goroutine`. `Task` allow top level
`Future` to be executed concurrently. In `main` let's spawn two `tokio tasks` which will both execute `my_function`.

```rust
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut handles = vec![];

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
}

async fn my_function(i: i32) {
    println!("[{i}] I'm an async function!");

    let s1 = read_from_database().await;
    println!("[{i}] First result: {s1}");
    let s2 = read_from_database().await;
    println!("[{i}] Second result: {s2}");
}

async fn read_from_database() -> String {
    sleep(Duration::from_millis(50)).await;
    "DB Result".to_owned()
}
```

`tokio::spawn` takes a `Future` as an argument and returns a `JoinHandle`. Notice that this API is very similar to the API for spawning a thread.
This is on-purpose. `tokio` tries to make it easy to switch from using `threads` to using `tasks`. We are passing in an `async` block to `spawn`.

`sleep` function in `tokio` is similar to `thread::sleep` function execept that it will stop the current `Future` from executing for given
duration instead of the entire thread.

By default, `tokio` uses a `threadpool` to execute tasks. This allows tasks to be executed in parallel. We could however force `tokio` to 
execute on one thread by changing the `flavour` to `current_thread`.

```rust
#[tokio::main( flavor = "current_thread" )]
async fn main() {
}
```

This will cause threads to be executed concurrently using time slicing instead of threads.

Like threads, in order to communicate between tasks we would either need to use message passing through a channel or share state through 
a `Mutex`. Unlike threads however, async code uses `cooperative scheduling` instead of `preemptive scheduling`. If you have two threads,
operating system can switch between two threads at any given time. But when you are dealing with async code, we as developers tell the
runtime when a block of async code is ready to yield, so that other async code can run.

E.g. let's look at `my_function`, there are two points where we yield execution by calling `await`. This gives developer more control.
However, it also puts more responsibility on developers. Specifically, we as developers need to make sure we are writing efficient
async/await code. E.g. we don't want to put CPU intensive operations inside an async function.

# CPU Intensive Code
`Async/Await` is great for I/O intensive code but it's not great for CPU intensive code. This is because if an async function performs a
CPU intensive operation not only does it block that function but it block all async function running on that thread.

As an example, let's create a new function called `expensive_computation`.

```rust
#[tokio::main]
async fn main() {
    let mut handles = vec![];

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }

    handles.push(tokio::spawn(async {
        expensive_computation();
    }));

    for handle in handles {
        handle.await.unwrap();
    }
}

fn expensive_computation() {
    let mut i = 0;

    for _ in 0..400_000_000 {
        i = i + 1;
    }

    println!("Done with expensive computation! i = {i}");
}
```

Now if we run this, we see that first two tasks started executing but then the third task blocked execution of all tasks until it was done.
As you can see this is not great. However, there will be situations where we might have CPU intensive operations inside async code.

Luckily for us `tokio` has a solution for these cases. We can wrap cpu intensive code in a call to `spawn_blocking`.

```rust
#[tokio::main]
async fn main() {
    let mut handles = vec![];

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }

    handles.push(tokio::spawn(async {
        let _res = tokio::task::spawn_blocking(|| {
            expensive_computation();
        }).await;
    }));

    for handle in handles {
        handle.await.unwrap();
    }
}
```

`spawn_blocking` accepts a closure which will run in a separate thread pool where blocking is acceptable. This means that our other tasks
will not be blocked. Let's go ahead and rerun. 

As we can see, expensive computation still took a long time to execute, however it didn't block the other tasks.

# Streams
A `Future` can return only one value on the other hand a `stream` is an asynchronous series of values. You can think of `Stream` as asynchronous
`Iterator`. `tokio` has support for streams, however in order to use them we need to add a dependency on `tokio-streams` crate.

```toml
...
[dependencies]
tokio = { version = "1.17", features = ["full"] }
tokio-stream = "0.1"
```

Then in main we will import `StreamExt` trait. To create a stream we use the `iter` function which accepts anything that implements `IntoIterator`
and turns it into a stream. In this case we are passing an array of string.

Just like regular iterators, stream support combinators like e.g. map. Unlike iterators, streams are not supported in `for` loops.
To get values out of a stream we can use a `while` loop instead.

```rust
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream = tokio_stream::iter(["Let's", "Get", "Rusty"])
        .map(|s| s.to_ascii_uppercase());

    while let Some(s) = stream.next().await {
        println!("{s}");
    }
}
```

Here, we are calling the `next` method to get the next value in the stream which returns a `Future` and then we are awaiting that `Future`.
As long as `next` returns a `Some` variant then we are taking the string inside that `Some` variant and printing it out.

One popular usecase for streams is when implementing a Server which accepts TCP connection. E.g. `tokio` has a struct called `TcpStream`.
`TcpStream` is used to listen to connections on a given IP address. Where each connection is a value in `stream`.

```rust
let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
```
