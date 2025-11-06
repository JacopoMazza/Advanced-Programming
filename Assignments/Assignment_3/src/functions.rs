
/*
Exercise 2
 */
use std::collections::HashMap;

pub enum Fuel{
    Diesel,
    Gasoline,
    LPG,
    Methan,
    Electic
}


pub enum IP{
    v4([u8; 4]),
    v6([u16; 8]),
}

pub struct Point3d {
    x: f64,
    y: f64,
    z: f64,
}


pub struct CarOwner{
    pub name: String,
    pub surname: String,
    pub age: u8
}

/*pub fn recognise_owner(carData: &HashMap<String,String>, plate: &String) -> Option<&String>{
    match carData.get(plate){
        Some(CarOwner) => {Some(*CarOwner)}
        _ => {None}
    }
}*/





