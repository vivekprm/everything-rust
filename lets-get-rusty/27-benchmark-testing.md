# Benchmarking
In following example we have a library with one public function called `sort_arr`. `sort_arr` takes one argument
which is a mutable slice containing items that could be ordered. Inside the function body we use a specific sorting
algorithm to sort the array in this case **Bubble sort**.

Sorting Algorithms are located in a module called **sorting**.

```rust
pub fn sort_arr<T: Ord>(arr: &mut[T]) {
    sorting::bubble_sort(arr);
}

mod sorting {
    pub fn selection_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        for i in 0..len {
            let mut min_idx = i;
            for j in (i + 1)..len {
                if arr[j] < arr[min_idx] {
                    min_idx = j;
                }
            }
            arr.swap(min_idx, i);
        }
    }

    pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
        
    }
}
```

Now let's say we want to benchmark `sort_arr` function to track performance and make sure performance is not regressed.

Rust actually has a built-in test crate, however at this moment that test crate is unstable and only available in nightly
versions of Rust. So instead we are going to use a library called `criterion`. Let's add it in `Cargo.toml`.

```toml
[package]
name = "benchmark_tests"
version = "0.1.0"
edition = "2021"

[dependencies]

[dev-dependencies]
criterion = "0.3"
```

Next we will configure our benchmark target:

```toml
[package]
name = "benchmark_tests"
version = "0.1.0"
edition = "2021"

[dependencies]

[dev-dependencies]
criterion = "0.3"

[[bench]]
name = "sorting_benchmark"
harness = false
```

First we declare the name of our benchmark in this case **sorting_benchmark** then we set harness to false. To disable Rust's
default bench harness. In otherword, we are disabling the default benchmarking system so that we can use **criterion** benchmarking
system.

Next we will create a folder called benches in the root of our project. Inside that folder we will add a new file called
`sorting_benchmark.rs`. Bring our sort array functions into scope.

```rust
use benchmark_tests::sort_arr;
user criterion::{
    black_box,
    criterion_main,
    Criterion
};

fn sort_arr_benchmark(c: &mut Criterion) {
    
}

criterion_group!(benches, sort_arr_benchmark);
criterion_main!(benches);
```

The `criterion_group` macro is used to define a collection of functions to call with a common criterion configuration. The first
argument is the name of the group in this case `benches` and the remaining arguments are names of functions. In this case we only
have one function `sort_arr_benchmark`,

The `criterion_main` macro expands to a main function which runs all the benchmarks in a given group, in this case `benches` group.

With the setup complete let's focus on our benchmark function. Each benchmark function gets access to an instance of `Criterion`
struct, which allows us to configure and execute benchmarks. We will use the Criterion struct to create a new benchmark, first lets
setup some data.

```rust
fn sort_arr_benchmark(c: &mut Criterion) {
    let mut arr = black_box(
        [6, 2, 4, 1, 9, -2, 5]
    );

    c.bench_function(
        "sorting algorithm",
        f: |b| b.iter(|| sort_arr(&mut arr))
    );
}
```

Here we are using `back_box` function, which prevents the compiler from optimizing away computations in a benchmark.
To create a new benchmark we call `bench_function` on Criterion struct instance. It takes two arguments:
- **id**
- **closure**: in this we get access to an instance of **Bencher** which allows us to iterate a benchmark function to measure it's 
performance.
    - Inside the closure body we call the iter method which takes another closure that executes our `sort_arr` function. `sort_arr`
    function will be executed many times for it's performance to be accurately measured.

To run our benchmarks we run:

```sh
cargo bench
```

Now let's go to `lib.rs` and change our sorting algorithm. Instead of bubble sort let's use selection sort which is much slower and
run the benchmark again.

Running benchmarks can help us in detecting regressions in performance sensitive code.
