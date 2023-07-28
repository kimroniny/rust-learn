use core::slice;
use std::{io::Read, result};

fn add(x: i32, y: i32) -> i32 {
    x + y // 没有分号就是直接返回
}

fn max(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else if x == y {
        10000
    } else {
        y
    }
}

fn change(x: &mut i32) {
    *x = 10;
}

fn main() {
    // ----------basic println----------
    println!("---basic println");
    let name: &str = "basic";
    println!("Hello, {}!", name);

    // ----------loop----------
    println!("---loop");
    for i in 1..10 {
        println!("idx: {}", i);
    }

    println!("---reverse");
    for i in (1..10).rev() {
        println!("idx: {}", i);
    }

    for i in 1..10 {
        let even_odd: &str = if i % 2 == 0 { "even" } else { "odd" };
        println!("even_odd({}) is {}", i, even_odd);
    }

    let mut sum = 0;
    for i in 1..10 {
        sum += i;
    }
    println!("sum is {}", sum);

    // ----------function----------
    let ans = add(1, 2);
    println!("ans is {}", ans);

    let maxx = max(1, 1);
    println!("max(1,1) is {}", maxx);

    // ----------ref----------
    let mut x = 10;
    change(&mut x);
    println!("x after change is: {}", x);

    // ----------array and slices----------
    let arr = [1, 2, 3, 4];
    let res = sum_arr(&arr);
    println!("result of sum array is: {}", res);

    let slice1 = &arr[0..2];
    let slice2 = &arr[1..];
    println!("slice1: {:?}", slice1);
    println!("slice1: {:?}", slice2);

    // ----------optional----------
    let first = slice1.get(0);
    println!("first: {:?}", first);
    println!("is some: {:?}", first.is_some());
    println!("unwrap: {:?}", first.unwrap_or(&0));
    let last = slice1.get(3);
    println!("last: {:?}", last);
    println!("is some: {:?}", last.is_some());
    println!("unwrap: {:?}", last.unwrap_or(&0));

    // ----------vector----------
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    let zero = *(v.get(10).unwrap_or(&0));
    println!("zero is: {}", zero);
    let mut v1 = vec![1, 2, 3];
    v1.extend(1..2);
    println!("v1: {:?}", v1);
    let mut v2 = Vec::new();
    v2.extend(1..4);
    v2.extend(1..2);
    assert_eq!(v2, v1);
    for x in &v2 {
        print!("{x},");
    }
    println!();
    println!("v2: {:?}", v2);
    v2.sort();
    println!("v2 after sort: {:?}", v2);

    // ----------iterator----------
    let mut iter = 0..3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
    for x in arr.iter() {
        // 数组必须要 .iter(), 但是 slice 不需要, 因为内部会自动将 slice 转换为 iter
        print!("{},", x);
    }
    println!();
    for x in &arr {
        print!("{},", x);
    }
    println!();
    let sum: i32 = arr.iter().sum();
    println!("sum iter: {sum}");
    let sum1: i32 = (1..2).sum();
    println!("sum1 iter: {sum1}");

    // ----------string----------
    let hello = "hello world";
    let s = hello.to_string();
    dump(hello);
    dump(&s);

    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() {
        print!("'{}' ", ch);
    }
    println!("");
    println!("len {}", multilingual.len());
    println!("count {}", multilingual.chars().count());

    let maybe = multilingual.find('п');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi {}", hi);
    }
    // let s = "¡";
    // println!("{}", &s[0..1]); // bad, first byte of a multibyte character
    let text = "the red fox and the lazy dog";
    let vs: Vec<&str> = text.split_whitespace().collect();
    println!("vs is {:?}", vs);
    let cs: String = text.chars().filter(|x| !x.is_whitespace()).collect();
    println!("cs is {:?}", cs);

    // ----------arguments----------
    for arg in std::env::args() {
        println!("{}", arg);
    }
    let args: Vec<String> = std::env::args().collect();
    println!("args is: {:?}", args);
    let first = std::env::args().nth(1).expect("please supply an argument");
    let n: String = first.parse().expect("not a string!");
    println!("arg first is: {n}");

    // ----------match----------
    if let Some(idx) = text.find('r') {
        println!("r in text is: {}", idx);
    }
    let n = 10;
    let text = match n {
        1..=3 => "little",
        4..=10 => "medium",
        _ => "unknown",
    };
    println!("text is: {}", text);

    // ----------files----------
    let filename = std::env::args().nth(2).expect("please input a file path");
    let result = readfile(&filename);
    println!("read from file, result: {:?}", result);
    let result2 = readfile2(&filename);
    println!("read from file, result2: {:?}", result2);
}

fn readfile(filename: &str) -> Result<String, std::io::Error> {
    let mut file = match std::fs::File::open(filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => return Err(e),
    }
}

fn readfile2(filename: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn dump(s: &str) {
    println!("s is: {s}");
}

fn sum_arr(array: &[i32]) -> i32 {
    let mut s = 0;
    for i in 0..array.len() {
        s += array[i];
    }
    s
}
