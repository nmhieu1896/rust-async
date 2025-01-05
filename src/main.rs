use std::thread::{self, sleep};

fn main() {
    println!("So, we start the program here!");
    let t1 = thread::spawn(|| {
        sleep(std::time::Duration::from_millis(200));
        println!("The long running tasks finish last!");
        return 1;
    });

    let t2 = thread::spawn(|| {
        sleep(std::time::Duration::from_millis(100));
        println!("We can chain callbacks...");
        let t3 = thread::spawn(move || {
            sleep(std::time::Duration::from_millis(50));
            println!("...like this!");
            return 2;
        });
        return t3.join().unwrap();
    });
    println!("The tasks run concurrently!");

    let res1 = t1.join().unwrap();
    let res2 = t2.join().unwrap();
    println!("{} - {}", res1, res2)
}
