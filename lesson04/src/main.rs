mod foo; // foo 是文件 foo.rs 的名称
mod boo;

fn main() {
    let df = foo::new(10, 20);
    println!("df.x: {}", df.x);
    println!("df.add: {}", df.add());
    println!("boo.answer: {}", boo::answer());
    println!("boo.boo2.answer2: {}", boo::boo2::answer2());
}
