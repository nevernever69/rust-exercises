use std::io;
enum State{
    Reboot,
    Sleep,
    Off,
    Shutdown,
    Hibernate,
    Nothing
}
impl State{
    fn new(ans: &str)->Option<State>{
        let ans = ans.trim().to_lowercase();
        match ans.as_str(){
            "reboot" => Some(State::Reboot),
            "sleep" => Some(State::Sleep),
            "off" => Some(State::Off),
            "shutdown" => Some(State::Shutdown),
            "hibernate" => Some(State::Hibernate),
            _ => None,
        }

    }

}
fn check(ans: &State){
    use State::*;
    match ans{
        Reboot => println!("Rebooting"),
        Sleep => println!("sleeping down"),
        Off => println!("permament offing the system"),
        Shutdown => println!("shuting down"),
        Hibernate => println!("hibernating"),
        _ => println!("inavalid state"),

    }


}
fn main(){

    let mut buffer = String::new();
    let input = io::stdin().read_line(&mut buffer);
    if input.is_ok(){
        match State::new(&buffer){
            Some(state) =>  check(&state),
            None => println!("invalid  power state"),

        }

    }else{
        println!("error readin input")
    
    }





}
