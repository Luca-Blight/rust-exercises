use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
  let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi thread 1"),
            String::from("from thread 1"),
            String::from("the thread 1"),
            String::from("thread thread 1"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more thread 2"),
            String::from("messages thread 2"),
            String::from("for thread 2"),
            String::from("you thread 2"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}