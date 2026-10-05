# Computer resources
A computer provides 2 basic resources:

- Computation
    - CPU
- Memory
    - Persistent
    - Volatile

## Memory
An example of **persistent memory** is a Hard Drive or SSD. It is:
- Slow
- Abundant
- Used to persist data

An example of **volatile memory** is RAM. It is:
- Fast
- Scarce
- Used during program execution

When we talk about memory management, we don't need to manage persistent memory. What we need to manage is,
how our program manages **Volatile Memory** while executing.

### Memory Regions in Volatile Memory
Below are different regions available in **Volatile Memory**.

pic

We have 3 distinct memory regions with their own unique characteristics:
- Static Memory
- Heap
- Stack

#### Stack
##### Contents
- Function Arguments
- Local Variables
- Known Size at compile time

##### Size
- Dynamic / Fixed upper limit
    - If we cross the fixed upper limit, we get the infamous **Stack Overflow** error.

##### Lifetime
- Lifetime of a function

##### Cleanup
- Automatic when function returns

#### Static Memory
##### Contents
- Stores program's binary instructions
- Static variables
- String literals

##### Size
- Fixed size (known at compile time)

##### Lifetime
- Values in this regions lives for the Lifetime of a program

##### Cleanup
- Automatic when program terminates

#### Heap Memory
##### Contents
- Values that live beyond a function's lifetime.
- Values accessed by multiple threads.
    - That's because each thread have their own stack but all threads share the same heap.
- Large Values
    - Since stack has upper fixed bound but heap doesn't it's good idea to store large values on Heap.
- Unknown size at compile time.
    - For example, we have a program that asks user their name. You don't know how long their name is going
    to be. So you don't know the size of the string that you need to store, therefore it's good idea to store
    it on heap.

##### Size
- Dynamic
    - Only limitation is amount of physical memory (RAM) you have.

##### Lifetime
- Lifetime of values in the heap is determined by the programmer.

##### Cleanup
- Manual
    - Programmer has to cleanup memory on the heap themselves.


So when we are discussing memory management, we are discussing managing memory on the heap.

# Memory Management Strategies

## Manual (C)
Pros:
- Full Control
- Efficient

Cons:
- Tedious
- Error Prone

## RAII(C++) /OBRM (Rust)
RAII stands for Resource Acqusition Is Initialization was created for C++.
OBRM stands for Ownership Based Resource Management created for Rust. Constrast to RAII which you can choose, 
OBRM is built into the language.

Pros:
- Full Control
- Efficient
- Mostly Error Free

Cons: 
- Somewhat tedious

## Automatic (Java, C#, etc)
Cleanedup usin Garbej Collection.

Pros:
- Easy
- Error Free

Cons:
- No Control
- Not Efficient
    - Because Garbaje collector has to pass program execution once in a while to cleanup memory.

# C++ RAII vs Rust OBRM
## RAII (Resource Acquisition Is Initialization)
This is a technique/pattern/best-practice for exception safe resource management in C++.

### What Is a Resource
- Something with a finite supply that requires management.
- Examples
    - Heap Allocated Memory
    - Network sockets
    - File Handles 
    - Database Handles
    - Mutexes
    - etc..

So far we have been just talking about memory and managing memory. However, memory is just one resource, there
are many other types of resources and all of them needs to be managed. Which brings us to the problem.

### Problem
Managing resources manually is error prone and ownership is ambiguous. To better understand it, let's look at
below simple C++ program:

```c
class Car {};

void memory_example() {
    Car* car = new Car();           // Allocates memory on the heap
    function_that_can_throw();      // memory leak if exception is thrown
    if(!should_continue()) return;  // memory leak if early return
    delete car;                     // cleanup memory on the heap
}
```

Let's look at similar example with file handle:

```c
void file_example() {
    ofstream file("example.txt");   // acquire file handle
    function_that_can_throw();      // file is never closed if exception is thrown
    if (!should_continue()) return; // file is never closed if early return
    file.close();                   // Close file handle
}
```

In both the cases we can see if an excpetion is thrown or there is early returns resource is not freed.
You can see manually managing the resource is error prone.

Also you can see ownership is ambiguous, e.g. imagine we have multiple pointer to the Car stored on heap, who
is responsible for cleaning up.

To fix this problem RAII was created, the idea being instead of manually managing resources let objects
specifically the constructors and destructors manage the resources.

```c
class CarManager {
private:
    Car* p; // pointer to a car
public:
    CarManager(Car* p): p(p){}
    ~CarManager() {
        // cleanup memory on the heap
        delete p;
    }
}

class Car{}

void memory_example() {
    CarManager car = CarManager(new Car);           // Allocates memory on the heap
    function_that_can_throw();      // memory leak if exception is thrown
    if(!should_continue()) return;  // memory leak if early return
}
```

You notice here we are not using new for CarManager, so instance will be created on stack instead of heap.
So when this function returns CarManager is cleanedup and it's destructor is called, which cleans up resource
being held by CarManager, in this case Car.

So code looks lot cleaner and we can't create memory leak by mistake like in above example.

Similarly we can use the pattern for file handle example.

```c
class File {
private:
    ofstream file; // file handle
public:
    File(string file_name) {
        file = ofstream(file_name);
    }
    ~File() {
        // close the handle
        file.close();
    }
};

void file_example() {
    File file = File("example.txt");
    function_that_can_throw();
    if(!should_continue()) return;
}
```

In this case as well, file is stored on the stack so at the end of this function it will be cleaned up and
it's destructor will be called and thus file handle will be closed.

We used CarManager class to manage a Car instance that's allocated on the heap. CarManager only works for cars,
however, many different datatypes could be stored on the heap what if we created a class that manages heap
allocated memory no matter what data type it is.

Fortunately, we don't have to implement it ourselves because C++ standard library already has something like this,
it's called **Unique Pointer**. It works as below:

```c
function memory_example() {
    unique_ptr<Car> car = make_unique<Car>();
    function_that_can_throw();
    if(!should_continue()) return;
}
```

We don't need to create new instance of car in this case using new keyword. It's automatically done.

We can't create another unique pointer pointing to the same car, that's compilation error. So below is error:
```c
unique_ptr<Car> car = make_unique<Car>();
unique_ptr<Car> car2 = car; // error
```

So it fixes ambiguous ownership issue. So unique pointer represents one single owner of a resource.
However, there is a way to have shared ownership, to do that we can use `shared_ptr` instead.

```c
shared_ptr<Car> car = make_shared<Car>();
shared_ptr<Car> car2 = car;
```

shared_pointer allows us to share ownership of a resource by using reference counting. When one shared_pointer
is created reference count is 1. When second shared_pointer is created and assigned to car reference count goes
up to 2. Now there are two shared references pointing to the same car allocated on the heap.

When this function returns car will be cleaned up it's destructor will be called and refrence count will become 1.
Then car2 will be cleaned up it's destructor will be called and reference count will be 0. Once the reference
count is 0 then the heap allocated memory will be cleaned up.

## Ownership Based Resource Management
It's very similar to RAII but instead of being a best-practice/pattern it's a bulit-in language feature.

Ownership rules are checked at compile time:
- Each value in Rust has a variable that's called it's **owner**.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

Let's take a example and compare ownership system with RAII.

```rust
struct Car{}

fn memory_example() {
    let car = Box::new(Car{});
    let my_string = String::from("LGR");
    function_that_can_panic();
    if !should_continue() { return; }
}

fn file_example() {
    let path = Path::new("example.txt");
    let file = File::open(&path).unwrap();
    function_that_can_panic();
    if !should_continue() { return; }
}
```

In Rust we can allocate memory on the heap using `Box` smart pointer, which is similar to unique_ptr in C++.

There are also other ways to allocate memory on the heap e.g. on the next line we are creating heap allocated
string.

According to the ownership rule each value in rust has a varible that's called it's owner. In this case car 
variable is the owner of the heap allocated car instance in the right and my_string is the owner of this
"LGR" string created on heap.

At the end of this function car and my_string will go out of scope, meaning there values will be cleaned up.

In OBRM it's language feature which is on by default and we can opt out of it if required, using unsafe Rust.

Another difference between C++ and Rust is that move semantics are impilict. e.g. let's say we want to create
another car unique_ptr called `car2` and we will set it equal to `car`, an error will be thrown because 
unique_ptr requires single ownership. We can't have to unique_ptr pointing to the same car instance stored on 
the heap.

```c
unique_ptr<Car> car = make_unique<Car>();
unique_ptr<Car> car2 = car; // error
```

We can however move the ownsership from car to car2 using move function.

```c
unique_ptr<Car> car = make_unique<Car>();
unique_ptr<Car> car2 = move(car);
```

At this point `car2` owns the car instance allocated on the heap and `car` is no longer valid.

Let's do the same thing in Rust:

```rust
let car = Box::new(Car{});
let car2 = car; 
```

Notice with Rust we don't get any errors and that's because move semantics are implicit. Meaning when we set 
`car2` equal to `car`, `car2` will be the owner of the heap allcated car instance and `car` will be invalidated.

Remember we said ealier, in rust there can be only one owner of a value at a time. But what happens if we want
to have shared ownership over a resource?
In C++ instead of using `unique_ptr` we can use `shared_ptr`:

```c
shared_ptr<Car> car = make_shared<Car>();
shared_ptr<Car> car2 = car;
```

Now `car` and `car2` both have shared ownership of the car instance created on heap. 
To do that in Rust, we simply replace `Box` with `Rc` which stands for Reference counting.

```rust
struct Car{}

fn memory_example() {
    let car = Rc::new(Car{});
    let car2 = car.clone();
    let my_string = String::from("LGR");
    function_that_can_panic();
    if !should_continue() { return; }
}

fn file_example() {
    let path = Path::new("example.txt");
    let file = File::open(&path).unwrap();
    function_that_can_panic();
    if !should_continue() { return; }
}
```

We use clone to create another reference pointing to the same instance.

