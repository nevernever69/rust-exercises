#[derive(Debug)]
struct User{
    user_id: i32,
    name: String,
}
fn find_user(name: &str)->Option<i32>{
    match name{
        "sam" => Some(1),
        "matt" => Some(3),
        "anyone" => Some(67),
        _ => None,
    }
    

}
fn main(){
    let user_name = "sam";
    let out = find_user(user_name).map(|user_id| User{
        user_id,
        name: user_name.to_string(),
    
    });
    match out{
        Some(out) => println!("{:?}",out),
        None => println!("user not found"),
        
    }
}





