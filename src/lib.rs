pub fn timer(block: &dyn Fn() -> ()) {
    let start = std::time::Instant::now();
    block();
    let duration = start.elapsed();
    println!("Finished in {duration:?}");
}
