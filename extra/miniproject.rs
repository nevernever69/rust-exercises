use std::io;

#[derive(Debug, Clone)]
pub struct Bill{
    name: String,
    amount: f64,
        
}
pub struct Bills{
    inner: Vec<Bill>

}

impl Bills{
    fn new() -> Self{
        Self{
            inner: vec![]
        }
    

    }
    fn add(&mut self, bill: Bill){
        self.inner.push(bill);
    }
    fn get_all(&self) -> Vec<&Bill>{
        self.inner.iter().collect()
    }
    
}

fn input() -> Option<String>{
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err(){
        println!("please enter data again");
    }
    let input = buffer.trim().to_owned();
    if &input == ""{
        None
    }else {
        Some(input)
    }

}
mod menu{
    pub fn add_bill(bills: &mut Bills){
        println!("Bill name");
        let name = match get_input(){
            Some(input) => input,
            None => return,
                
        };
        let amount = match get_input(){
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill {name, amount};
        bills.add(bill);
        println!("Bill added");

    

    }

}
enum MainMenu{
    Addbill,
    Viewbill
}

impl MainMenu{
    fn from_str(string: &str) -> Option<MainMenu>{
        match string{
            "1" => Some(Self::Addbill),
            "2" => Some(Self::Viewbill),
            _ => None,
        }
    }
    fn show(){
        println!("");
        println!("==ADDBILL==");
        println!("1. Add bill");
        println!("2.View bill");
        println!("");
        println!("Enter Selection: ");
    }

}



fn main(){
    loop{
        MainMenu::show();
        let inp = input().expect("no input found");
        match MainMenu::from_str(inp.as_str()){
            Some(MainMenu::Addbill) => (),
            Some(MainMenu::Viewbill) => (),
            None => return,

        
        }
    }

        

}

