use std::collections::{HashMap};

fn main() {

    let txt = "ciao Mamma";
    //println!("String was:{}, now it is {}", txt,string_reverse(txt));

    let a = 10;
    let b = 20;
    //println!("Biggest Number between {} and {} is {}",a,b,bigger(a,b));

    println!("Product of numbers: {}",multiply(5,7,7.89));

    let v = vec![-1,1,2,3,7,3,4,8];
    let (min,max) = max_min(v);
    println!("max : {}, min: {}",max,min);


    let mut furniture_set:HashMap<String,f32> = HashMap::new();
    furniture_set.insert("Kitchen".to_string(),145.9);
    furniture_set.insert("Chair".to_string(),167.9);
    println!("{:?}", furniture_set);
    println!("{}", find_furniture(&furniture_set,"Chair".to_string()));
    println!("{}", find_furniture(&furniture_set,"Kitchen".to_string()));
    println!("{}", find_furniture(&furniture_set,"Hi".to_string()));

    println!("{}",is_armstrong(157));
}


///Write a function string_reverse that takes a &str as input and returns it, reversed as a String ;
pub fn string_reverse(text: &str) -> String {

    let mut reversed = String::new(); //has to be mutable to be modified
    for c in text.chars().rev() {
        reversed.push_str(&c.to_string());
    }

    return reversed
}


///Write a function bigger that takes two i32 and returns the bigger number ( i32 ) without using
///another function call and additional variables;
pub fn bigger(a: i32, b: i32) -> i32 {
    return if a >= b {
        a
    }
    else { b }
}

///Write a function multiply that takes an i32 , a f32 and a f64 and returns the multiplication of the
/// three of them as a f64 value;
pub fn multiply(a: i32, b: i32, c:f64) -> f64 {
    return a as f64 * b as f64 * c
}

///Given a vector of i32 , create a function max_min that returns the maximum and the minimum value
/// inside that vector;
pub fn max_min(v: Vec<i32>) -> (i32, i32) {

    let mut max = v[0];
    let mut min = v[0];
    for i in &v {
        if *i < min { min = *i; }
        else if *i > max { max = *i; }
    }
    return (min, max)
}


pub fn lord_farquaad(s: String) -> String {
    return  s.replace("e"," 💥")
}



pub fn find_furniture (f: &HashMap<String,f32>, furniture: String) -> f32 {

    if f.contains_key(&furniture) {
        return f[&furniture];
    }

    return -1.0
}


pub fn is_armstrong(mut n: i32) -> bool {

    let original_n = n.clone();
    let len = n.to_string().len() as u32;

    let mut sum = 0;
    while n != 0 {
        sum += (n % 10).pow(len);
        n /= 10;
    }

    original_n == sum

}