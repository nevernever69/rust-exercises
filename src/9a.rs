fn tu() -> (i32, i32){
    (1,2)
    
}
fn main(){
    
    let (x, y) = tu();
    if y < 5{

        println!("less than 5");

    }
    else if y > 5{

        println!("greater than 5");

    }
    else{

        println!("equal to 5");
    }
    
    
}
