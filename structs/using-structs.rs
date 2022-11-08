#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}

fn main(){
    let rect = Rectangle {width:40,height:60};
    println!("The area of the triangle is {} square pixels", area(&rect));
    println!("rect is {:#?}", rect);
}

fn area(rectangle:&Rectangle) -> u32{
    rectangle.width * rectangle.height
}
 

//methods are defined within the context of a struct and their first parameter is always self.
//The self represents the instance of the struct the method is being called on