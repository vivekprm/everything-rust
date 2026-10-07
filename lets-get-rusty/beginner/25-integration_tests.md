# Integration Tests 
Let's add integration tests for our bank library created previously.

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
        if amount < 0 {
            panic!("Can not deposit a negative amount!"); // no ideal for prod code
        }
        self.balance += amount
    }
}

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

    #[test]
    #[should_panic]
    fn should_panic_if_deposit_is_negative() {
        let mut account = SavingsAccount::new();
        account.deposit(-1);
    }

    #[test]
    fn should_transfer_money() -> Result<(), String> {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        account.transfer(123456, 100)?;
        Ok(())
    }
}
```

Integration tests are going to be external to our library and will only test the public interface.

While unit tests are meant to test small units of code, integration tests test the interaction between multiple units
of code. In Rust, integration tests are stored in a top-level directory called tests. **Cargo** knows to look for
integration tests in this directory. Lets create a new file in tests directory called `savings_account.rs`.

```sh
mkdir tests
touch tests/savings_account.rs
```

Cargo will compile each file in tests directory as a separate trait.

```rust
use bank::SavingsAccount;

#[test]
fn should_have_a_starting_balance_of_0() {
    let account = SavingsAccount::new();
    assert_eq(account.get_balance(), 0);
}
```

To run this test we can run `cargo test`. Unit tests and integration tests are separated out in console output. 

Since each file in tests directory is treated as separate crate, downside is we can't share code.

Let's say we wanted to share code between multiple files so we created a file called `utils.rs`.

```sh
touch tests/utils.rs
```

Inside this file we will created a public function called `common_setup`, which is supposed to be shared across files.

```rust
pub fn common_setup() {

}
```

If we run our test again using `cargo test`, we see that Cargo is running it as a separate test file. Because Cargo treats
every file in the tests directory as a separate crate. To work around this we can create a utils module using a `mod.rs` file.

```sh
mkdir tests/utils
touch tests/utils/mod.rs
```

Copy the code from `utils.rs` and delete `utils.rs`.

```rust
pub fn common_setup(){}
```

We now have a utils module and because `mod.rs` is not a top level file in the tests directory, Cargo will not treat it as a
test file. Now add utils as a child module in `savings_account.rs` and we can use `common_setup` in our test function.

```rust
use bank::SavingsAccount;

mod utils;

#[test]
fn should_have_a_starting_balance_of_0() {
    uitls::common_setup();
    let account = SavingsAccount::new();
    assert_eq(account.get_balance(), 0);
}

```

**One thing to note is that unit tests can't import items from binary crate directly because of this in Rust a common pattern is to
have a small binary crate and a library crate which contains most of the functionality, which could be integration tested**.
