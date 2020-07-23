use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn _main() {
    loop {
        // Print current time
        let unix_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        println!("{}", unix_time.as_secs());

        // Sleep one second
        sleep(Duration::from_secs(1));
    }
}

fn main() {
    // Print current time
    let unix_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    print!("{}", unix_time.as_secs());
}
