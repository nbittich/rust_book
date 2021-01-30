fn tuples() -> (i32,bool,String) {
    (1,false, "Yayy".to_string())
}

pub fn print_tuples(){
    let (x,y,z) = tuples();
    println!("x: {}, y: {}, z: {}",x,y,z);
}

pub fn test_array_mut(){
    let arr1: [i32;3] = [1,2,3];
    println!("{},{},{}",arr1[0],arr1[1],arr1[2]);
    let arr1 = [2;3];
    println!("{},{},{}",arr1[0],arr1[1],arr1[2]);
}

pub fn test_if_condition(x:i32) {
    let result = if x <10 {"X is lower than 10"} else if x < 20 {"X is lower than 20"} else {"I don't know man"};
    println!("{}", result);
}

pub fn test_countdown(){
    let mut counter = 0;
    let final_countdown = loop {
        counter += 1;
        if counter == 10 {
            break counter
        }
    };
    println!("Done! {}",final_countdown);
    for element in 1..=10 {
        println!("{}...", element);
    }

    for element in ["alala","ololol", "ululu"].iter() {
        println!("{}...", element);
    }
    println!("LiftOff!");
}