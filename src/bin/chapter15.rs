use crate::List::{Cons, Nil};

fn main() {
    let recurs = list(Cons(5, Box::new(Nil)));
    println!("{:?}", &recurs);
    println!("{:?}", fact(&recurs))
}

#[derive(Debug)]
enum List {
    Cons(i128, Box<List>),
    Nil,
}

fn list(f: List) -> List {
    match f
    {
        Cons(x, acc) if x > 1 => list(Cons(x - 1, Box::new(Cons(x, acc)))),
        _ => f
    }
}

fn fact(f: &List) -> i128 {
        match f
        {
            Cons(x, acc) if x >= &1 => x * fact(acc),
            _ => 1
        }
}
