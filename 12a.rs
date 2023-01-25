enum Color{
    Blue,
    Black, 
    Green,

}
impl Color{
    fn print(&self){
        print!("color is ");
        match self{
            Color::Blue => println!("blue"),
            Color::Black => println!("black"),
            Color::Green=> println!("Green"),
        }

    }
    
}
struct Dimension{
    width: i32,
    height: i32,
    depth: i32,
}
impl Dimension{
    fn print(&self){
            println!("height is  {}",self.height);
            println!("widht is {}",self.width);
            println!("depth is {}",self.depth);
        
    }
    
}
struct Box{
    dimension: Dimension,
    weight: i32,
    color: Color,
}
impl Box{
    fn new(dimension: Dimension, weight: i32, color: Color)-> Self{
       Self{
         dimension,
        weight,
         color,
       }
    }
    fn print(&self){
        self.color.print();
        self.dimension.print();
        println!(" weight is{}",self.weight);
    
    }
    
}

fn main(){
    let dim = Dimension{
        width: 12,
        height: 14,
        depth: 15,
    };
    let co = Color::Blue;
    
    let ans = Box::new(dim,100,co);
    ans.print();


}
