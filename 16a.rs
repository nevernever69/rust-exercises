//to create a student struct and have name and locker, locker is of type option<i32> 
struct Student{
    name: String,
    locker: Option<i32>
}

fn main(){
    let data = Student{
        name: "ashish".to_string(),
        locker: Some(1),
    };
    let dev = Student{
        name: "nothing".to_string(),
        locker: None
    };
    match dev.locker{
        Some(value) => println!("{}", value),
        _ => println!("no value assigned"),
    };
    match data.locker{
        Some(value) => println!("{}", value),
        _ => println!("no value assigned"),
    };

}

