mod calculator_model;
use calculator_model::model;
fn main() {
    // match model::parse_to_tree("5.5+5.5*2"){
    //     Ok(_) => (),
    //     Err(e) => println!("{}",e)
    // }
    match model::test(){
        Ok(_) => (),
        Err(e) => print!("{}",e)
    }
}
