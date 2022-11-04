#[derive(Debug)]
struct Matrix(f32, u32, f32, u32);

fn main(){
    let tuple1 = ("smith", 1u8, -10i8, false);
    let (a,b,c,d) = tuple1;
    println!("{:?}, {:?}, {:?}, {:?}", a,b,c,d);
    println!("the number 2 item in tuple1: {:?}", tuple1.1);
    println!("tuple1: {:?}",tuple1);
    let matrix = Matrix(1.5, 8, 4.2, 6);
    println!("matrix: {:?}",matrix);
}
