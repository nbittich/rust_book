use std::{thread};
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    let val = vec!["Hello","from","other","thread","!"];
    thread::spawn(move || {
        let message = val.join(" ");
        println!("{}",message);
    });
    println!("hello from thread {}","main");
    thread::sleep(Duration::from_secs(2));

    multiple_producer_single_consumer();
    arc_mutex();
}



fn multiple_producer_single_consumer () {
    let (tx,rx) = mpsc::channel();
    let second_sender = tx.clone();

    thread::spawn(move || {
        let vals = vec!["A","E","I","O","U", "Y"];
        for a in vals {
            second_sender.send(a).unwrap();
            thread::sleep(Duration::from_nanos(2))
        }
    });
    thread::spawn(move || {
        let vals = vec!["B","C","D","F", "G","H","J","K","L","M","N","P","Q","R","S","T","V","W","X","Z"];
        for a in vals {
            tx.send(a).unwrap();
            thread::sleep(Duration::from_nanos(10))
        }
    });

    for msg in rx {
        println!("{}", msg);
    }


}

fn arc_mutex () {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        let cloned_arc = Arc::clone(&counter);
        handlers.push(thread::spawn(move ||{
            let mut count =  cloned_arc.lock().unwrap();
            println!("count: {}", count);
            *count+=1;
        }));
    }
    for h in handlers {
        h.join().unwrap()
    }
    println!("Final count: {}", counter.lock().unwrap());
}
