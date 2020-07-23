use std::time::SystemTime;

fn main() {
    let unix_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    println!("{}", unix_time.as_secs());
}
