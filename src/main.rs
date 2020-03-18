#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}

impl Rectangle {
    fn area(&self) -> u32
    {
        self.height * self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width : 50,
        height : 50,
    };

    println!("The area of Rectangle is : {}", rect1.area());
}
