use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Car{
    model: String,
    year: i32,
    rent: bool,
    price: i32
}

pub struct CarDealer{
    cars: Vec<Rc<RefCell<Car>>>
}

pub struct User{
    pub(crate) car: Option<Rc<RefCell<Car>>>,
}

impl CarDealer {
    pub fn new(cars: Vec<Rc<RefCell<Car>>>) -> Self {
        Self { cars: cars }
    }

    pub fn add_car(&mut self, car: Rc<RefCell<Car>>){
        self.cars.push(car);
    }

    pub fn print_cars(&self){
        for car in self.cars.iter(){
            println!("{:?}", car);
        }
    }

    pub fn rent_user(&mut self, user: &mut User, model: &str){
        match self.cars.iter().position(|c| c.borrow().model == model) {
            None => {println!("car not found")},
            Some(pos) => {
                self.cars[pos].borrow_mut().rent = true;
                user.car = Some(Rc::clone(&self.cars[pos].clone()));
            }
        }
    }

    pub fn end_rent (&self, user: &mut User){
        match &user.car {
            None => {println!("User has no car")},
            Some(c)  => {
                c.borrow_mut().rent = false;
                user.car = None;
            },
        }
    }


}

impl User{
    pub fn print_car(&self){
        match &self.car
        {
            None => {println!("user has no car")},
            Some(c)  => {println!("{:?}",c.borrow())},
        }
    }
}
impl Car{
    pub fn new(model: &str, year: i32, price: i32 ) -> Car{
        Car{
            model: model.to_string(),
            year,
            price,
            rent: false
        }
    }
}