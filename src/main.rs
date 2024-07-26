mod args;

fn main() {
    let options = args::parse();
    println!("{options:?}");
}
