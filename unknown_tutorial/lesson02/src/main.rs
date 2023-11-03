/*
ref: https://stevedonovan.github.io/rust-gentle-intro/2-structs-enums-lifetimes.html
*/

fn main() {
    let x = 10;
    let y = x;
    println!("x is: {}", x); // 基本类型是不会被移动的
    println!("y is: {}", y); // 基本类型是不会被移动的

    let s1 = "123";
    let s2 = s1;
    println!("s1 is: {}", s1); // &str 也不会被移动
    println!("s2 is: {}", s2); // &str 也不会被移动, 这也是为什么函数传递的时候需要借用 &

    let str1 = "123".to_string();
    let str2 = str1;
    // println!("str1 is: {}", str1); // string 会被移动
    println!("str2 is: {}", str2); // string 会被移动

    /*
       String
       | addr | ---------> Call me Ishmael.....
       | size |                    |
       | cap  |                    |
                                   |
       &str                        |
       | addr | -------------------|
       | size |

       f64
       | 8 bytes |

       因为 String 涉及到更多的内存
    */

    // ----------Tuples----------
    let r = add_mul(10, 20);
    println!("add_mul, 0: {}", r.0);
    println!("add_mul, 1: {}", r.1);
    for t in ["1", "2", "3"].iter().enumerate() {
        println!("enumerate: {}, {}", t.0, t.1);
    }
    let arr1 = [1, 2, 3];
    let arr2 = ["11", "22", "33"];
    for t in arr1.iter().zip(arr2.iter()) {
        println!("enumerate: {}, {}", t.0, t.1);
    }

    // ----------struct----------

    let p1 = Person {
        name: "yiiguo".to_string(),
        age: 10,
    };
    println!("{:?}", p1);
    println!("name is: {}", p1.name);
    println!("age is: {}", p1.age);

    let mut p2 = Person::new("baby", 10); // 必须设置为 mut 才可以 set_name
    println!("{:?}", p2);
    println!("name is: {}", p2.name);
    println!("age is: {}", p2.age);
    println!("full info --> {}", p2.full_info());
    p2.set_name("kimroniny");
    println!("name is: {}", p2.name);

    // ----------lifetime----------
    let sa: A = A { s: "hi-lifetime" };
    println!("sa is: {:?}", sa);

    // ----------traits----------
    let x = 10;
    println!("x show: {}", x.show());

    // ----------Generic Functions----------
    dump(&p2);
    dump(&10);
    println!("mul(10) = {:?}", mul(10));
    println!("mul(11.1) = {:?}", mul(11.1));

    // ----------Enums----------
    let drt: Direction = Direction::Up;
    println!("direction is: {:?}", drt.as_str());
    let sex: &Sex = &Sex::Boy;
    let sex_int = (*sex) as i32;
    println!("sex({:?}) as int: {}", *sex, sex_int);
    println!("equal? --> {}", *sex == Sex::Boy);
    match sex {
        Sex::Boy => println!("match(sex) -> Boy"),
        Sex::Girl => println!("match(sex) -> Girl"),
    }

    let v_i: Value = Value::Int(10);
    let v_s: Value = Value::Str("hello-value".to_string());
    let v_b: Value = Value::Bool(true);
    println!("v_i: {}", match_value(&v_i));
    println!("v_s: {}", match_value(&v_s));
    println!("v_b: {}", match_value(&v_b));

    // ----------More about Matching----------
    let t = (1, "123455".to_string());
    // let (a, b) = t; // 这种方式, t 的值会被 moved
    // println!("t is: {:?}", t);
    let (ref a, ref b) = t; // 这种方式, t 不会被 moved, (a,b) 只是 borrowed from t;
    println!("t is: {:?}", t);
    println!("a is: {:?}, b is {:?}", a, b);
    let t1 = (1, 2);
    let (x, y) = t1; // 因为 t1 的所有元素都是基本类型, 所以不会被 moved
    println!("t1 is: {:?}", t1);
    println!("x is: {:?}, y is {:?}", x, y);

    // ----------Closure----------
    let f = |x| x * x;
    let ans = f(10);
    println!("ans is {:?}", ans);

    let mut s = "123";
    let mut change = || s = "456";
    change();
    assert_eq!(s, "456");
    println!("s is: {}", s);

    let mut s2 = "456".to_string();
    let change2 = move || {
        s2 = "56+".to_string();
        s2
    };
    change2();
    // assert_eq!(s2, "56+"); // 前面已经把 s2 move 到了 change2 中, 所以这里便不可以再使用了

    let sin2: Vec<i32> = [1, 2, 3, 4].iter().map(|x| x * 2).collect();
    println!("sin2: {:?}", sin2);
    let sum2: i32 = [1, 2, 3, 4].iter().map(|x| x * 3).sum();
    println!("sum2: {}", sum2);
    let sum3: i32 = [1, 2, 3, 4].iter().filter(|x| *x % 2 == 0).sum();
    println!("sum3: {}", sum3);

    // ----------Dynamic Struct Pointer----------
    let mut root = Node::new("I-am-root");
    let left = Node::new("I-am-left-1");
    root.set_left(left);
    println!("struct: {:#?}", root);
    let key1 = "I-am-left-1";
    let key2 = "I-am-left-2";
    println!("look key1: {}", root.look(key1));
    println!("look key2: {}", root.look(key2));
    println!("look2 key1: {}", root.look2(key1));
    println!("look2 key2: {}", root.look2(key2));
    root.insert(key2);
    println!("look key1: {}", root.look(key1));
    println!("look key2: {}", root.look(key2));
    println!("look2 key1: {}", root.look2(key1));
    println!("look2 key2: {}", root.look2(key2));
}

type NodeBox = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    value: String,
    left: NodeBox,
    right: NodeBox,
}

impl Node {
    fn new(value: &str) -> Node {
        Node {
            value: value.to_string(),
            left: None,
            right: None,
        }
    }

    fn boxer(node: Node) -> NodeBox {
        Some(Box::new(node))
    }

    fn set_left(&mut self, node: Node) {
        self.left = Node::boxer(node);
    }

    fn set_right(&mut self, node: Node) {
        self.right = Node::boxer(node);
    }

    fn insert(&mut self, data: &str) {
        if data < &self.value {
            match self.left {
                Some(ref mut left) => left.insert(data), // 直接用 ref 就相当于取 self.left 的引用
                None => self.set_left(Node::new(data)),
            }
        } else {
            match self.right {
                Some(ref mut right) => right.insert(data),
                None => self.set_right(Node::new(data)),
            }
        }
    }

    fn look(&self, data: &str) -> bool {
        if &self.value == data {
            return true;
        }
        let mut flag = match self.left {
            Some(ref left) => left.look(data),
            None => false,
        };
        if !flag {
            flag = match self.right {
                Some(ref right) => right.look(data),
                None => false,
            };
        }
        return flag;
    }

    fn look2(&self, data: &str) -> bool {
        if &self.value == data {
            return true;
        }
        let mut flag = if let Some(ref left) = self.left {
            left.look2(data)
        } else {
            false
        };
        if !flag {
            flag = if let Some(ref right) = self.right {
                right.look2(data)
            } else {
                false
            };
        }
        return flag;
    }
}

fn match_value(v: &Value) -> String {
    use Value::*;
    match v {
        Int(n) => format!("value is {}", n),
        Str(s) => format!("value is {}", s),
        Bool(x) => format!("value is {}", x),
    }
}

#[derive(Debug)]
enum Value {
    Int(i32),
    Str(String),
    Bool(bool),
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_str(&self) -> &str {
        match *self {
            Direction::Up => "UP",
            Direction::Down => "DOWN",
            Direction::Left => "LEFT",
            _ => "RIGHT",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Sex {
    Boy = 1,
    Girl,
}

fn dump<T>(value: &T)
where
    T: std::fmt::Debug, // 必须加上 where, 即 value 必须得实现 debug trait
{
    println!("dump value: {:?}", value);
}

fn mul<T>(x: T) -> T::Output
where
    T: std::ops::Mul + Copy, // 由于使用了两次 x, 第一次使用的时候已经 move 了, 所以第二次要 Copy
{
    x * x
}

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("i32 -> {}", self)
    }
}

impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "refmt -> {}", self.full_info())
    }
}

#[derive(Debug)] // 必须添加才能 println!()
struct A<'a> {
    s: &'a str,
}

// fn make_a() -> A<'static> {
//     let s = "hei-make-a".to_string();
//     A { s: &s } // 这里会报错, 因为 s 引用了一个临时变量, 当该函数执行完成后就会被销毁的临时变量
// }

// #[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: &str, age: i32) -> Person {
        Person {
            name: name.to_string(),
            age: age,
        }
    }

    fn full_info(&self) -> String {
        format!("name is {}, age is {}", self.name, self.age)
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn set_age(&mut self, age: i32) {
        self.age = age;
    }
}

fn add_mul(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}
