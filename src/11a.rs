struct Grocery{
    Quantity: i32,
    Item: i32,
}
fn ptq(qu: &Grocery){
    println!("{}",qu.Quantity);


}
fn pti(it: &Grocery){
    println!("{}", it.Item);
    
}
fn main(){

    let first = Grocery{
        Quantity: 12,
        Item:  10

    };
    ptq(&first);
    pti(&first);


}

