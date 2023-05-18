fn double(n: i32) -> i32 {
    n * 2
}

#[allow(clippy::needless_return)]
fn double_or_nothing(n : i32) -> i32 {
    if n > 0 {
        return n * 2;
    } else {
        return 0;
    }
}

fn main() {
    let n = double(2);
    let a = double_or_nothing(2);
    let b = double_or_nothing(-2);
    println!("{n} {a} {b}");
    println!("{}", double(2));
}
