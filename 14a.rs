struct Person{
    age: i32,
    name: String,
    color: String,
}
impl Person{
    fn new(age: i32, name: String, color: String)-> Person{
       Person{
            age,
            name,
            color,  
        }
        
    }

}
fn print(st: &str){
            
        println!("{}",st);


}

fn main(){
    let vector = vec![Person::new(12, "ashish".to_string(), "blue".to_string()), Person::new(13, "nothing".to_string(),"red".to_string()), Person::new(15, "anything".to_string(),"green".to_string())];
    for person in vector{
        if person.age > 10 {
            print(&person.name);
            print(&person.color);

    
        }
    
    }


}
