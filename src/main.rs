// use std::{thread::sleep, time::Duration};
use tokio::{join, time::{sleep, Duration}};
use once_cell::sync::{Lazy, OnceCell};
use std::sync::Mutex;

// static mut TEST:i32 = 4;
static A: Lazy<Mutex<i32>> = Lazy::new(|| {
    Mutex::new(0)
});

// static A:OnceCell<Mutex<i32>> = OnceCell::new();

// static mut ABC:i32 = 0;

async fn test1() -> &'static str {
    sleep(Duration::from_millis(1000)).await;
    println!("test1");

    let mut bb = A.lock().unwrap();
    println!("{}", bb);
    *bb = 553;

    "test1"
}

async fn test2() -> &'static str {
    sleep(Duration::from_millis(3000)).await;
    println!("test2");
    let mut bb = A.lock().unwrap();
    println!("{}", bb);
    *bb = 400;

    "test2"

}

async fn test3() -> &'static str {
    sleep(Duration::from_millis(2000)).await;
    println!("test3");

    "test3"
}

async fn test4() -> &'static str {
    sleep(Duration::from_millis(5000)).await;
    println!("test4");
    let bb = A.lock().unwrap();
    println!("{}", bb);

    "test4"

}

fn main() {

    {
        let mut bb = A.lock().unwrap();
        *bb = 44;
    }

    let a = async {
        let h1 = tokio::spawn(test1());
        let h2 = tokio::spawn(test2());

        if let(Ok(re1), Ok(re2)) = tokio::join!(h1, h2) {
            let m = format!("{} {}", re1, re2);
            println!("{}", m);
        }
    };

    let b = async {
        let h1 = tokio::spawn(test3());
        let h2 = tokio::spawn(test4());

        if let(Ok(re1), Ok(re2)) = tokio::join!(h1, h2) {
            let m = format!("{} {}", re1, re2);
            println!("{}", m);
        }
    };

    let c = async {
        tokio::join!(a, b);
    };

    tokio::runtime::Runtime::new().unwrap().block_on(c);

    // tokio::runtime::Runtime::new().unwrap().block_on(a);

}