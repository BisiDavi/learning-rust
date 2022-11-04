/*
Primitives
    1. Scalar Types
    2. Compound Types

    Scalar Types
    i. signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
    ii. unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
    iii. floating point: f32, f64
    iv. char Unicode scalar values like 'a','b'
    v. bool - true/false
    vi. unit type () - empty tuple

    Compound Types
    i. Arrays [1,2,4,10]
    ii. Tuples (1, false)

    unsigned integers can't store negative values, can't go below zero. Zero is their minimum value, they can't have a minus sign in front.
    signed integers can store both positive and negative values. 

    5 stable signed integers (i8, i16, i32, i64 , isize) , i128 is mainly experimental
    5 stable unsigned integers (u8, u16, u32, u64, usize) , u128 is also experimental

    usize & isize are pointer sized. They are based on the hardware architecture.
    if the hardware is a 32-bit system, usize is u32, isize is i32
    if the hardware arch. is 64-bit, usize is u64, isize is i64

    assigning negative value to unsigned integer(u) will result to error
*/

fn main(){
    let logical:bool = true;

    let a_float:f64 = 1.0; //regular annotation
    let an_integer = 5i32; //suffix annotation

    let default_float:f64 = 3.0;

    let mut inferred_type  = 12; //inferred type is inferred from another line;
    inferred_type = 45678909876i64;

    // a mutable variable's value can be changed but not the type
    let mut mutable = 12;
    mutable = 202;

    //Error! the type of a variable can't be changed
    mutable =true;

    // variables can be overwritten with shadowing
    let mutable = false;
    
}