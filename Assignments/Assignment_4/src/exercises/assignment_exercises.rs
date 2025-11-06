use std::collections::{HashMap, HashSet, LinkedList};
use std::f32::consts::PI;
use std::fmt::Display;
use std::ops::{Add, Sub};

//EX1
pub fn find_equal <'a, 'b> (s1: &'a str, s2: &'b str) -> Option<(&'a str, &'b str)> {

    for sub1_start in 0..(s1.len() - 1) {

        let sub1_end = sub1_start + 2;
        let sub1 = &s1[sub1_start..sub1_end];

        for sub2_start in 0..(s2.len() - 1){

            let sub2_end = sub2_start + 2;
            let sub2 = &s2[sub2_start..sub2_end];

            if sub1 == sub2 { return Some((sub1, sub2)) }

        }
    }
    None

}

fn random_letter() -> char{
    let mut n = rand::random::<u8>();
    n = n % 26 + 'a' as u8;
    n as char
}

pub fn lucky_slice(input_str: &str) -> Option<&str> {

    let mut other_str = String::with_capacity(input_str.len());
    for _ in 0..input_str.len() {
        other_str.push(random_letter());
    }

    println!("{}", other_str);

    match find_equal(input_str, &other_str) {
        Some((s1, s2)) => Some(s1),
        None => None
    }


}


//EX2
pub struct Person <'a> {
    pub name: String,
    pub mother: Option<&'a Person<'a>>,
    pub father: Option<&'a Person<'a>>,

}

impl <'a> Person<'a> {
    pub fn new(name: &str, f: Option<&'a Person>, m: Option<&'a Person>) -> Person<'a> {
        Person{
            name: name.to_string(),
            mother: m,
            father: f,
        }
    }

    pub fn find_relatives(&self, generations: u32) -> Vec<&Self> {

        let mut relatives: Vec<&Self> = Vec::new();
        self.find_relatives_helper(generations, &mut relatives);
        relatives

    }

    fn find_relatives_helper <'b> (&'b self, generations: u32, relatives:  &mut Vec<&'b Self> ) {

        if generations == 0 {relatives.push(self);}

        else if generations > 0 {
            if let Some(m) = self.mother {
                m.find_relatives_helper(generations - 1, relatives);
            }
            if let Some(f) = self.father {
                f.find_relatives_helper(generations - 1, relatives);
            }
        }
    }


    pub fn find_roots(&self) -> Vec<&Self> {

        let mut roots: Vec<&Self> = Vec::new();
        self.find_roots_private(&mut roots);
        roots

    }

    fn find_roots_private<'b>(&'b self, roots: &mut Vec<&'b Self>){

        match (self.mother, self.father) {

            (Some(m), Some(f)) => {
                m.find_roots_private(roots);
                f.find_roots_private(roots);
            },
            (Some(m), None) => {
                roots.push(self);
                m.find_roots_private(roots);
            }
            (None, Some(f)) => {
                roots.push(self);
                f.find_roots_private(roots);
            }
            (None, None) => {
                roots.push(self);
            }
        }

    }
}

//EX4

struct DoubleRef<'r,'s: 'r, T> {
    r: &'r T,
    s: &'s T
}


//EX 5
pub trait Split<'a> {
    type ReturnType;
    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType);

}

impl<'a> Split<'a> for String{
    type ReturnType = &'a str;
    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType ) {
        self.split_at(self.len() / 2)
    }
}

impl<'a> Split<'a> for &'a[i32]{
    type ReturnType = &'a [i32];
    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        self.split_at(self.len() / 2)
    }
}

impl<'a> Split<'a> for LinkedList<f64>{
    type ReturnType =  LinkedList<f64>;

    fn split(&'a self) -> ( Self::ReturnType, Self::ReturnType ) {

        let mut l1: LinkedList<f64> = LinkedList::new();
        let mut l2: LinkedList<f64> = LinkedList::new();

        for (i,item) in self.iter().enumerate() {
            if i < (self.len() / 2) {
                l1.push_front(*item);
            }
            else {
                l2.push_front(*item);
            }
        }
       

        (l1, l2)
    }
}


//EX 7

pub fn skip_prefix <'a: 'b, 'b>(telephone_number: &'a str, prefix: &'b str) -> &'b str {
    let number = &telephone_number[prefix.len()..];

    if telephone_number.starts_with(prefix) {
        number
    }
    else {telephone_number}
}

//Ex6
pub struct Point{
    pub x: i32,
    pub y: i32
}

pub struct Circle{
    pub center: Point,
    pub radius: i32
}

pub struct Rectangle{
    top_left: Point,
    bottom_right: Point
}

#[derive(Copy,Clone, Debug)]
pub struct Area {
    area: f32
}

impl Default for Circle{
    fn default() -> Self {
        Self{
            center: Point::default(),
            radius: 1
        }
    }
}

impl Default for Point{
    fn default() -> Self {
        Self{
            x : 0,
            y : 0,
        }
    }
}

impl Default for Rectangle{
    fn default() -> Self {
        Rectangle{
            top_left: Point{
                x : -1,
                y : 1,
            },
            bottom_right: Point{
                x : 1,
                y : -1,
            }
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub trait GetArea {
    fn get_area(&self) -> Area;
}

impl Default for Area {
    fn default() -> Self {
        Self {area: 0.}
    }
}

impl GetArea for Point {
    fn get_area(&self) -> Area {
        Area::default()
    }
}

impl GetArea for Circle {
    fn get_area(&self) -> Area {
        Area{
            area : PI*(self.radius as f32).powi(2)
        }
    }
}

impl GetArea for Rectangle {
    fn get_area(&self) -> Area {

        let width = (self.top_left.x - self.bottom_right.x).abs() as f32;
        let height = (self.top_left.y - self.bottom_right.y).abs() as f32;

        Area {
            area: width * height
        }
    }
}


impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Area is {} cm²", self.area)
    }
}

impl Add for Area {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            area: self.area + rhs.area
        }
    }
}

pub fn sum_areas(items: &[&dyn GetArea]) -> f32{
    let mut sum = Area::default();
    items.iter().for_each(|item| sum = sum + item.get_area());
    sum.area
}


//EX8


pub struct Chair<'a> {
    pub(crate) color: &'a str,
    pub(crate) quantity: &'a usize
}

pub struct Wardrobe<'b>{
    pub(crate) color: &'b str,
    pub(crate) quantity: &'b usize
}

pub trait Object {
    fn build(&self) -> &str;
    fn get_quantity(&self) -> String;
}

impl<'a> Object for Chair<'a> {
    fn build(&self) -> &str {
        "A Chair has been built"
    }

    fn get_quantity(&self) -> String {
        format!("There are {} Chairs", self.quantity)
    }
}

impl<'b> Object for Wardrobe<'b> {
    fn build(&self) -> &str {
        " A Wardrobe has been built"
    }

    fn get_quantity(&self) -> String {
        format!("There are {} Wardrobes", self.quantity)
    }
}


impl Display for Chair<'_> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match *self.quantity {
           0 => write!(f, "There are no Chairs"),
           1 => write!(f, "There is 1 {} chair", self.color),
           _ => write!(f, "There are {} {} chairs", self.quantity, self.color),
       }
    }
}


impl Display for Wardrobe<'_> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self.quantity {
            0 => write!(f, "There are no Wardrobes"),
            1 => write!(f, "There is 1 {} Wardrobe", self.color),
            _ => write!(f, "There are {} {} Wardrobes", self.quantity, self.color),
        }
    }
}

//EX 9
#[derive(PartialEq, Eq, Debug)]
pub enum Role{
    GUEST,
    USER,
    ADMIN
}
#[derive(PartialEq, Eq, Hash)]
pub enum Permission {
    READ,
    WRITE,
    EXECUTE
}

pub struct User{
    pub(crate) name: String,
    pub(crate) role: Role,
    pub(crate) actions: Vec<Actions>
}

pub struct Actions{
    pub action: String,
    pub permission: HashMap<Permission, bool>
}


pub trait Auth {
    fn check_permission(&self, permission: &Permission, action: &str) -> bool;
    fn can_write(&self, action: &str) -> bool;
    fn can_read(&self, action: &str) -> bool;
    fn can_execute(&self, action: &str) -> bool;
}

impl Auth for User {
    fn check_permission(&self, permission_type: &Permission, action: &str) -> bool {
        self.actions.iter().any(
            |a|
                a.action == action && *a.permission.get(permission_type).unwrap_or(&false)
        )
    }

    fn can_write(&self, action: &str) -> bool {
        self.check_permission(&Permission::WRITE, action)
    }

    fn can_read(&self, action: &str) -> bool {
        self.check_permission(&Permission::READ, action)
    }
    fn can_execute(&self, action: &str) -> bool {
        self.check_permission(&Permission::EXECUTE, action)
    }
}

impl Default for Actions {
    fn default() -> Self {
        Self {
            action: "".to_string(),
            permission: HashMap::from([(Permission::READ, false),(Permission::WRITE, false),(Permission::EXECUTE, false)],),
        }
    }
}

impl Actions {
    pub fn new(action: String, read: bool, write: bool, execute: bool) -> Self {
        Self{
            action,
            permission: HashMap::from([(Permission::READ,read), (Permission::WRITE, write), (Permission::EXECUTE,execute)]),
        }
    }
}

impl Default for User{
    fn default() -> Self {
        Self{
            name: "Guest".to_string(),
            role: Role::GUEST,
            actions: Vec::new()
        }
    }
}

impl User {
    pub fn change_role(&mut self, new_role: Role) -> Result<(),String> {
        match self.role {
            Role::GUEST => {
                if new_role == Role::GUEST {
                    Ok(())
                }
                else { Err(format!("Cannot change role from {:?} to {:?}",self.role, new_role)) }
            }

            Role::USER => {
                if new_role == Role::USER || new_role == Role::GUEST {
                    self.role = new_role;
                    Ok(())
                }
                else {
                    Err(format!("Cannot change role from {:?} to {:?}",self.role, new_role))
                }
            }

            Role::ADMIN => {
                self.role = new_role;
                Ok(())
            }
        }

    }
}

pub fn sudo_change_permission(user: &mut User, permission: Permission, string: &String, value: bool ) {
    if let Some(pos) = user.actions.iter().position(|a| a.action == *string){
        let action_to_change: &mut Actions = user. actions.get_mut(pos).unwrap();
        action_to_change.permission.insert(permission, value);
    }
}