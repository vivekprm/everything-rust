# Macros
Macros are also called `Syntax Extensions`. Or a way of writing code that writes other code. This is a form of **meta-programming**. Macros
are expanded at compile time.

Before diving into the details, let's look at the pros and cons.

Pros:
- On the upside `macros` help reduce the amount of code you have to write and maintain.
- Allow us to extend the syntax of the language.

Cons:
- Macros can be difficult to read, understand, debug and maintain.
- Also because `macros` are processed at compile time they can increase compile time.

# How Compilation Works
```
Source
Code
  |
  v

Lexical Analysis -> Syntax Analysis -> Semantic Analysis -> Intermediate Code Generation -> Code Optimization -> Target Code Generation
                                                                                                                            |
                                                                                                                            v
                                                                                                                          Binary
```
For the macros, we will focus on first three stages of compilation process.

## Lexical Analysis

<img width="1710" height="923" alt="Screenshot 2026-10-31 at 9 44 09 PM" src="https://github.com/user-attachments/assets/656e8022-8655-4516-9382-d47edf633819" />

From the perspective of compiler, your source code starts of as a meaningless text file. The Job of `Lexical Analysis` is to turn this
meaningless text into a stream of tokens and then classify the tokens. **Tokens** are indivisible lexical units. Think about them as 
words a language understands. And they can be grouped into several categories. Such as:

- Identifiers
- Literals
- Keywords
- Symbols

In above example, `impl`, `for` and `fn` are all `Keywords`. `Animal`, `Dog` and `speak` are all `Identifiers`. 
The string "woof!" is a `Literal` and `{`, `(` and `&` are all `Symbols`.

Note that in this stage of compilation process tokens have been identified but no meaning have been attached to them.

Once tokens are identified and categorized they could be used by tools such as syntax highlighters which we can see in the example to the right.

Stream of tokens is then turned into a stream of token trees. Almost all tokens are token trees themselves, specifically leaf nodes. The only
non-leaf nodes are grouping tokens such as parantheses, brackets or curly braces. 

<img width="843" height="250" alt="Screenshot 2026-10-31 at 9 50 47 PM" src="https://github.com/user-attachments/assets/28fb1f37-9b9d-44fa-b07b-c87089b7a0df" />

In this example `impl`, `Animal`, `for` and `Dog` are all `token trees` with one leaf node. The `{}` are a root node in a token tree containing
`fn`, `speak`, `()` and `{}`.

`Macros` in Rust deal with tokens and token trees. So it's important to understand these two concepts.

Next thing in the compilation process is, `Syntax Analysis`, which turns a `Stream of token trees` into an `Abstract Syntax Tree`.

<img width="845" height="360" alt="Screenshot 2026-10-31 at 9 57 30 PM" src="https://github.com/user-attachments/assets/24d6c96e-8fd0-4c93-a5db-68055a7451c7" />

The `Abstract Syntax Tree` represents the syntax structure of your program. The `nodes` in this syntax tree could be `literals`, `functioncalls`
`if else branches` and more. Using the `Abstract Syntax Tree` programs like `linters`, `test coverage tools` and `transpilers` are implemented.

In Rust, `macros` are processed or expanded, after the `Abstract Syntax Tree` is constructed but prior to `Semantic Analysis`.

<img width="836" height="376" alt="Screenshot 2026-10-31 at 10 03 46 PM" src="https://github.com/user-attachments/assets/5cc10a92-7ef5-46fc-ae79-bdb81b75982a" />

Looking at the code to the left we can see that it contains a `macro` the `println!` macro. 

<img width="839" height="419" alt="Screenshot 2026-10-31 at 10 04 08 PM" src="https://github.com/user-attachments/assets/74565f2c-a337-4e03-b75d-693d95782887" />

This `macro` will be identified as a node in the `Abstract Syntax Tree` and then be expanded into regular rust code.

After all `macros` are expanded `Semantic Analysis` checks to make sure that `Abstract Syntax Tree` semantically makes sense.

<img width="843" height="430" alt="Screenshot 2026-10-31 at 10 05 46 PM" src="https://github.com/user-attachments/assets/64ae095b-d093-45d7-8605-58bfa8cdcd9c" />

In this case, AST is semantically correct. However, Imagine if we implemented `Animal` for a type that didn't exist e.g. `xzy`, in this case
our tree is structurally correct but semantically it doesn't make sense.

<img width="836" height="426" alt="Screenshot 2026-10-31 at 10 07 15 PM" src="https://github.com/user-attachments/assets/82a96d75-0048-4208-a774-25218d0a1ebc" />

In summary, we learned that `macros` are a way of writing code that writes other code and they are expanded or processed at compile time.
Specifically, macros are expanded after the `Abstract Syntax Tree` is constructed but before it gets validated by `Semantic Analysis`.
