use std::f64::consts::PI;

// 定义一个 Shape trait
trait Shape {
    fn area(&self) -> f64;
}

// 实现 Circle 结构体，并为其实现 Shape trait
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

// 实现 Triangle 结构体，并为其实现 Shape trait
struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 实现 Square 结构体，并为其实现 Shape trait
struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 打印图形面积的函数，接受实现了 Shape trait 的类型
fn print_area<T: Shape>(shape: T) {
    println!("Area: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 10.0 };
    let triangle = Triangle { base: 10.0, height: 10.0 };
    let square = Square { side: 10.0 };

    // 打印不同图形的面积
    print_area(circle);
    print_area(triangle);
    print_area(square);
}
