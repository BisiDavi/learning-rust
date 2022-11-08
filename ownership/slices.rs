/*
    slices is a type of reference
    slices doesn't have ownership, it lets you reference a part of a collection not the whole collection.
*/

fn main(){
    let mut name:String = String::from("Olubisi David");
    let word = first_word(&name);
    println!("word {}", word);
    name.clear();
    let user_name = String::from("Olatayo Amusan");
    let first_name = &user_name[0..7]; //using string slice
    let last_name = &user_name[8..user_name.len()]; 
    let slice = &user_name[..];

    println!("slice {}", slice);
    println!("first_name {}", first_name);
    println!("last_name {}", last_name);
} 

// return first word in a spaced string
fn first_word(given_word: &String) -> usize  {
    let word_array = given_word.as_bytes();

    for (i, &item) in word_array.iter().enumerate(){
        println!("&item {}", item);
        if item == b' '{
            return i;
        }
    }

    given_word.len()
}