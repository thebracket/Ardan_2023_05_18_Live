#[derive(Debug)]
struct MyStruct {
    n: i32,
}

impl MyStruct {
    fn new(n: i32) -> Self {
        Self { n }
    }
}

#[derive(Debug)]
struct HasDrop {
    id: i32,
    d: MyStruct,
}

impl HasDrop {
    fn new(id: i32) -> Self {
        Self { id, d: MyStruct::new(id + 100) }
    }
}

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn move_me(x: MyStruct) {
    println!("Moved {x:?}");
}

fn main() {
    {
        let ms = MyStruct::new(1);
        move_me(ms);
        println!("Ending scope");
    }
    println!("Ended scope");

    let hd = HasDrop::new(12);
}
