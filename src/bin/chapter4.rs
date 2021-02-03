pub fn run() {
    str_type();
    out_of_scope_move();
    deep_copy();
    owner_ship();
    borrowing();
    slices();
}


fn str_type() {
    let mut message = String::from("Ok many");
    let message_literal = "I can't";
    message.push_str(", Rebenga");
    println!("{}. I can mutate, I'm on the heap", message);
    println!("{}, mutate. I'm on the stack", message_literal);
}

fn out_of_scope_move() {
    let s1 = String::from("oyee sapapaya");
    let _s2 = s1; // not a shallow copy but a move. s1 is invalidated
    //println!("{}", s1); it won't work
}

fn deep_copy() {
    let s1 = String::from("tu veux jouer à la vache?");
    let s2 = s1.clone();
    println!("{} === {}", s2, s1);
}

/* Ownership */

fn owner_ship() {
    let x: u32 = 3; // in the stack, implements Copy trait, will be copied if it's passed to a function
    make_a_copy_of_x(x);
    println!("but didn't change here as you can see: {}", x);

    let y: String = String::from("hola que tal"); // in the heap, implement Drop  trait, will be moved to the function and cannot be used anymore

    move_y(y);
    // println!("{} is dropped", y) ; // won't compile
}

fn move_y(y: String) {
    println!("y received, will be dropped from the heap once the function call is done : {} ", y);
}

fn make_a_copy_of_x(mut x: u32) {
    println!("old value equals {}", x);
    x = 9;
    println!("x has changed in this scope as you can see: {}... ", x)
}

/* Borrowing */

fn borrowing() {
    let s1 = String::from("ola que passa");
    let length = borrow_me(&s1); // we only pass the reference here, so we can still use s1 later on this block
    println!("{} has a length of {}", s1, length);

    let mut s2 = String::from("amigo"); // we can allow mutation to a reference like so
    change_me(&mut s2);
    println!("now s2 is {}", s2);

    let s3 = String::from("why not?");
    let _x = &s3;
    let _y = &s3;// it's ok as s3 is immutable

    let mut s4 = String::from("just because");
    let _x = &s4;
    let _y = &mut s4;
    //println!("{}, {},", x, y); // it's not ok as s4 has an immutable reference
    println!("{}", _y); // it's  ok as s4's immutable reference is no longer used
}

fn change_me(s2: &mut String) {
    s2.push_str(" I can change!");
}

fn borrow_me(s1: &String) -> usize {
    s1.len()
}

fn slices(){
    let str = String::from("find the first word here");
    let str2 = "and find the first one in that slice";

    let first = find_first_word(&str[..]);
    let other = find_first_word( str2);
    println!("{} is the first word of str", first);
    println!("{} is the first word of str2", other);
}

fn find_first_word(slice: &str) -> &str {
    for(index, &item) in slice.as_bytes().iter().enumerate() {
        if item == b' ' {
             return &slice[..index];
        }
    }
    &slice[..]
}

fn main() {
   run();
}
