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

    Freeing memory twice can lead to memory corruption which can result to several security vulnerablity.

    type String doesn't have a Copy trait, instead it moves.

    Note: Rust will never automatically create "deep" copies of your data

    Ways that variables and data interact: "Clone"
    Clone creates deep copy of the heap data of a String.

    Copyable Type
    1. integers (u32)
    2. Boolean (true, false);
    3. Character, char;
    4. Floating Point type
    5. Tuples (u32,u32) ✅ (u32, String) ❌

*/

fn main(){
    let s1 = String::from("hello");
    let mut s2 = s1; //s1 moving into s2, with only s2 valid.
    s2.push_str(", world!");
    println!("{}",s2);

    let b1 = String::from("Good afternoon");
    let b2 = b1.clone();

    println!("b1 = {}, b2 = {}", b1,b2);

    let x = 5; 
    let y = x; //x is copied into y, this is possible because it's on the stack not heap.

    take_ownership(s2); //s2 is valid here
    //s1 is no longer valid here
    


    make_copy(x); // x moves into the function
    //x is still valid


    println!("x = {}, y = {}",x,y);
}                                                                                                      

fn take_ownership(some_string:String){ // some_string comes into scope
    println!("{}",some_string);
} // some_string out of scope, "drop" is called, memory is freed

fn make_copy(some_integer:i32){ //some_integer is in scope
    println!("{}", some_integer);
} // some_integer is out of scope. nothing special happens