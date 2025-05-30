fn print_str(s: &str) {
    println!("{}", &s[0..3]);
}

fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s = "Hello, world!";
    print_str(s);

    let salutation = String::from("hello");
    print_string(salutation);
}
