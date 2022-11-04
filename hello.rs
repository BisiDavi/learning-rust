#[derive(Debug)]
struct Person<'a>{
    name: &'a str,
    age:u8
}

fn main(){
    let d = 8 + 20;
    println!("Hello Nigeria");
    println!("Elections holds in Nigeria, February");
    println!("d is {}", d);
    println!("{number:>5}", number=1);
    println!("{number:0>3}", number=1);
    println!("My name is {0}, {1} {0}", "Cole", "Andrew");

 
    let name = "Dave Olubisi";
    let age = 22;
    let user = Person {name, age};
    println!("{:#?}",user);
}