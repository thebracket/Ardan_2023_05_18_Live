use std::rc::Rc;

#[derive(Debug)]
struct Droppable(i32);

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

impl Droppable {
    fn new(n: i32) -> Self {
        Self(n)
    }
}

fn move_me(x: Rc<Droppable>) {
    println!("Moved {}", x.0);
}

fn main() {
    let my_shared = Rc::new(Droppable::new(1));
    {
        let _x = my_shared.clone();
        let _y = my_shared.clone();
        let _z = my_shared.clone();
    }
    move_me(my_shared.clone());
    println!("{my_shared:?}");
}
