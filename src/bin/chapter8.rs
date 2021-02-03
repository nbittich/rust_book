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

}
