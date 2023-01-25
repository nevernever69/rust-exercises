enum Position{
    Maintenance,
    Marketing,
    Managers,
    Linesupervisor,
    Kitcherstaff,
    Assemblytech,
}
enum Status{
    Active,
    Terminated,
}
struct Employee{
    position: Position,
   status: Status

}
fn try_access(employee: &Employee)->Result<(),String>{
    match employee.status{
        Status::Terminated => return Err("terminated".to_owned()),
        _ => (),
    }
    match employee.position{
        Position::Maintenance => Ok(()),
        Position::Managers=> Ok(()),
        Position::Marketing=> Ok(()),
        _ => Err("invalid position".to_owned()),

        
    }
    

}
fn print_access(employee: &Employee)-> Result<(),String>{
    let attempt = try_access(employee)?;
    println!("access ok");
    Ok(())

}

fn main(){

        let manager = Employee{
            position: Position::Managers,
           status: Status::Active,

        };
        match print_access(&manager){
            Err(e) => println!("access denied {:?}",e),
            _ => (),

        }


}

