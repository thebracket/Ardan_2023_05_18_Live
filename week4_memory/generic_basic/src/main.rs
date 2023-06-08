/*fn print_it<T: ToString>(x: T) {
    println!("{}", x.to_string());
}*/

fn print_it<T, U>(x: T, y: U)
where
    T: ToString,
    U: ToString,
{
    println!("{}", x.to_string());
}

struct Degrees(f32);
struct Radians(f32);

impl From<Radians> for Degrees {
    fn from(value: Radians) -> Self {
        Degrees(value.0.to_radians())
    }
}

impl From<Degrees> for Radians {
    fn from(value: Degrees) -> Self {
        Radians(value.0.to_degrees())
    }
}

fn sin<ANGLE: Into<Radians>>(x: ANGLE) -> f32 {
    let x: Radians = x.into();
    x.0.sin()
}

fn main() {
    print_it("Hello!", 23);
    print_it("Hello".to_string(), "world");
    print_it(32, "yay");

    let behind_you = Degrees(180.0);
    let behind_you_radians = Radians::from(behind_you);
    let behing_you_radians2: Radians = Degrees(180.0).into();
}
