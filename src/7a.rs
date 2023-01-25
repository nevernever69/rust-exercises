enum Color{
    Red,
    Blue,
    Green,
    Black,
}
fn printcolor(ans: Color){
    match ans{
        Color::Red => println!("Red"),
        Color::Blue=> println!("Blue"),
        Color::Black=> println!("Black"),
        Color::Green=> println!("Green"),
        _ => println!("don't know what color is this"),

    }



}
fn main(){
    let color = Color::Green;
    printcolor(color);
        
    


}
