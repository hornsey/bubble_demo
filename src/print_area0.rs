
// 定义枚举类，包含圆形、三角形、正方形
enum SHAPE {
    Circle(f32),
    Triangle(f32, f32),
    Rectangle(f32),
}

// 定义一个trait, 计算面积

trait Area {
    fn area(&self) -> f32;
}

impl Area for SHAPE {
    fn area(&self) -> f32 {
        match self {
            SHAPE::Circle(r) => 3.14 * r * r,
            SHAPE::Triangle(a, b) => (a * b) / 2.0,
            SHAPE::Rectangle(w) => w * w,
        }
    }
}

fn main() {
    let circle = SHAPE::Circle(10.0);
    let triangle = SHAPE::Triangle(10.0, 10.0);
    let rectangle = SHAPE::Rectangle(10.0);

    println!("The area of circle is {}", circle.area());
    println!("The area of triangle is {}", triangle.area());
    println!("The area of rectangle is {}", rectangle.area());
}
