fn main(){
    let arr = vec!(10,20,30,40);
    for i in &arr{
      match i{
          30 => println!("thirty"),
        _ => println!("{:?}",i),

      }
    }
    println!("size of vector is  {}", arr.len());

}
