// integers, floats, string, char, boolean and unit type can be expressed via LITERALS

// integers can also be further expressed via
// 1. Hexadecimal - 0x
// 2. Octal - 0o
// 3. Binary - 0b

// numeric literals - 1000 => 1_000 , 0.00001 => 0.000_01 (for easy readability)

// signed integers (i) stores both positive and negative values
// unsigned integer (u) stores only positive values

fn main(){
    // integer addition
    println!("1 + 4 = {}", 1u32 + 4);

    // println!("1 - 4 = {}", 1u32 - 4); will result to an error , instead - ("1-4={}",1i32 - 4);

    
}