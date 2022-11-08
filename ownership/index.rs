/*
    Ownership Rules
    1. Each value in Rust has a variable that's called it's *owner*.
    2. There can be only one owner at a time.
    3. When the owner goes out of scope,the value will be dropped.

    Rust doesn't use Garbage Collector (GC), Rust manage memory usage by ensuring once a variable is in scope it's valid,
    but when not in scope, the memory is returned to the operating system.

    for example 
    {  
        let s=String::from("Good morning 9ja"); //s is valid from here onward

    }// s is no longer valid after this point.

    the braces "{}" acts as the scope.
    There's global scope and function scope.

    Rust calls "drop" at the end of the curly bracket "}"                     
    
    A typical String in a Heap has:
    1. pointer
    2. length
    3. capacity

*/

fn main(){
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}",s);
}