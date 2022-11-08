/*
    using structs to structure data, like interface in typescript.
*/

fn main(){

    struct User {
        username:String,
        email:String,
        sign_in_count:u64,
        active:bool
    }

    //all fields are either mutable or none

    let mut user1 = User {
        email: String::from("oludavidconnect@gmail.com"),
        username:String::from("oludavid"),
        active:true,
        sign_in_count:1
    }

    let user2 = User {
        email: String::from("cole@gmail.com"),
        username: String::from("cole"),
        ..user1
        // active:user1.active,
        // sign_in_count: user1.sign_in_count
    }

    user1.email = String::from("readydevfreelancer@gmail.com");

    fn build_user(email:String, username:String) -> User{
        User {
            email,
            username,
            active:true,
            sign_in_count:1
        }
    }

    //tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}