
/**************
*
* EXERCISE 1
*
***************/
pub trait Printable{
    fn print(&self);
}

impl Printable for i32{
    fn print(&self){
        println!("i32 number: {}", self);
    }
}

impl Printable for String{
    fn print(&self){
        println!("String: {}", self);
    }
}

impl <T> Printable for Vec<T>
where T: Printable
{
    fn print(&self){
        println!("Vector:");
        for element in self.iter(){
            element.print();
        }
    }
}

pub fn print<T: Printable>(item: &T){
    item.print();
}

/*
//dynamic dispatch version
 pub fn print(item: &dyn Printable){
    item.print();
 }
 */


/**************
*
* EXERCISE 2
*
***************/
#[derive(Debug)]
pub struct Book<'a>{
    pub title: &'a str,
    pub cat: Category
}
#[derive(Debug,Default)]
pub enum Category{
    Thriller,
    Love,
    #[default]
    Science,
}

#[derive(Debug, Default)]
pub struct Library<'a>{
    bookcases: [Vec<Book<'a>>; 10]
}

impl Default for Book<'_>{
    fn default()->Self{
        Book{
            title: "default title",
            cat: Category::default(),
        }
    }
}

impl Book<'_>{
    pub fn default_with_cat(cat: Category)->Self{
        Book{
            cat,
            ..Self::default()
        }
    }
}

pub trait Populatable{
    fn populate(&mut self);
}

impl Populatable for Library<'_>{
    fn populate(&mut self){
        for floor in self.bookcases.iter_mut(){
            for _ in 1..=3 {
                floor.push(Book::default());
            }
        }
    }
}

/**************
*
* EXERCISE 3
*
***************/
pub fn restricted<T,U>(t1:T, t2:T, u: U) -> T
where
    T: Debug + PartialOrd + Ord,
    U: Display,
{
    let minor = t1.min(t2);

    println!("minor: <{:?}>", minor);
    println!("u: <{u}>");
    minor
}
/**************
*
* EXERCISE 4
*
***************/

pub struct Task{
    name:String,
    priority:i32,
    done:bool,
}

pub struct Tasks{
    tasks:Vec<Task>,
}

impl Task{
    pub fn new(name:String,priority:i32,done:bool)->Self{
        Task{
            name,
            priority,
            done
        }
    }

}

impl Tasks{
    pub fn new(tasks:Vec<Task>)->Self{
        Tasks{
            tasks
        }
    }

    pub fn add(&mut self, task:Task){
        self.tasks.push(task);
    }
}


impl Iterator for Tasks{
    type Item = Task;
    fn next(&mut self) -> Option<Self::Item>{
        self.tasks
            .iter()
            .position(|t| !t.done)
            .map(|i| self.tasks.remove(i))
    }
}

/**************
*
* EXERCISE 5
*
***************/

pub struct Pair(i32,String);

impl Add<i32> for Pair{
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Pair(self.0 + rhs, self.1)
    }
}

impl Add<&str> for Pair{
    type Output = Self;

    fn add(self, rhs: &str) -> Self::Output {
        Pair(self.0, self.1 + rhs)
    }
}

impl Add for Pair{
    type Output = Self;

    fn add(self, rhs: Pair) -> Self::Output {
        self + rhs.0 + rhs.1.as_str()
    }
}

impl Sub<i32> for Pair{
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Pair(self.0 + rhs, self.1)
    }
}

impl Sub<Pair> for Pair{
    type Output = Self;

    fn sub(self, rhs: Pair) -> Self::Output {
        self - rhs.0 - rhs.1.as_str()
    }
}

impl Sub<&str> for Pair{
    type Output = Self;

    fn sub(self, rhs: &str) -> Self::Output {
        Pair(self.0, self.1.replace(rhs, ""))
    }
}

impl Mul<i32> for Pair{
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        /*let mut s = String::new();

        for _ in 0..rhs {
            s.push_str(self.1.as_str());
        }*/
        Pair(self.0.pow(rhs as u32), self.1.repeat(rhs as usize))

    }
}

/**************
*
* EXERCISE 6
*
***************/
use std::fmt::{Debug, Display};
use std::ops::{Add, Mul, Sub};
use rand::{self, Rng};

pub struct Gate<S>{
    _state: S
}

pub struct Open;
pub struct Closed;
pub struct Stopped{
    _reason: String
}

impl Gate<Open>{
    pub fn close(self) -> Result<Gate<Closed>,Gate<Stopped>>{
        let n = rand::rng().random_range(0 ..20);
        match n {
            0..=10 => Ok(Gate{_state: Closed}),
            11..=16 =>Err(
                Gate{
                    _state: Stopped{
                        _reason: "motor malfunctioning".to_string(),
                    },
                }),
            _ => Err(
                Gate{
                    _state: Stopped{
                        _reason: "detected person".to_string(),
                    },
                })
        }
    }
}

impl Gate<Closed>{
    pub fn open(self) -> Result<Gate<Open>,Gate<Stopped>>{
        let n = rand::rng().random_range(0 ..20);
        match n {
            0..=10 => Ok(Gate{_state: Open}),
            11..=16 =>Err(
                Gate{
                    _state: Stopped{
                        _reason: "motor malfunctioning".to_string(),
                    },
                }),
            _ => Err(
                Gate{
                    _state: Stopped{
                        _reason: "sensors malfunctioning".to_string(),
                    },
                })
        }
    }
}

impl Gate<Stopped>{

    pub fn close(self) -> Gate<Closed>{
        Gate{_state: Closed}
    }

    pub fn open(self) -> Gate<Open>{
        Gate{_state: Open}
    }
}

/**************
*
* EXERCISE 7
*
***************/
pub trait Heatable {
    fn cook(&mut self);
}

pub trait Friable {
    fn cook(&mut self);
}

pub trait Heater {
    fn heat(&self, elem: &mut dyn Heatable);
}

pub trait Frier {
    fn fry(&self, elem: &mut dyn Friable);
}

pub struct Oven;
impl Heater for Oven{
    fn heat(&self, elem: &mut dyn Heatable){
        elem.cook();
    }
}

pub struct Pan;
impl Heater for Pan{
    fn heat(&self, elem: &mut dyn Heatable){
        elem.cook();
    }
}
impl Frier for Pan{
    fn fry(&self, elem: &mut dyn Friable){
        elem.cook();
    }
}


pub struct Pie{
    pub ready : bool,
}

pub struct Carrot{
    pub state: CarrotState,
}

pub enum CarrotState{
    Raw,
    Cooked,
    Fried,
    Burnt
}

pub trait Edible{
    fn eat(&self);
}

impl Heatable for Pie{
    fn cook(&mut self){
        if self.ready{
            println!("you burned the pie");
        }
        else {
            self.ready = true;
        }
    }
}

impl Heatable for Carrot{
    fn cook(&mut self){
        match self.state {

            CarrotState::Raw => {
                self.state = CarrotState::Cooked
            },

            _ => self.state = CarrotState::Burnt
        }
    }
}

impl Friable for Carrot{
    fn cook(&mut self){
        match self.state {

            CarrotState::Fried => {
                self.state = CarrotState::Burnt
            },

            _ => self.state = CarrotState::Fried
        }
    }
}

impl Edible for Carrot{
    fn eat(&self){
        match self.state {
            CarrotState::Fried => {println!("mmh, crispy" )},
            CarrotState::Raw => {println!("mmh, crunchy" )},
            CarrotState::Burnt => {println!("mmh, burnt" )},
            CarrotState::Cooked => {println!("mmh, yummy" )},

        }
    }
}

impl Edible for Pie{
    fn eat(&self){
        if self.ready{
            println!("yummy")
        }else {
            println!("your stomach aches")
        }
    }
}