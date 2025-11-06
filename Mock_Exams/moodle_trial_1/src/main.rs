use std::fmt::Display;
use std::fmt::Formatter;

fn main() {
    #[derive(Clone)]
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
            write!(f,"Student {{ name: \"{}\", id: {} }}",self.name,self.id)
        }
    }

    pub struct University {
        pub name: String,
        pub students: Vec<Student>
    }

    impl University {
        pub fn new(name: &str, s_list: &[Student]) -> University{
            University {
                name: name.to_string(),
                students: s_list.to_vec()
            }
        }

        pub fn remove_student(&mut self, id:u32) -> Result<Student,String>{
            if let Some(pos) = self.students.iter().position(
                |s| s.id == id
            )
            {
                Ok(self.students.remove(pos))
            }
            else {
                Err("Student not found".to_string())
            }
        }
    }

    impl Display for University {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

            let s_string = self.students.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ");
            let ret_string = format!("Students: [{}]",s_string);
            write!(f,"{}\n{}",self.name, ret_string)
        }
    }
}








pub enum AirplaneCompany {
    Boeing,
    Airbus
}

pub struct Airplane {
    pub model: String,
    pub company: AirplaneCompany
}

pub struct AirFleet {
    pub fleet: Vec<Airplane>,
}

impl AirFleet {
    pub fn remove_boeing(&mut self){
        self.fleet.retain(
            |f| {
                match f.company {
                    AirplaneCompany::Boeing => false,
                    _ => true
                }
            }
        )
    }

    pub fn add_airplane(&mut self, airplane: Airplane ){
        self.fleet.push(airplane);
    }

    pub fn search_airplane(&self, model:&str) -> Result<AirplaneCompany,String>{
        if let Some(pos) = self.fleet.iter().position(
            |f| f.model == model
        ){
            Ok(match self.fleet.get(pos).unwrap().company {
                AirplaneCompany::Airbus => AirplaneCompany::Airbus,
                AirplaneCompany::Boeing => AirplaneCompany::Boeing
            })
        }
        else {
            Err("Not in this fleet".to_string())
        }
    }
}



/////////

pub mod hashmaps {

    use std::collections::HashMap;
    use unumber::Unumber;


    pub mod unumber {
        pub type Unumber = usize;
    }


    pub struct Maps {
        pub map: HashMap<Unumber, String>
    }
}

pub mod test {
    use std::collections::HashMap;
    use super::hashmaps::unumber::Unumber;
    use super::hashmaps::Maps;
    pub fn string_to_tuple(map: Maps) -> HashMap<Unumber, (Unumber,String)>{
        map.map.iter().map(
        |(n,string)| {
            (*n,(string.len(), string.to_string()))
        }
        ).collect()
    }
}

pub fn order(strings: Vec<String>) -> Vec<String> {
    strings.iter().enumerate().map(|(i,s)| format!("{} - {}", i.to_string(),s.to_string())).collect()
}


///////

pub struct Size {
    pub width: f32,
    pub height: f32
}

impl Size {
    pub fn new(w:f32,h:f32) -> Size {
        Size {
            width: w,
            height: h
        }
    }

    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    pub fn compare(&self, other: &Size) -> Option<bool>{
        if self.area() == other.area() {
            None
        }
        else if self.area()  > other.area() {
            Some(true)
        }
        else { Some(false) }
    }
}

pub struct NameSurname {
    pub name: String,
    pub surname: String
}

pub fn replace_surname(mut ns: NameSurname,mut new_s: String) -> String{
    std::mem::swap(&mut ns.surname, &mut new_s);
    new_s
}

////////
pub struct X {
    pub s: Option<String>,
    pub i: i32
}

impl X {
    pub fn new(s:&str, i:i32) -> X {
        X {
            s: Some(s.to_string()),
            i
        }
    }

    pub fn take_str(&mut self) -> Option<String> {
        self.s.take()
    }
}

/////
pub fn prev_str(s: &str) -> String {
    s.to_string().chars().map(
        |c| prev_char(c)


    ).collect()
}

pub fn prev_char(c: char) -> char {

    if c.is_alphabetic(){

        match c {
            'a'..='z' =>  if  c == 'a' {return 'a' } else {(c as u8 - 1 ) as char},
            'A'..='Z' => if  c == 'A' {return 'A' } else {(c as u8 - 1 ) as char},
            _ => c
        }


    }else { c }
}