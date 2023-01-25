fn check(cond: bool){
    match cond{
        true => println!("it's big"),
        false => println!("too small"),
    }
    
}
fn main(){
    let val = 101;
    let cond = val > 100;
    check(cond);
    
    

}
