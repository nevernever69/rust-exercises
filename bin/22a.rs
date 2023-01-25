fn clamp(n: i32, lower: i32, upper: i32) -> i32{
    if n < lower{
        lower
    }
    else if n > upper{
        upper

    }else{
        n

    }
}
fn div(a: i32, b: i32) -> Option<i32>{
        Some(a/b)

}
fn concat(first: &str, second: &str)-> String{
    format!("{}{}",first, second)
    
}
fn main(){
    use crate::*;
    #[test]
    fn clamp(){
        let result = clamp(100, 80, 150);
        let expected = 100;
        assert_eq!(result, expected);

    }
}
