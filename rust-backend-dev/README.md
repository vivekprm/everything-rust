This repository was created to practice code examples from book "Zero to Production In Rust".

What are cloud native applications?
Cloud native applications are expected to:
- Achieve high-availability while running in fault-prone environment.
- To allow us to continuously release new versions with zero downtime.
- To handle dynamic workload.

For details read [this book](https://www.manning.com/books/cloud-native-patterns).

# Rust Tool Chain
Installing ```rustup``` gives us the latest compiler with our host platform as target.

We can update the toolchain using below command:
```sh
rustup update
```

Below command gives an overview of what's installed on our system.
```sh 
rustup toolchain list
```

# Project Setup
We will be using cargo (Rust's build tool) to create skeleton of our new project:

```sh
cargo new zero2prod
```

# Inner Development Loop
While working on our project we will go through the same steps over and over again:
- Make a change
- Compile the applications
- Run tests
- Run the applications

This is known as **Inner Development Loop**. 
The speed of Inner Development Loop is an upper bound on the number of iterations that we can complete in a unit of time.

Rust doesn't help us here - compilation speed can become a pain point on Big Projects. Let's see what we can do to mitigate the issue 
before moving forward.

## Faster Linking
When looking at the inner development loop, we are primarily looking at the performance of incremental compilation - how long it takes
```cargo``` to rebuild our binary after having made a small change to the source code.

A sizable chunk of time is spent in the **linking phase** - assembling the actual binary given the outputs of the earlier compilation
stages.

The default linked does a good job, but there are faster alternatives depending on the operating system you are using.
- **lld** on Windows & Linux, a linker developed by LLVM project.
- **zld** on MacOS.

To speed up the linink phase you have to install the alternative linker on your machine and add this configuration file to the project:

```toml
# .cargo/config.toml
# On Windows
# 
# cargo install -f cargo-binutils
# rustup component add llvm-tools-preview
# 
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# On Linux:
# - Ubuntu, `sudo apt-get install lld clang`
# - Arch, `sudo pacman -S lld clang`
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]
# On MacOS, `brew install michaeleisel/zld/zld`
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
```

There is ongoing work on the Rust compiler to use lld as the default linker where possible - soon enough this custom configuration will 
not be necessary to achieve higher compilation performance! **mold** is the newest linker on the block and it looks even faster than
lld!

## cargo-watch
We can also mitigate the impact on our productivity by reducing the perceived compilation time - i.e. the time you spend looking at 
your terminal wating for ```cargo check``` or ```cargo run``` to complete.

Tooling can help here - let's install **[cargo-watch](https://github.com/watchexec/cargo-watch)** 
```sh
cargo install cargo-watch
```

cargo-watch monitors your sourcecode to trigger commands everytime a file changes. E.g.

```sh
cargo watch -x check
```

will run cargo check after every code change. This reduces the perceived compilation time.
- You are still in your IDE, re-reading the code change you just made.
- ```cargo-watch```, in the meantime, has already kickstarted the compilation process.
- Once you switch to your terminal, the compiler is already halfway through!

```cargo-watch``` supports command chaining as well.

```sh
cargo watch -x check -x test -x run
```

It will start by running ```cargo check```
If it succeeds, it launches ```cargo test```
If tests pass, it launches the application with ```cargo run```

Our inner development loop right there!

# Continuous Integration
One last thing to look ar before we get into the details of what we will be building: **Our Continuous Integration (CI) pipeline**.

With a collection of automated checks running on every commit - our CI pipeline
If one of the checks fails you can't merge to **main** - as simple as that.

## CI steps
### tests
If your CI pipeline had a single step, it should be testing. Tests are a first class concept in the Rust ecosystem and you can leverage
**cargo** to run your unit and integration tests:

```sh
cargo test
```

```cargo test``` also takes care of building the project before running tests, hence you don't need to run ```cargo build``` beforehand.
(eventhough most pipelines will invoke ```cargo build``` before running tests to cache dependencies).

### Code coverage
While using [code coverage as a quality check has several drawbacks](http://www.exampler.com/testing-com/writings/coverage.pdf) I do 
argue that it is a quick way to [collect information](https://martinfowler.com/bliki/TestCoverage.html) and spot if some portions of 
the codebase have been overlooked over time and are indeed poorly tested.

The easiest way to measure code coverage of a Rust project is via ```cargo tarpaulin```, a cargo sub-command developed by [xd009642](https://github.com/xd009642). 

You can install tarpaulin with:

```sh
# At the time of writing tarpaulin only supports
# x86_64 CPU architectures running Linux.
cargo install cargo-tarpaulin
```

while
```sh
cargo tarpaulin --ignore-tests
```

will compute code coverage for your application code, ignoring your test functions.

```tarpaulin``` can be used to upload code coverage metrics to popular services like [Codecov](https://codecov.io/) or 
[Coveralls](https://coveralls.io/) - instructions can be found in tarpaulin’s README.

### Linting
Writing idiomatic code in any programming language requires time and practice. It is easy at the beginning of your learning journey
to endup with fairly convoluted solutions to problems that could otherwise be tackled with a much simpler approach.

Static analysis can help: in the sameway a compiler steps through your code to ensure it conforms to the language rules and constraints,
a linter will try to spot unidiomatic code, overly complex constructs and common mistakes/inefficiencies.

The Rust team maintains **[Clippy](https://github.com/rust-lang/rust-clippy)** the official Rust linter. **clippy** is included in 
the set of components installed by **rustup** if you are using the **default** profile.
Often CI environments use **rustup’s minimal profile**, which does not include clippy.

You can easily install it with:
```sh
rustup component add clippy 
```

If it's already installed the command is a no-op.

You can run ```clippy``` on your project with.

```sh
cargo clippy
```

In our CI pipeline we would like to fail the linter check if ```clippy``` emits any warning. We can achieve it with:
```sh
cargo clippy -- -D warnings
```

Static analysis is not infalliable: from time to time ```clippy``` might suggest changes that you don't believe to be either correct 
or desirable.

You can mute a warning using the ```#[allow(clippy::lint_name)]``` attribute on the affected code block or disable the noisy lint 
altogether for the whole project with a configuration line in ```clippy.toml``` or a project-level ```#![allow(clippy::lint_name)]``` 
directive.
Details on the available lints and how to tune them for your specific purposes can be found in clippy’s 
[README](https://github.com/rust-lang/rust-clippy#configuration).

### Formatting
Let machines deal with Formatting while reviewers focus on architecture, testing thoroughness, reliability, observability. automated
formatting removes a distration from the complex equation of the PR review process. You might dislike this or that formatting choice
but the complete erasure of formatting bikeshedding is worth the minor discomfort.

**[rustfmt](https://github.com/rust-lang/rustfmt)** is the official Rust Formatter.
Just like ```clippy```, ```rustfmt``` is included in the set of default components installed by ```rustup```. If missing, you can easily 
install it with

```sh
rustup component add rustfmt
```

You can format your whole project with

```sh
cargo fmt
```

In our CI pipeline we will add a formatting step 

```sh
cargo fmt -- --check
```

It will fail when a commit contains unformatted code, printing the difference to the console. It can be annoying to get a fail in CI for a formatting issue. 
Most IDEs support a “format on save” feature to make the process smoother. Alternatively, you can use a [git pre-push hook](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks).

You can tune rustfmt for a project with a configuration file, ```rustfmt.toml``````. Details can be found in rustfmt’s [README](https://github.com/rust-lang/rustfmt#configuring-rustfmt).

### Security Vulnerabilities
```Cargo``` makes it very easy to leverage existing crates in the ecosystem to solve the problem at hand. On the flip side, each of those crates might hide an 
exploitable vulnerability that could compromise the security posture of your software.

The [Rust Secure Code working group](https://github.com/RustSec) maintains an [Advisory Database](https://github.com/RustSec/advisory-db) - an up-to-date collection 
of reported vulnerabilities for crates published on crates.io.

They also provide **cargo-audit** (**[cargo-deny](https://github.com/EmbarkStudios/cargo-deny)**, developed by Embark Studios, is another cargo sub-command that 
supports vulnerability scanning of your dependency tree. It also bundles additional checks you might want to perform on your dependencies - it helps you identify 
unmaintained crates, define rules to restrict the set of allowed software licenses and spot when you have multiple versions of the same crate in your lock file 
(wasted compilation cycles!). It requires a bit of upfront effort in configuration, but it can be a powerful addition to your CI toolbox) that we might be actively
working on at the moment but are still running in our production environment!

## Ready-to-go CI pipelines
The exact steps that we just discussed, ready to be embedded in our project repository for different CI providers.
- [Github Actions](https://gist.github.com/LukeMathWalker/5ae1107432ce283310c3e601fac915f3)
- [CircleCI](https://gist.github.com/LukeMathWalker/6153b07c4528ca1db416f24b09038fca)
- [Gitlab CI](https://gist.github.com/LukeMathWalker/d98fa8d0fc5394b347adf734ef0e85ec)
- [Travis](https://gist.github.com/LukeMathWalker/41c57a57a61c75cc8a9d137a8d41ec10)

# Building An Email Newsletter
## Capturing Requirements: User Stories
A user story helps us to capture who we are building for (as a), the actions they want to perform (want to) as well as their motives (so that).
We will fulfill three user stories:
- As a blog visitor,
    I want to subscribe to the newsletter,
    So that I can receive email updates when new content is published on the blog;
- As the blog author,
    I want to send an email to all my subscribers,
    So that I can notify them when new content is published;
- As a subscriber,
    I want to be able to unsubscribe from the newsletter,
    So that I can stop receving email updates from the blog.

We will not add features
- Manage multiple newsletters;
- Segment subscribers in multiple audiences;
- track opening and click rates;

What does this mean *in practice*? What do we need to build?
As soon as you start looking closer at the problem tons of questions pop up - e.g. how do we ensure that the caller is indeed the blog author? Do we need to 
introduce an authentication mechanism?
Do we support HTML in emails or do we stick to plain text? What about emojis?

We could easily spend months implementing an extremely polished email delivery system without having even a basic subscribe/unsubscribe functionality in place.
We might become the best at sending emails, but nobody is going to use our email newsletter service
- it does not cover the full journey.

Instead of going deep on one story, we will try to build enough functionality to satisfy, to an extent, the requirements of all of our stories in our first release.
We will then go back and improve: add fault-tolerance and retries for email delivery, add a confirmation email for new subscribers, etc.

**We will work in iterations**: each iteration takes a fixed amount of time and gives us a slightly better version of the product, improving the experience of our 
users. Worth stressing that we are iterating on product features, not engineering quality: the code produced in each iteration will be tested and properly documented 
even if it only delivers a tiny, fully functional feature.

Our code is going to production at the end of each iteration - it needs to be production-quality.

# SignUp A New Subscriber
We will take a first stab at implementing this user story:

```
As a Blog Visitor,
I want to subscribe to the newsletter,
So that I can receive email updates when new content is published on the blog.
```

We expect our blog visitors to input their email address in a form embedded on a web page.
The form will trigger an API call to a backend server that will actually process the information, store it and send back a response.

## Our Strategy
We are startinga new project from scratch - there is a fair amount of upfront heavy-lifting we need to take care of:
- Choose a web framework and get familiar with it;
- define our testing strategy;
- choose a crate to interact with our database (we have to save those emails somewhere!)
- define how we want to manage changes to our database schemas over time (a.k.a. migrations);
- actually write some queries;

Before tackling ```/subscriptions``` we will implement a ```/health_check``` endpoint. A good opportunity to become friends with our web framework.

we will be relying on our Continuous Integration pipeline to keep us in check throughout the process - if you have not set it up yet either use the setup above or
use this readymade template https://www.lpalmieri.com/posts/2020-06-06-zero-to-production-1-setup-toolchain-ides-ci/#5-2-ready-to-go-ci-pipelines

## Choosing a Web framework
Check out [Choosing a Rust web framework, 2020 edition](https://www.lpalmieri.com/posts/2020-07-04-choosing-a-rust-web-framework-2020-edition/) for a deep-dive on actix-web, rocket, tide and warp.

**active-web** should be our go-to web framework, it has a large and healthy community behind it and it runs 
on **tokio**, therefore minimizing the likelihood of having to deal with incompatibilities/interop between
different async runtimes.

## Our First Endpoint: A Basic health_check
Let's implement a basic ```/health_check``` endpoint: when we receive a GET request for ```/health_check```
we want to return a **200 OK** response with no body.

We can use health_check to verify that the application is up and ready to accept incoming requests.
Combine it with a SAAS sevice like pingdom.com and you can be [alerted](https://www.pingdom.com/product/alerting/) 
when your api goes dark - quite a good baseline for an email newsletter that you are running on the side.

A health-check endpoint can also be handy if you are using a container orchestrator to juggle your
application (e.g. Kubernetes or Nomad): the orchestrator can call ```/health_check``` to detect if the API
has become unresponsive and trigger a restart.

### Wiring Up Actix Web
Our starting point will be the *Hello World!* example on actix-web's homepage.

```rust
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
```

A quick ```cargo check``` shows some errors. ```cargo check``` allows us to check if our code compiles or not without producing
runnable binary. It runs the same checks that are run by ```cargo build```, but it doesn't bother to perform any machine code 
generation.

We have not added actix-web and tokio to our list of dependencies, therefore the compiler can't resolve what we imported.
```toml
[dependencies]
actix-web = "4"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

under ```[dependencies]``` in our ```Cargo.toml``` or we can use cargo add to quickly add the latest version
of both crates as a dependency of our project:

```sh
cargo add actix-web --vers 4.0.0
```

cargo add is not a default cargo command: it is provided by ```cargo-edit```, a community-maintained (cargo follows the same philosophy, 
of Rust's standard library: where possible, the addition of new functionality is explored via third-party crates and then upstreamed 
where it makes sense to do so (e.g. [cargo-vendor](https://github.com/alexcrichton/cargo-vendor))) cargo extension. 
You can install it with:

```sh
cargo install cargo-edit
```

If you run ```cargo check``` again there should be no errors.

You can now launch the application with ```cargo run``` and perform a quick manual test.

```sh
curl http://127.0.0.1:8000
```

#### Anatomy of An ```actix-web``` Application
Let's have a closer look at our ```main.rs``` code.

**Server - HttpServer** [HttpServer](https://docs.rs/actix-web/4.0.1/actix_web/struct.HttpServer.html) is the backbone supporting our
application. It takes care of things like:
- Where should the applicationbe listening for incoming requests? A TCP socket ```(e.g. 127.0.0.1:8000)?``` A Unix domain socket?
- What is the maximum number of concurrent connections that we should allow? How many new connections per unit of time?
- Should we enable Transport Layer Security (TLS)?
- Etc.

**HttpServer**, in other words, handles all *transport level* concerns.
What happens afterwards? What does HttpServer do when it has established a new connection with a client of our API and we need to 
start handling their requests?

That is where **App** comes into play.

**Application - App** [App](https://docs.rs/actix-web/4.0.1/actix_web/struct.App.html) is where all your application logic lives:
routing, middlewares, request handlers, etc.
App is the component whose job is to take an incoming request as input and spit out a response. Let's zoom in on our code snippet:

```rust
App::new()
    .route("/", web::get().to(greet))
    .route("/{name}", web::get().to(greet))
```

App is a practical example of the *builder pattern*: ```new()``` gives a clean slate to which we can add, one bit at a time, new behaviour
using a fluent API (i.e. chaining method calls one after the other). We will cover the majority of App's API surface on a need-to-know
basis.

**Endpoint - Route** How do we add a new endpoint to our App?
The [route](https://docs.rs/actix-web/4.0.1/actix_web/struct.App.html#method.route) method is probably the simplest way to go about
doing it - it is used in a *Hello World!* example after all!

**route** takes two parameters:
- **path**, a string possibly templated (e.g. "/{name}") to accomodate dynamic path segments;
- **route**, an instance of the **Route** struct

[Route](https://docs.rs/actix-web/4.0.1/actix_web/struct.Route.html) combines a *handler* with a set of guards.


