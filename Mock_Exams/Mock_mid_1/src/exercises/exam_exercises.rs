use std::fmt::{Display, Formatter};
use std::mem::{replace, swap};

pub fn prev_str(s: &str) -> String {
    s.to_string().chars().map(
        |c| prev_char(c)


    ).collect()
}

pub fn prev_char(c: char) -> char {

    if c.is_alphabetic(){

        match c {
            'a'..='z' => (c as u8 - 1 ) as char,
            'A'..='Z' => (c as u8 - 1 ) as char,
            _ => c
        }


    }else { c }
}

//Ex2
pub struct X  {
    s: Option <String>,
    i: i32
}

impl X {
    pub fn new(s:&str, i: i32) -> X {
        X{
            s: Some(s.to_string()),
            i
        }
    }

    pub fn take_str(&mut self) -> Option<String> {
        self.s.take()
    }
}

//Ex3
pub struct NameSurname{
    pub name: String,
    pub surname: String
}

pub fn replace_surname(name_surname:  &mut NameSurname, mut new_s: String) -> String {
    //replace( &mut name_surname.surname, new_s);
    swap(&mut name_surname.surname, &mut new_s);
    new_s
}

//Ex4
#[derive(Clone)]
pub struct Student{
    pub name: String,
    pub id: i32
}

impl Student {
    pub fn new(s:&str, i: i32) -> Student {
        Student{
            name: s.to_string(),
            id: i
        }
    }
}

impl Display for Student {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"name: {}, id: {}", self.name, self.id)
    }
}

pub struct University{
    pub name: String,
    pub students: Vec<Student>
}

impl University {
    pub fn new(name:&str, s_list: &[Student]) -> University {

        //let students = s_list.into_iter().collect::<Vec<Student>>();



        University{
            name: name.to_string(),
            students: s_list.to_vec()
        }
    }
}

impl Display for University {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let students_str = self
            .students
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f,"name: {}, students: {}", self.name, students_str)
    }
}

//Ex5

pub enum AirplaneCompany {
    Boeing,
    Airbus
}

pub struct Airplane{
    pub company: AirplaneCompany,
    model: String
}
pub struct AirFleet {
    pub fleet: Vec<Airplane>
}

impl AirFleet {
    pub fn remove_boeing(&mut self){
        self.fleet.retain(|plane| match plane.company {
            AirplaneCompany::Airbus => {true}
            _ => false
        })
    }

    pub fn add_airplane(&mut self, plane: Airplane){
        self.fleet.push(plane)
    }

    pub fn search_airplane(&self, model: &String) -> Result<AirplaneCompany, String>{
        /* if self.fleet.is_empty() {
            Err("Fleet is empty".to_string())
        } else {
            for plane in self.fleet.iter() {
                if plane.model.eq(model) {
                    return match plane.company {
                        AirplaneCompany::Boeing => Ok(AirplaneCompany::Boeing),
                        AirplaneCompany::Airbus => Ok(AirplaneCompany::Airbus)
                    };
                }
            }
            Err("Model not found".to_string())
        }
        */

        let filtered = self.fleet.iter().filter(
            |plane| {
                plane.model == *model
            }
        ).collect::<Vec<_>>();

        if filtered.len() == 0 {
            Err("Model not found".to_string())
        }else {
            match filtered.get(0).unwrap().company {
                AirplaneCompany::Airbus => {Ok(AirplaneCompany::Airbus)},
                AirplaneCompany::Boeing => {Ok(AirplaneCompany::Boeing)},
            }
        }

    }
}

//Ex6




mod hashmaps {
    use std::collections::HashMap;
    use crate::exercises::exam_exercises::hashmaps::unumber::Unumber;


pub struct Map {
        pub map : HashMap<Unumber, String>
    }

    pub mod unumber {
        pub type Unumber = usize;
    }
}

pub mod test {
    use std::collections::HashMap;
    use crate::exercises::exam_exercises::hashmaps::Map;
    use crate::exercises::exam_exercises::hashmaps::unumber::Unumber;

    pub fn string_to_tuple(map: Map) -> HashMap<Unumber, (Unumber, String)> {
        map.map.iter().map(
            |(u,s)| (*u,(*u,String::from(s)))
        ).collect()
    }
}



//Ex7

pub struct Size {
    pub width: u32,
    pub height: u32
}

impl Size {
    pub fn new(w: u32, h:u32) -> Size {
        Size{
            width: w,
            height: h
        }
    }

    pub fn area(&self) -> u32{
        self.width * self.height
    }

    pub fn compare(&self, other: &Size) -> Option<bool>{
        if self.area() == other.area() {
            None
        }else if self.area() > other.area(){
            Some(true)
        }else { Some(false) }
    }
}

//EX10
pub fn order(strings: Vec<String>) -> Vec<String>{
    strings.iter().enumerate().map(|(i,s)|{
        format!("{} - {}",i,s)
    }).collect()
}

//EX8
pub struct MaybePoint {
    pub x: Option<i32>,
    pub y: Option<i32>
}

impl MaybePoint {
    pub fn new(x: Option<i32>, y:Option<i32>) -> MaybePoint{
        MaybePoint{
            x,
            y
        }
    }

    pub fn is_some(&self) -> bool {
        match (self.x, self.y) {
            (Some(_),Some(_)) => true,
            _ => false
        }
    }

    pub fn maybe_len(&self) -> Option<f32>{
        if self.is_some(){
            Some(((self.x.unwrap().pow(2) + self.y.unwrap().pow(2)) as f32).sqrt())
        }
        else { None }
    }
}

//EX9
pub fn rs1(n: i32) ->Result<i32, String> {
    if n % 10 == 0 {
        Ok(n)
    }
    else { Err("error".to_string()) }
}

pub fn rs2(n: i32) ->Result<i32, String> {
    if n % 5 == 0 {
        Ok(n)
    }
    else { Err("error".to_string()) }
}

pub fn wrapper(n: i32) -> Result<i32, String> {
    match (rs1(n), rs2(n)) {
        ((Ok(n)), (Ok(_))) => Ok(n),
        _ => Err("error".to_string())
    }
}

//Ex11
mod modx {
    pub enum X{
        S(char),
        C(String),
        F((f64, usize))
    }
}

mod mody {
    pub enum X{
        S(char),
        C(String),
        F((f64, usize))
    }
}

mod mod_sum{
    use super::mody;
    use super::modx;
    pub fn sum(x: modx::X, y: mody::X) -> f64{
        todo!()
    }
}



pub struct Student {
    pub name: String,
    pub id: u32
}

impl Student {
    pub fn new(name: &str, id: u32) -> Student{
        Student {
            name: name.to_string(),
            id
        }
    }
}

impl Display for Student {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {}",self.name,self.id)
    }
}