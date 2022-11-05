// array is a collection of objects of the same types, their length are known at compile time.
// slice is also a collection of objects of same type but with an unknown length a compile time.

fn main(){
    let age_group: [i32; 4] = [2,20, 14, 32];
    let brd:[i32; 500] = [0; 500];

    println!("brd is:{:?}",brd.len());
    println!("age_group is:{:?}",age_group);
}