use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::os::macos::raw::stat;
use std::ops::Add;

fn main() {
    let tomato = Tomato {
        color: Color::RED,
        expiry_date: "12/02/2021".to_string(),
        price_per_kilogram: 1,
    };
    let superman = Ticket {
        movie_name: "Superman".to_string(),
        price_per_entry: 12,
    };

    println!("Price for {} tomato: {}", tomato.get_color(), tomato.get_price());
    println!(
        "Price for {}: {}",
        superman.movie_name,
        superman.get_price()
    );

    let mut map= HashMap::new();
    let mut stock = MyStock {stock: map};
    println!("stock for tomato now: {}",stock.add_to_stock(&tomato));
    println!("stock for tomato now: {}",stock.add_to_stock(&tomato));
    println!("stock for tomato now: {}",stock.add_to_stock(&tomato));
    println!("stock for tomato now: {}",stock.add_to_stock(&tomato));
    println!("stock for tomato now: {}",stock.add_to_stock(&tomato));
    println!("stock for tomato now: {}",stock.add_to_stock(&tomato));
    println!("stock for tomato now: {}",stock.add_to_stock(&tomato));

    let mut map= HashMap::new();
    let mut stock = MyStock {stock: map};
    println!("stock for superman now: {}",stock.add_to_stock(&superman));
    println!("stock for superman now: {}",stock.add_to_stock(&superman));
    println!("stock for superman now: {}",stock.add_to_stock(&superman));
    println!("stock for superman now: {}",stock.add_to_stock(&superman));
    println!("stock for superman now: {}",stock.add_to_stock(&superman));
    println!("stock for superman now: {}",stock.add_to_stock(&superman));

}
#[derive(Eq, PartialEq,Clone)]
pub enum Color {
    RED,
    GREEN,
}


pub struct MyStock<'a, T> where T: Article {
    stock: HashMap<&'a T, u32>
}

#[derive(Eq, PartialEq, Clone)]
pub struct Tomato {
    color: Color,
    expiry_date: String,
    price_per_kilogram: u32,
}

#[derive(Eq, PartialEq, Clone)]
pub struct Ticket {
    movie_name: String,
    price_per_entry: u32,
}

pub trait WithColor {
    fn get_color(&self) -> &str;
}

pub trait Article {
    fn get_price(&self) -> &u32;
}

pub trait Stock<'a,T> {
    fn add_to_stock(&mut self, article: &'a T) -> &u32;
    fn get_stock(&self) -> &HashMap<&'a T, u32>;
}

impl Article for Tomato {
    fn get_price(&self) -> &u32 {
        &self.price_per_kilogram
    }
}

impl Article for Ticket {
    fn get_price(&self) -> &u32 {
        &self.price_per_entry
    }
}

impl WithColor for Tomato {
    fn get_color(&self) -> &str {
        match self.color {
            Color::RED => "red",
            Color::GREEN => "green"
        }
    }
}

impl <'a, T:Article + Eq + Hash> Stock<'a, T> for MyStock<'a, T> {
    fn add_to_stock(&mut self,article: &'a T) -> &u32 {
        let st = self.stock.entry(article).or_insert(0);
        *st+=1;
       st
    }

    fn get_stock(&self) -> &HashMap<&'a T, u32> {
        &self.stock
    }
}

impl Hash for Tomato {
    fn hash<H: Hasher>(&self, state: &mut H) {

    }
}
impl Hash for Ticket {
    fn hash<H: Hasher>(&self, state: &mut H) {

    }
}
