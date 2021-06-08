fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{} vstest args: {:?}", args.len(), args);
}
