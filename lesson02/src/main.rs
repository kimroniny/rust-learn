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
}

fn add_mul(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}
