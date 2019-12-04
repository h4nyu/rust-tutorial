pub struct Point {
    pub value: f64,
}

pub struct PointLike {
    pub value: f64,
}

pub struct Human {
    pub age: u8,
}

pub fn func0(p:&Point) -> f64 {
    p.value
}

pub trait Mammals {
    fn do_mammals_work(& self);
}


impl Mammals for Human {
    fn do_mammals_work(& self) {
        println!("人間は哺乳類です。");
    }
}

