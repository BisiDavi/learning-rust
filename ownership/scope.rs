/*
    variable ownership
    1. Assigning a value to another variable moves it.
    2. When a variable that has data on goes out of scope, the value will be drop 
*/

fn main(){
    let s1 = gives_ownership();
    let s2 = String::from("Hackathon");
    let s3 = take_and_gives_back(s2);

    
    let (s4, len) = calculate_length(s1);
    
    println!("s3={}, s4={}, length={}",s3, s4, len);
}

fn gives_ownership() -> String{ // move return value into calling function scope.
    let some_string = String::from("Hello"); //comes into scope
    some_string //returned and moves into calling function
}

fn take_and_gives_back(a_string:String) -> String{
    a_string
}

fn calculate_length(s:String) ->(String , usize){
    let length = s.len();

    (s, length)
}