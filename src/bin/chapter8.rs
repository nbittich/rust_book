use std::collections::HashMap;
use std::ops::{Div};

fn main() {

    let  mut vector = vec![1,22,33,44];

    vector.push(43);

    for  i in &mut vector {
        *i+=1
    }

    vector.iter().for_each(|i| {
        println!("{}", i);
    });

    let smileys = String::from("😀😆😅🤣🤩🤪🤑🤭🤐😪😷😵‍💫");
    println!("{}", smileys.len());
    println!("{}", &smileys[0..4]);
    for  c in smileys.chars() {
        println!("{}", c);

    }
    let (median,avg,map) = exercise1(&mut vec![4,3,2,1,4,4,7,2,3,9,1,7,8]);
    println!("median: {}, avg: {}", median, avg);
    map.iter().for_each(|e|{
        println!("occurrence for {}: {}",e.0, e.1);
    })

}

fn exercise1(list_of_integers:&mut Vec<u32>) -> (u32, u32, HashMap<u32, u32>) {
    let len:usize = list_of_integers.len() - 1;
    let mut avg = 0;
    let mut occurrences: HashMap<u32, u32> = HashMap::new();

    list_of_integers.sort();
    let median =  list_of_integers[len / 2];


    list_of_integers.iter().for_each(|i| {
        let x = *i;
        let count = occurrences.entry(x).or_insert(0);
        *count+=1;
        avg += x;
    });

    let avg:u32 = avg.div(len as u32);


   (median, avg, occurrences)
}
