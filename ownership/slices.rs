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
    let bn = [1,2,3,4,5];
    let bn_slice = &bn[1..4];

    println!("bn_slice {:?}", bn_slice);
    println!("slice {}", slice);
    println!("first_name {}", first_name);
    println!("last_name {}", last_name);
} 

// return first word in a spaced string
fn first_word(given_word: &str) -> &str  {
    let word_array = given_word.as_bytes();

    for (i, &item) in word_array.iter().enumerate(){
        if item == b' '{
            return &given_word[0..i];
        }
    }

   &given_word[..]
}