use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Vivek Mishra",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long, default_value = "3")]
    number: usize,
}

fn main() {
    let opts: Opts = Opts::parse();
    let num_fruits = opts.number;
    let salad = create_fruit_salad(num_fruits);
    println!(
        "Created Fruit Salad with {} fruits: {:?}",
        num_fruits, salad
    );
}
