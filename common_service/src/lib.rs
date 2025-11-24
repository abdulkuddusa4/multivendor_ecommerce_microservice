pub mod jwt_utils;

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
enum LoginType{
    CUSTOMER,
    BUSINESS
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Payload{
    username: String,
    login_type: LoginType
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct NewPayload{
    username: String,
    is_active: bool,
    exp: i32,
    // user_type: i32
}

fn print_type_of<T>(obj: &T){
    println!("{}", std::any::type_name::<T>());
}

use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let payload = Payload{username: "roni".to_string(), login_type: LoginType::CUSTOMER};
        let token = jwt_utils::jwt_encode(&payload, "abc").unwrap();
        let b = jwt_utils::jwt_decode::<NewPayload>(&token, "abc");
        print_type_of(&UNIX_EPOCH);
        println!("token::> {:?}", payload);
        println!("payload:> {:?}", token);
        println!("IT IS WORKING");
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
