/*
只写 mod 的内容就可以, 不用写 mod{}
*/
pub struct Df {
    pub x: i32,
    y: i32,
}

pub fn new(x: i32, y: i32) -> Df {
    Df { x, y }
}

impl Df {
    pub fn add(&self) -> i32 {
        self.x + self.y
    }
}