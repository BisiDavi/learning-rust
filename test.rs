
fn reverse(pair : (i32, bool)) -> (bool, i32){
    let (int_param, bool_param) = pair;
   return (bool_param,int_param);
}

fn main(){
 println!("true AND false is {}", true && false);
 println!("true OR false is {}", true || false);
 println!("not false is {}", !false);

 let tuple_of_tuples = ((1u8,22u8,-4i8),(10u8,16u8), -25i8);
 println!("tuple of tuples: {:?}",tuple_of_tuples);

 let tuple_1 = (-56, false);
 println!("reverse(tuple_1): {:?}",reverse(tuple_1));
} 