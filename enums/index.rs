/*
    enumerations also known as enums
    enums allow you to define a type by enumerating its possible values
*/

fn main(){
    enum IpAddrKind{
        V4, V6
    }

    enum Message {
        Quit,
        Move {x:i32, y:i32},
        Write(String),
        ChangeColor(i32, i32, i32)
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind){
    println!("Ip-Kind, {}", ip_kind);
}