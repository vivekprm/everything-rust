Let's create a new Cargo package called bank with a library crate.

```sh
cargo new bank --lib
```

In `lib.rs` we can see that a test was automatically added for us. First we see a module called test
with configuration attribute. This attribute is saying, only compile this module when running tests
via `cargo test`. Inside the module we have one test function called `it_works`. Test functions in 
Rust are annotated with `#[test]`. Below is the default added test.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

In Rust, test functions could be standalone. Meaning if we move this function `it_works` outside of the tests module
it would still work. However, convention in Rust is to create a module called tests which holds our test functions.

Let's implement some code which we can test. We will create a public struct called *SavingsAccount*.

```rust
pub struct SavingsAccount {
    balance: i32,
}

impl SavingsAccount {
    pub fn New() -> SavingsAccount {
        SavingsAccount {
            balance: 0,
        }
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn deposit(&mut self, amount: i32) {
        self.balance += amount
    }
}
```

Now let's add tests to test SavingAccount functionality.

```rust
#[cfg(test)]
mod tests {
    // import everything from parent module
    use super::*;

    #[test]
    fn starting_balance_should_be_0() {
        let account = SavingsAccount::new();
        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn should_be_able_to_deposit() {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        assert_eq!(account.get_balance(), 100);
    }
}
```

Now run these tests with `cargo test`.

To make sure these are not false positives, let's change the new function. Instead of starting balance 0 make it 100.
Running the test again should fail both the tests.

There are two other useful macros in addition to `assert_eq` and those are `assert_ne` and `assert`. We can use it as below:

```rust
assert_eq!(account.get_balance(), 100, "Balance should be 100");
assert_ne!(account.get_balance(), 0);
assert!(account.get_balance() == 100);
```

We can pass the additional failure message to these macros by passing an additional parameter.

You may have noticed that these test functions don't have a return type. If the test function panics, test fails, otherwise the
test passes. All the three macros above will panic if the assertion passed in fails. It is however possible for test functions
to return a `Result` enum. This is typically used when function being tested returns a `Result` type itself.

Let's implement a function that returns a `Result` type. We will add a new method to SavingAccount called transfer, which will take
an account number and an amount. The return type will be a Result because transferring money could fail for various reasons.

```rust
impl SavingsAccount {
...
    pub fn transfer(&self, acc_number: i32, amount: i32) -> Result<String, String> {
        Ok(format!("Transferred ${amount} to ${acc_number}"))
    }

...
}
```

Next let's write a test to test transfer.

```rust
#[cfg(test)]
mod tests {
    ...

    #[test]
    fn should_transfer_money() -> Result<(), String> {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        account.transfer(123456, 100)?;
        Ok(())
    }
}
```

Now instead of using assert macros, we are returning the Ok variant if our test passes. If our test fails we return the error
variant. But we don't have to do that explicitly. We can use `?` operator to propagate errors.

Besides using the assert macros or returning the Result types, we can also check to make sure a function panics. This won't work
if a function returns a Result type. 

So let's create a new test for deposit method. Our new test asserts that deposit will panic if the negative amount is passed in.

```rust
#[test]
#[should_panic]
fn should_panic_if_deposit_is_negative() {
    let mut account = SavingsAccount::new();
    account.deposit(-1);
}
```

If we run the test we see that this test fails with "test did not panic as expected". Our test failed because our deposit method
accepts negative values. Let's fix that.

```rust
pub fn deposit(&mut self, amount: i32) {
    if amount < 0 {
        panic!("Can not deposit a negative amount!"); // no ideal for prod code
    }
    self.balance += amount
}
```

So far we have been testing public functions and methods, however we can also test private functions. E.g. If we have below function:

```rust
fn some_function() {}
```

We can access this function in the tests module. Since test module is a child module it have access to private functions as well.

We are used to separating unit tests from code, say by writing them in a test directory. However, in Rust unit tests are written
along with the source code. The convention is to have a test module in each file you are testing. Content of the module could be 
inline or separated out in separate file.
