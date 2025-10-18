#[derive(Debug)]
struct Rectangle3 {
    height: u32,
    width: u32,
}

impl Rectangle3 {
    fn area (&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rect3 = Rectangle3 {
        height: 50,
        width: 35,
    };

    println!("The are of the rectangle is {} and the struct is {:#?}", rect3.area(), rect3);
}
