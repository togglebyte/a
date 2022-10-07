pub fn run() {
    println!("Hello, world!");
    #[cfg(feature="bob")]
    println!("you enabled bob");
}
