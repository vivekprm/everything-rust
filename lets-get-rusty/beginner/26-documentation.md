# Documentation
When developing a library that others will use, it's important to have good documentation. In Rust, you can document
your code using documentation comments. Documentation comments start with `///` instead of `//` like regular comments.

**Document Comments** are written in markdown and support code blocks. Let's document our Bank library:


```rust
/// A savings account
pub struct SavingsAccount {
    balance: i32,
}

impl SavingsAccount {
    /// Creates a `SavingsAccount` with a balance of 0
    ///
    /// # Examples
    /// 
    /// \`\`\`
    /// use bank::SavingsAccount;
    /// let account = SavingsAccount::new();
    /// assert_eq!(account.get_balance(), 0);
    /// \`\`\`
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

Some comments sections included in documentation blocks are:
- **An Example Section**: To show examples
- **A Panic Section**: To explain why a function might panic.
- **A Failure Section**: Explaining what type of failure your function can return.

In this case we have an **Example** section with a code block. Cool things about codeblocks in documentation comments is that
`cargo test` will **execute those code blocks as tests**. This means that you could be sure that the example code will compile.

We can generate documentation for our library, by running:

```sh
cargo doc
```

To open cargo doc as a webpage run:

```sh
cargo doc --open
```
