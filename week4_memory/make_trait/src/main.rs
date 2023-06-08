trait Animal {
    fn speak(&self);
}

#[derive(Debug, Clone)]
struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("mew");
    }
}

#[derive(Debug, Clone)]
struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("woof");
    }
}

fn get_animal() -> impl Animal {
    Dog
}

fn main() {
    let cat = Cat;
    cat.speak();
    let dog = Dog;
    dog.speak();

    let fetched_animal = get_animal();
    fetched_animal.speak();

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];
    let (tx, rx) = std::sync::mpsc::channel::<Box<dyn Animal>>();
}
