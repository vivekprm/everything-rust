# How programs are executed.

pic

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

pic

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

pic

There are different ways an **Async Runtime** could be implemented. This is one common way.

The **Green Boxes** represent many **Async Tasks**, which maps to a few **OS threads** in a **Thread Pool**. In other languages or contexts
**Async Tasks** are referred to as **green threads** or **user threads** or **m-n threads** etc.

The underlying concept is same. Let's now dig deep into, how these **OS Threads** in the **Thread Pool** execute tasks.

# Task Scheduler/Executor
pic

Inside the Thread we have something called a **Scheduler or Executor**, now implementations may vary but here is the very simple example.

**Tasks** are stored in a **run queue**. **Processor** picks up a task, begins executing it until the task pauses execution. At that point
the task is either complete meaning there is nothing to do or it's waiting for some other operation e.g. it made a database call and now it's
waiting for the results to return. Once the database result return then the Task will notify that it's ready to make more progress. At that
point it will be back on the **run queue**. In the meantime **Processor** can continue executing other tasks.

# Creating Threads

