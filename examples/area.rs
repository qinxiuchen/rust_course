use std::f64::consts::PI;
fn main() {
    let c = Circle { radius: 2.5 };
    println!("circle area is {}", cal_area(c));
    let s = Squre {
        width: 2.5,
        height: 2.5,
    };
    println!("squre area is {}", cal_area(s));
    let t = Triangle {
        base: 3.5,
        height: 5.5,
    };
    println!("triangle area is {}", cal_area(t));
}

trait Area {
    fn cal(&self) -> f64;
}

fn cal_area<T: Area>(s: T) -> f64 {
    s.cal()
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn cal(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn cal(&self) -> f64 {
        (self.base * self.height) / 2.0
    }
}

struct Squre {
    width: f64,
    height: f64,
}

impl Area for Squre {
    fn cal(&self) -> f64 {
        self.width * self.height
    }
}
