# Matching
Match is a powerful control flow operator. It allows you to compare a value against a series of patterns to
determine which code path to execute. Patterns can be:
- Literal values
- Variable names
- Wildcard
- Etc.

Let's look at match expression and how it works. Let's say we want to print different string based on a person's age:

```rust
fn main() {
    let age = 35;

    match age {
        1 => println!("Happy first birthday!"),
        13..19 => println!("You are a teenager!"),
        _ => println!("");
    }
}
```

_ is catch all so if an age doesn't match first two patterns we print "". What if we want to print out person's
age instead of empty string. _ pattern doesn't bind to values, so instead let's use a variable name x.

```rust
fn main() {
    let age = 35;

    match age {
        1 => println!("Happy first birthday!"),
        13..19 => println!("You are a teenager!"),
        x => println!("You are {x} years old!");
    }
}
```

You can use match expressions to match against:
- Integers
- Booleans
- Tuples 
- Structs 
- etc.

Match expressions are extremely useful to match against Enums. Let's look at below example from previous lesson.

In the last lesson we looked at dummy implementation of editor command and added a dummy serialize method. 

```rust
enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String,
    }
}

impl Command {
    fn serialize(&self) -> String {
        let json_string = match self {
            Command::Undo => String::from(
                "{ \"cmd\": \"undo\" }"
            ),
            Command::Redo => String::from(
                "{ \"cmd\": \"redo\" }"
            ),
            Command::AddText(s) => {
                format!(
                    "{{ \
                        \"cmd\": \"add_text\", \
                        \"text\": \"{s}\" \
                    }}"
                )
            },
            Command::MoveCursor(x, y) => {
                format!(
                    "{{ \
                        \"cmd\": \"move_cursor\", \
                        \"x\": \"{x}\", \
                        \"y\": \"{y}\", \
                    }}"
                )
            },
            Command::Replace{from, to} => {
                format!(
                    "{{ \
                        \"cmd\": \"replace\", \
                        \"from\": \"{from}\", \
                        \"to\": \"{to}\", \
                    }}"
                )
            }
        };

        json_string
    }
}

fn main() {
    let cmd1 = Command::Undo;
    let cmd2 = Command::AddText(String::from("test"));
    let cmd1 = Command::MoveCursor(22, 0);
    let cmd1 = Command::Replace {
        from: String::from("a"),
        to: String::from("b"),
    };
    
    println!("{}", cmd1.serialize());
    println!("{}", cmd2.serialize());
    println!("{}", cmd3.serialize());
    println!("{}", cmd4.serialize());
}
```

Lets add real implementation using matching.
