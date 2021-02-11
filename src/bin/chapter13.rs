fn main() {
    let parenthesis_str = "( a(b) (c) (d(e(f)g)h) I (j(k)l)m)";

    let max_depht = parenthesis_str.chars()
        .filter(|c| c == &'(' || c == &')')
        .map(|c| if c == '(' {1} else {-1})
        .scan(0, |state,x| {*state+=x; Some(*state)})
        .max().unwrap();

    println!("max depht is {}", max_depht);

}
