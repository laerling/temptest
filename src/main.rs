//use std::thread::{JoinHandle, sleep, spawn};
//use std::time::Duration;

use scrypt::{scrypt, Params};

const KEY_LEN: usize = 32;

fn main() {
    /*
    let x: JoinHandle<u64> = spawn(|| {
        println!("Thread sleeping...");
        sleep(Duration::from_secs(1));
        println!("Thread returning");
        return 3;
    });
    println!("Outside of thread, waiting...");
    let y: u64 = x.join().unwrap();
    println!("Thread joined");
    println!("Result: {}", y);
    */

    let mut password = [0; KEY_LEN];
    let (log_n, r, p) = (16, 8, 16);
    let scrypt_params = Params::new(log_n, r, p, KEY_LEN).expect("Cannot create scrypt parameters");
    let salt = "salt";
    let master_password = "your mom";
    let _ = scrypt(
        master_password.as_bytes(),
        salt.as_bytes(),
        &scrypt_params,
        &mut password,
    );
}
