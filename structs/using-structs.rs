#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other:&Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)]
struct Circle{
    radius:f64
}

impl Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * (22.0 / 7.0)
    }
}

fn main(){
    let rect_1 = Rectangle {width:40,height:60};
    let rect_2 = Rectangle {width:10,height:20};
    let circle = Circle {radius: 7.0};
    println!("Can rect_1 hold rect_2? {}", rect_1.can_hold(&rect_2));
    println!("The area of the rectangle is {:#?} square pixels", rect_1.area());
    println!("The area of the rectangle is {:#?} square pixels", rect_2.area());
    println!("The area of the circle is {} m2", circle.area());
    println!("rect is {:#?}", rect_1);
}
 

//methods are defined within the context of a struct and their first parameter is always self.
//The self represents the instance of the struct the method is being called on
// methods helps for organization