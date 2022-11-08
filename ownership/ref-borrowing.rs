/*
    using '&' to make reference to a variable prevent ownership
    '&' = reference
    reference refers to a variable, but doesn't own that variable

    you can't modify a reference(&), except a mutable reference (&mut)

    Note: 
    You can only have one mutable reference (&mut) to a particular piece of data in a particular scope
    
    This is prevents data racing.

    Data racing occurs when all this behaviour occur:
    1. Two or more pointers access the same data at the same time.
    2. At least one of the pointers is being used to write to the data.
    3. There's no mechanism being used to synchronize access to the data.

    Note: 
    You can't combine mutable and immutable references

    Dangling Reference:
    It's a pointer referencing a location in memory that has been allocated to another pointer.
    Rust ensure it catches Dangling Reference at compile time
*/

fn main(){
    let mut s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);
    make_change(&mut s1); 
    // dont_do_this();
    do_this();
    // _dont_do_this_2();

    let reference_to_nothing = dangle();
}

fn calculate_length(s:&String) -> usize{
    s.len()
}

fn make_change(some_string: &mut String){
    some_string.push_str(", world");
    println!("some_string {}", some_string);
}

fn _dont_do_this(){
    let mut d1 = String::from("Hello");
    let r1 = &mut d1;
    // let r2 = &mut d1; //❌, you can borrow a particular variable twice in same scope

    println!("{}", r1);
    // println!("{}, {}", r1,r2);
}


fn do_this(){
    let mut d1 = String::from("Hello");
    {
        let r1 = &mut d1;
        println!("{} r1", r1);

    }
    let r2 = &mut d1; //❌, you can borrow a particular variable twice in same scope

    println!("{} r2",r2);
}

// combine both mutable reference and reference of a variable in a scope
fn _dont_do_this_2(){
    let mut d = String::from("Hello");
    let r1 = &d;
    let r2 = &d; // reference
    // let r3 = &mut d; // mutable reference

    println!("{}, {}", r1,r2);
    // println!("{}, {}, {}", r1,r2,r3);
}

fn dangle()-> &String{
    let s = String::from("Hello David");

    &s //an error is returned because we'll be making reference to s when out of scope.
}