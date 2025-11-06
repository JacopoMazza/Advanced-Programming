extern crate core;

use std::collections::HashMap;
use crate::sentence::sentence::test::magic_sentence;
use crate::sentence::sentence::Sentence as sent;

mod sentence;
mod functions;
mod Ex4;
mod Ex5;
mod Ex6;
mod Point;

fn main() {

 /*   let items = HashMap::from(
       [    (Item::Coffee,5),
                (Item::Coke,7),
                (Item::Latte,2),
           (Item::Water,1)
       ]
    );

    let mut v_machine = VendingMachine::new(items);
    let _ =  v_machine.add_coins(Coin::Euro2);



    match v_machine.buy_item(&Item::Water) {
        Ok(c) => {
            println!("Change: {} cents", c);
            println!("{:?}", v_machine);
        }

        Err(e) => {println!("Error: {}", e)}
    }

    let _ =  v_machine.add_coins(Coin::Euro2);
    match v_machine.buy_item(&Item::Snack) {
        Ok(c) => {
            println!("Change: {} cents", c);
            println!("{:?}", v_machine);
        }

        Err(e) => {println!("Error: {}", e)}
    }
    


    let string = "hello".to_string();
    let s = string.chars().map(|c| if c.is_alphabetic() {
        (c as u8 + 1) as char
    } else {
        c
    }).collect::<String>();

    println!("{}", string);


    let mut hmap: HashMap<char,i32> = HashMap::new();
    for c in string.chars(){
        hmap.entry(c).and_modify(|occ| *occ += 1).or_insert(1);
    }

    hmap.retain(|c,_ | c.eq(&'h'));
    println!("{:?}", hmap);

    let even_squares: i32 =
        (0..=10)
            .map(|n| n * n)
            .filter(|n| n % 2 == 0)
            .sum();

    println!("{}", even_squares);
*/

    let mut map:HashMap<i32, sentence::sentence::Sentence> = HashMap::new();

    let s1 = sentence::sentence::Sentence::new("Hello everyone i'm Jacopo");
    let s2 = sentence::sentence::Sentence::new("Hello guys how are you i'm Jacopo");
    map.insert(1,s1);
    map.insert(2,s2);

    match magic_sentence(&map, 1, 2) {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("{}", e)
    }
}
