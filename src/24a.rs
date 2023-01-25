fn main(){

    let mut data = vec![1,2,3,4,5];
    let new_arr: Vec<_> = data.iter().map(|num| num * 3).filter(|num| num > &10).collect();
    for i in new_arr{
        println!("{}", i);
    }

}
