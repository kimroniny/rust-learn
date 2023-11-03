extern crate pipeliner;
use pipeliner::Pipeline;
use std::{
    cell::{Cell, RefCell},
    fmt::format,
    net::TcpStream,
    process::Command,
    rc::{self, Rc},
    result,
    sync::{mpsc, Arc, Mutex},
    thread, time, vec,
};

fn main() {
    // ----------Change Unchangeable----------
    let answer = Cell::new(34); // 只能用于基本类型, 或者说只能用于实现了 copy 功能的类型
    answer.set(10);
    println!("answer after set: {}", answer.get());
    println!("Hello, world!");

    let greet = RefCell::new("hello-yiiguo".to_string());
    assert_eq!(*greet.borrow(), "hello-yiiguo");
    println!("greet init: {}", greet.borrow());
    *(greet.borrow_mut()) = "hello-future".to_string();
    println!("greet after mut: {}", greet.borrow());

    // ----------Shared Reference----------
    let s = "shared hello".to_string();
    let rc1 = Rc::new(s);
    let rc2 = rc1.clone(); // 这里要 clone, 因为 s 已经被 moved 到 rc1 了
    println!("rc1: {}", rc1);
    println!("rc2: {}", rc2);

    // ----------MultiThreading----------
    let mut threads = Vec::new();
    for i in 0..5 {
        let thread = thread::spawn(move || {
            let thread_name = format!("thread-{}", i).to_string();
            for j in 0..5 {
                println!("{thread_name}: {j}");
                thread::sleep(time::Duration::from_millis(100));
            }
        });
        threads.push(thread);
    }
    for th in threads {
        let _ = th.join();
    }
    println!("-----------arc-----------");
    let name = MyName::new("yiiguo");
    let arcname = Arc::new(name);
    let mut threads = Vec::new();
    for _ in 0..5 {
        let nm = arcname.clone();
        let thread = thread::spawn(move || {
            for j in 0..3 {
                println!("{}: {j}", nm.name);
                thread::sleep(time::Duration::from_millis(100));
            }
        });
        threads.push(thread);
    }
    for th in threads {
        let _ = th.join();
    }

    // ----------Channel----------
    println!("----------Channel----------");
    let (sx, rx) = mpsc::channel();
    let mut threads = Vec::new();
    for i in 0..5 {
        let sender = sx.clone();
        let thread = thread::spawn(move || {
            let thread_name = format!("thread-{}", i).to_string();
            for j in 0..5 {
                sender
                    .send(format!("{thread_name}: {j}").to_string())
                    .unwrap();
                thread::sleep(time::Duration::from_millis(100));
            }
            drop(sender);
        });
        threads.push(thread);
    }
    for th in threads {
        let _ = th.join();
    }
    drop(sx); // 把 sx 删除掉(即 disconnect), recv 才会停止

    loop {
        if let Ok(x) = rx.recv() {
            println!("{x}");
        } else {
            break;
        }
    }

    // ----------Shared State----------
    println!("----------Shared State----------");
    let count = Arc::new(Mutex::new(0));
    let mut threads = Vec::new();
    for i in 0..5 {
        let count = count.clone();
        let thread = thread::spawn(move || {
            let thread_name = format!("thread-{}", i).to_string();
            for j in 0..2 {
                {
                    // 最好是放在一个短的 scope 下面, 这样能够保证 lock 的时间尽可能短, 因为只有当 data 被释放的时候才会 unlock
                    let mut data = count.lock().unwrap();
                    *data += 1;
                    println!("{thread_name} change data: {}", *data);
                }
                thread::sleep(time::Duration::from_millis(100));
            }
        });
        threads.push(thread);
    }
    for th in threads {
        let _ = th.join();
    }
    println!("final data: {:?}", count.lock().unwrap());

    // ----------Higher-Level Operations----------
    println!("----------Higher-Level Operations----------");
    for result in (0..10).with_threads(5).map(|x| x + 1) {
        println!("result: {}", result);
    }

    let addresses: Vec<String> = (0..10).map(|_x| format!("ping -c1 www.baidu.com")).collect();
    for result in addresses.with_threads(10).map(|cmd| shell(&cmd)) {
        println!("result: {:#?}", result);
    }
}

fn shell(cmd: &str) -> (String, bool) {
    let cmd = format!("{} 2>&1", cmd);
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("no shell?");
    (
        String::from_utf8_lossy(&output.stdout).trim().to_string(),
        output.status.success(),
    )
}

struct MyName {
    name: String,
}

impl MyName {
    fn new(s: &str) -> MyName {
        MyName {
            name: s.to_string(),
        }
    }
}
