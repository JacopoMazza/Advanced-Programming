use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq,Debug)]
pub enum Item {
    Coke,
    Coffee,
    Snack,
    Latte,
    Water
}

pub enum Coin {
    Euro1,
    Euro2,
    Cents50,
    Cents20,
    Cents10
}

impl Coin {
    pub fn to_cents(&self) -> u32 {
        match self {
            Coin::Euro1 => 100,
            Coin::Euro2 => 200,
            Coin::Cents50 => 50,
            Coin::Cents20 => 20,
            Coin::Cents10 => 10,
        }
    }
}
#[derive(Debug)]
pub struct VendingMachine{
    coins: u32,
    items:HashMap<Item,usize>
}


impl VendingMachine {
    pub fn new( items:HashMap<Item,usize>) -> VendingMachine {
        VendingMachine{
            items,
            coins: 0,
        }
    }

    pub fn add_item(&mut self, item: Item, item_num: usize) -> Result<(), &str> {
        match self.items.insert(item, item_num){
            None => {Err("item not found")}
            _ => {Ok(())}
        }
    }

    pub fn add_coins(&mut self, coins:Coin) -> Result<&str, &str> {
       self.coins += coins.to_cents();
       return Ok("coins inserted");
    }

    pub fn get_item_price(item: &Item) -> u32
    {
        match item {
            Item::Coke => 250,
            Item::Coffee => 150,
            Item::Snack => 100,
            Item::Latte => 200,
            Item::Water => 50,
        }

    }

    pub fn buy_item(&mut self, item: &Item) -> Result<u32, &str> {

        if self.coins >= Self::get_item_price(item){

           let change = self.coins - Self::get_item_price(item);
           match self.items.get_mut(item) {
               None => {
                   Err("item not found")
               }
               
               Some(0) => {
                   Err("item is not available")
               }
               
               Some(quantity) => {
                   *quantity -= 1;
                   self.coins = 0;
                  Ok(change)
               }
           }
        }
        else {
            Err("insufficient coins")
        }

    }
}

