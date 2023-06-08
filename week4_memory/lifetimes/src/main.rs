fn do_something<'a>(x: &'a i32) {
    println!("{x}");
}

fn get_x<'a>(x: &'a i32, _y: &i32) -> &'a i32 {
    x
}

struct Cat(String);

struct CatFeeder<'a> {
    cat: &'a Cat
}

fn main() {
    let x = 12;
    do_something(&x);

    let a = 1;
    let b = 2;
    let _ = get_x(&a, &b);
}
