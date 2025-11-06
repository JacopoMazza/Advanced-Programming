use std::collections::HashMap;
use crate::exercises::assignment_exercises::{lucky_slice, sum_areas, Actions, Auth, Chair, GetArea, Permission, Role};
use crate::exercises::assignment_exercises as ex;
mod exercises;

fn main() {
    //test_ex1();
    //test_lucky_slice("giraffe");
    //test_ex6();
    //test_ex8();
    test_ex9();
}

fn test_ex1(){

    let s1 = "jacopo";
    let s2 = "Opossum";
    //let s2 = "ajax";
    let res = crate::exercises::assignment_exercises::find_equal(s1,s2);
    println!("{:?}",res);
}

fn test_lucky_slice(input: &str){
    let res = lucky_slice(input);
    println!("{:?}",res);

}

fn test_ex6(){
    let p = ex::Point::default();
    let rect = ex::Rectangle::default();
    let circle = ex::Circle::default();

    //println!("Area of p: {:?}\nArea of rect: {:?}\nArea of circle:{:?}",p.get_area(),rect.get_area(),circle.get_area());
    let areas: [&dyn GetArea; 3] = [&p, &rect, &circle];

    println!("{:?}", sum_areas(&areas));

}

fn test_ex8(){

    let chair = ex::Chair{color: "red", quantity: &(10usize)};
    let chair2 = ex::Chair{color: "blue", quantity: &(1usize)};
    let chair3 = ex::Chair{color: "black", quantity: &(0usize)};

    let w1 = ex::Wardrobe{color: "blue", quantity: &(20usize)};
    let w2 = ex::Wardrobe{color: "red", quantity: &(1usize)};
    let w3 = ex::Wardrobe{color: "black", quantity: &(0usize)};
    
    println!("{}",chair);
    println!("{}",chair2);
    println!("{}",chair3);
    println!("{}",w1);
    println!("{}",w2);
    println!("{}",w3);
}


fn test_ex9(){


    let a1 = Actions {
        action: "run".to_string(),
        permission: HashMap::from([
            (Permission::WRITE, true),
            (Permission::EXECUTE, false),
            (Permission::READ, true),
        ])
    };


    let a2 = Actions {
        action: "read file".to_string(),
        permission: HashMap::from([
            (Permission::WRITE, false),
            (Permission::EXECUTE, false),
            (Permission::READ, true),
        ])
    };

    let mut u1 = ex::User::default();
    let mut u2 = ex::User{
        name: "Jacopo".to_string(),
        role: Role::ADMIN,
        actions: Vec::from([a1,a2])
    };

    println!("{}",u1.check_permission(&Permission::WRITE,"run"));
    println!("{}",u2.check_permission(&Permission::WRITE,"read file"));

    match u1.change_role(Role::USER) {
        Ok(_) => println!("Changed role for u1"),
        Err(e) => println!("{e}"),
    }

    let new_role = Role::GUEST;
    match u2.change_role(new_role) {
        Ok(_) => println!("u2 is now a Guest"),
        Err(e) => println!("{e}"),
    }

}