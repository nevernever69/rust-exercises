enum Flavour{
    Stawberry,
    Coke,
    Lemon,
}
struct Drink{
    flavour: Flavour,
    ounce: i32
}


fn main(){

    let first = Drink{
        flavour: Flavour::Lemon,
        ounce: 12
    };
    match first.flavour{
    Flavour::Coke => println!("coke"),
    Flavour::Stawberry=> println!("Stawberry"),
    Flavour::Lemon=> println!("Lemon"),

    
    }


}
