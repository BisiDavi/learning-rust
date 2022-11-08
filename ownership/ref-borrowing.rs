/*
    using '&' to make reference to a variable prevent ownership
    '&' = reference
    reference refers to a variable, but doesn't own that variable

    you can't modify a reference(&), except a mutable reference (&mut)
*/

fn main(){
    let mut s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);
    make_change(&mut s1); 
}

fn calculate_length(s:String) -> usize{
    s.len()
}

fn make_change(some_string: &mut String){
    some_string.push_str(", world");
    println!("some_string {}", some_string);
}