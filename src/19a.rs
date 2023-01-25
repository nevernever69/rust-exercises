use std::collections::HashMap;
fn main(){
    let mut map = HashMap::new();
    map.insert("chairs",5);
    map.insert("Beds",3);
    map.insert("Tables",2);
    map.insert("couches",0);
    for (furn,qu) in &map{
        print!("{furn} ");
        if qu == &0 {
            println!("out of stock");
        
        }else{

            println!("{}",qu);
        }
        
    }
    println!("total number of item in {:?}",map.len());



}
