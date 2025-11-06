use std::collections::HashMap;
use std::slice::Iter;


enum DoubleType {
    Int(i32),
    Str(String)
}

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}


fn main() {

    /* modify_odd and create_vec*/
    //let mut v = create_vec();
    //modify_odd(&mut v);
    //println!("modified vector: {:?}", v);

    //count_chars
    //let chars_in_str = count_chars(&String::from("hello World!"));
    //println!("chars in str: {:?}", chars_in_str);

    //split at value
    //let vec = vec![1,3,5,7,9,11];
    //let value = 2;
    /*match split_at_value(&vec,value) {
        Some((v1,v2)) => println!("v1: {:?}\nv2: {:?}", v1,v2),
        None => println!("Value {} not found",value ),
    }*/

    //max_recursive
    //let vec = Vec::new();
    //println!("{:?}", max_recursive(&vec));

    //swap
    //let mut v = vec![1,2,3,4,5,6,7,8,9,10];
    //let mut v = Vec::new();
    //swap_first_last(&mut v);
    //println!("{:?}", v);

    //IS SORTED
    //let vector = vec![1,2,3,4,4,5,6,7,8,9,10];
    //let vector = vec![10,10,9,8,7,6,5,4,3,2,1];
    //let vector = vec![3,5,4,6,7,3,4,5];
    //println!("{:?}",is_sorted_rec(&vector));

    //INSERT
    //let mut strV = vec![String::from("hello"),String::from("world")];
    //let s = String::from("Ciao");
    //let s1 = String::from("Hello Everyone!");
    //insert_if_longer(&mut strV,s1 );
    //println!("{:?}",strV);

    //merge
    //let mut vector = vec![1,2,3,4,4,5];
    //let mut vector1 = vec![1,1,2,3,4,4,5,6,7,8,9,10];
    //let merged = merge(&mut vector, &mut vector1);
    //println!("{:?}", merged);


    let p = Person {
        name: String::from("Jacopo"),
        age: 22
    };

}


pub fn modify_odd(slice: &mut Vec<i32>) {
    for i in 0..slice.len() {
        if slice[i] %2 == 1 {
            slice[i] = 0;
        }
    }
}

pub fn create_vec() -> Vec<i32> {

    let mut vec: Vec<i32> = Vec::new() ;
    for i in 0..100  {
        vec.push(i);

    }
    vec
}


pub fn count_chars(str: &String) -> HashMap<char, u32>  {
    let mut count: HashMap<char, u32> = HashMap::new();
    for c in str.chars() {
        if count.contains_key(&c) {
            *count.get_mut(&c).unwrap() += 1;
        }
        else { count.insert(c, 1); }
    }
    count
}

pub fn split_at_value(vec: &Vec<i32>, value: i32) -> Option<(&[i32], &[i32])> {

    match get_index_of(vec,value) {
        Some(i) => {
            Some(vec.split_at(i))
        }
        None => { None }
    }
}


pub fn get_index_of(vec: &Vec<i32>, val: i32) -> Option<usize> {
    for i in 0..vec.len() {
        if vec[i] == val {
            return Some(i)
        }
    }
    None
}

pub fn max_recursive (vec: &[i32]) -> Option<i32>{

    if vec.len() == 0 {return None}
    if vec.len() == 1 {return Some(vec[0])}

    let max_but_first = max_recursive(&vec[1..]);
    match max_but_first {
        Some(x) => {Some(i32::max(x,vec[0]))},
        None => {None}
    }

}


pub fn swap_first_last(vec: &mut [i32]){

    let len = vec.len();

    if len == 0 || len == 1  {return}

    vec.swap(0,len - 1);

}

pub fn is_sorted_rec(v : &[i32]) -> bool
{

    let len = v.len();
    if len == 0 || len == 1 { return true }

   v[0] <= v[1] && is_sorted_rec(&v[1..])
}



pub fn insert_if_longer(v: &mut Vec<String>,str: String)  {
    if str.len() >= 10 { v.insert(v.len(), str); }
}

pub fn build_vector(elem: Iter<i32>) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    for e in elem{
        v.push(*e);
    }
    v
}


pub fn merge(v1: &mut Vec<i32>, v2: &mut Vec<i32>) -> Vec<i32> {

    let mut merged: Vec<i32> = Vec::new();
    let mut left_index : usize = 0;
    let mut right_index : usize = 0;
    let v1len = v1.len();
    let v2len = v2.len();

    if v1len == 0 && v2len == 0 { merged }
    else if v1len == 0 {
        merged.append(v2);
        merged
    } else if v2len == 0 {
        merged.append(v1);
        merged
    }
    else {

        for _ in 0..v1.len() + v2.len(){

            if left_index < v1.len() && right_index < v2.len() && v1[left_index] <= v2[right_index] || right_index >= v2len  {
                merged.push(v1[left_index]);
                left_index += 1;
            }
            else if left_index < v1.len() && right_index < v2.len() && v1[left_index] > v2[right_index] || left_index >= v1len  {
                merged.push(v2[right_index]);
                right_index += 1;
            }

        }
        merged

    }
}


