enum Ticket{
    Backstage(i32, String),
    Vip(i32, String),
    Standard(i32)
}
fn main(){
    let vector = vec![Ticket::Backstage(150, "anything".to_string()), Ticket::Vip(100, "nothing".to_string()), Ticket::Standard(30)];

    for num in vector{
        match num{
            Ticket::Backstage(price, name) => {
                println!("Backstage price is {} and name is {}", price, name);
        
            }
            Ticket::Standard(price) => {
                println!("Standard price is {}", price);
    
            }
            Ticket::Vip(price, name) => {
                println!("Vip price is {}, name is {}", price, name);
    
            }

        };
    

    }


}
