#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
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
    let rect = Rectangle {width:40,height:60};
    let circle = Circle {radius: 7.0};
    println!("The area of the triangle is {:#?} square pixels", rect.area());
    println!("The area of the circle is {} m2", circle.area());
    println!("rect is {:#?}", rect);
}
 

//methods are defined within the context of a struct and their first parameter is always self.
//The self represents the instance of the struct the method is being called on
// methods helps for organization