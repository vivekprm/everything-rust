# Constants
Instead of `let` we use `const` keyword to create constants.
Constant variables can't be mutated. Difference is variables can be made mutable with `mut` keyword but
constants can never be mutated.

```rust
const MAX_PLAYERS: u8 = 10;
fn main(){}
```

Constants can be declared in any scope, including the Global scope. Meaning outside of main.
Value of a constant must be a constant expression, meaning value must be computed at compile time.

# Static Variables
Static variables are declared using `static` keyword and like constants naming convention is screaming 
snake case.

As constants explicit type must be specified.

```rust
static CASINO_NAME:&str = "Rusty Casino";

fn main() {}
```

Like constants it can be declared in any scope including Global scope.

Unlike constants static variables can be marked mutable. However, accessing and modifying a mutable static
variable is unsafe, so those operations must be done within unsafe block.

Let's look at the difference between using constant vs static variables. 

When using constant variables, the value of the constant will be inlined. E.g. in below snippet MAX_PLAYER
will be replaced with 10. This means constants don't occupy a specific location in memory.

On the otherhand, static variables do occupy a specific location in memory, which means that there is only one
instance of the value.

Default is to use constant. Some usecase of using static variables are:
- Storing large amount of data when you need the single address property of statics or when you are using 
interior mutability which we will look at later.

```rust
const MAX_PLAYER:u8 = 10;
static CASINO_NAME:&str = "Rusty Casino";

fn main() {
    let a = MAX_PLAYER;
    let b = MAX_PLAYER;

    let c = CASINO_NAME;
    let d = CASINO_NAME;
}
```
