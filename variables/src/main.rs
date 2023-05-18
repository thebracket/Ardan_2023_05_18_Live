fn main() {
    let n = {
        let mut accumulator = 0;
        for _i in 0..10 {
            accumulator += 1;
        }
        accumulator // No semicolon means "return from this scope"
    };
    println!("{n}");
}
