trait Animal {
    fn speak(&self);
}

struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}

fn main() {
    let cat = Cat;
    cat.speak();
}
