use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("The input query string is: {}", query);
    println!("The file to be searched is: {}", filename);
}
