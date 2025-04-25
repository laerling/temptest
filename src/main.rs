#![allow(unused_imports)]
#![allow(dead_code)]

use std::thread::{JoinHandle, sleep, spawn};
use std::time::Duration;

use scrypt::{scrypt, Params};

const KEY_LEN: usize = 10; // minimum is 10 (80 bit), maximum is 64 (256 bit)

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

    // test scrypt performance
    let mut key = Vec::new();
    key.resize(KEY_LEN, 0);
    let (log_n, r, p) = (4, 8, 16);
    let scrypt_params = Params::new(log_n, r, p, KEY_LEN)
        .expect("Cannot create scrypt parameters");
    let salt = "salt";
    let master_password = "your mom";
    scrypt(
        master_password.as_bytes(),
        salt.as_bytes(),
        &scrypt_params,
        &mut key,
    ).expect("scrypt failed");
    println!("{:?}", key);
}
