struct Customer{
    age: i32,
}
fn purchase(customer: &Customer) -> Result<(), String>{
    if customer.age < 21{
        Err("age is not sufficient".to_owned())
        
    }else{
        Ok(())
    
    }

    

}
fn main(){
    let any  = Customer{age:12};
    let purchased = purchase(&any);
    println!("{:?}",purchased);
    


}
