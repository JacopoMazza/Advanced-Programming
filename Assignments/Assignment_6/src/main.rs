mod exercises;

use std::cell::RefCell;
use std::rc::Rc;
use exercises::*;
use crate::exercises::ex2::{Car, CarDealer, User};
use crate::exercises::ex3::{Cat, Cow, FarmCell, Sound};
use crate::exercises::ex6::EntangledBit;

fn main() {
    /*let nodes = [3,5,6,2,8];
    test_ex_1(&nodes);*/

    //test_ex_2()

    //test_ex_3()

    test_ex_6()
}


fn test_ex_1<T>(vec: &[T])
where
    T:std::fmt::Debug + PartialOrd + Clone
{
        let tree = exercises::ex1::TreeNode::from_vec(vec);
        //tree.in_order();
        //tree.pre_order();
        tree.post_order();
}

fn test_ex_2(){

    let c1 = exercises::ex2::Car::new("Audi", 2025, 60000);
    let c2 = exercises::ex2::Car::new("Toyota", 2021, 5400);
    let c3 = exercises::ex2::Car::new("Ferrari", 2024, 120000);
    let c4 = exercises::ex2::Car::new("Pagani", 2021, 150000);
    let cars: Vec<Rc<RefCell<Car>>> = vec![Rc::new(RefCell::new(c1)),Rc::new(RefCell::new(c2)),Rc::new(RefCell::new(c3))];



    let mut u1 = User{car: None};
    let mut dealer = CarDealer::new(cars);

    dealer.add_car(Rc::new(RefCell::new(c4)));
    dealer.rent_user(&mut u1, "Audi");

    dealer.print_cars();
    u1.print_car();

    dealer.end_rent(&mut u1);
    dealer.print_cars();
    u1.print_car();

}


fn test_ex_3(){

    let d = exercises::ex3::Dog{};
    let c = Cat{};
    let cow = Cow{};

    let mut farm = FarmCell::new(Box::new(d));
    farm.insert(Rc::new(RefCell::new(FarmCell::new(Box::new(c)))));
    farm.insert(Rc::new(RefCell::new(FarmCell::new(Box::new(cow)))));

    println!("{}",farm.make_sound());

}


fn test_ex_6(){
    let b1 = EntangledBit::default();
    let mut b2 = EntangledBit::default();
    let mut b3 = EntangledBit::default();

    b1.set();
    b2.set();
    println!("{}, {}",b1.get(), b2.get());

    b1.entangle_with(&mut b3);
    println!("{}, {}",b1.get(), b3.get());

    b2.reset();
    b2.entangle_with(&mut b3);
    println!("{}, {}, {}",b1.get(), b2.get(), b3.get());

}