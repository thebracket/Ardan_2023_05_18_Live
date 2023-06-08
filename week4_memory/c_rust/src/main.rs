extern "C" {
    fn double_it(x: i32) -> i32;
}

mod rust {
    pub fn double_it(x: i32) -> i32 {
        x * 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_double_it() {
        assert_eq!(unsafe { double_it(2) }, 4);
    }

    #[test]
    fn test_c_rust() {
        assert_eq!(unsafe { double_it(2) }, rust::double_it(2));
    }
}

fn main() {
    println!("Hello, world!");
}
