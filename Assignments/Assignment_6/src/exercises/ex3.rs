use std::cell::RefCell;
use std::fmt::format;
use std::rc::Rc;

type FarmCellRef = Rc<RefCell<FarmCell>>;

pub trait Sound{
    fn make_sound(&self) -> String;
}
pub struct FarmCell{
    element: Box<dyn Sound>,
    next: Option<FarmCellRef>
}

impl FarmCell{
    pub fn new(element: Box<dyn Sound>) -> FarmCell{
        Self{element, next: None}
    }

    pub fn insert(&mut self, farm_cell: FarmCellRef){
        match self.next {
            None => {self.next = Some(farm_cell);},
            Some(ref e) => e.borrow_mut().insert(farm_cell)
        }
    }
}

impl Sound for FarmCell{
    fn make_sound(&self) -> String {
        let mut s = self.element.make_sound();
        if let Some(ref next) = self.next {
            s.push_str(&format!(" {}", next.borrow().make_sound()))
        }
        s
    }


}

pub struct Dog{}
impl Sound for Dog{
    fn make_sound(&self) -> String {
        String::from("Bau Bau")
    }
}

pub struct Cat{}
impl Sound for Cat{
    fn make_sound(&self) -> String {
        String::from("Meow")
    }
}


pub struct Cow{}
impl Sound for Cow{
    fn make_sound(&self) -> String {
        String::from("MOOOOH")
    }
}

