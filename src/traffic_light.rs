// 定义交通信号灯的枚举
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 定义一个 trait，表示交通信号灯的时间
trait TrafficLightTimer {
    fn duration(&self) -> u32;
}

// 为枚举 TrafficLight 实现 TrafficLightTimer trait
impl TrafficLightTimer for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}


fn main() {
    // 创建交通信号灯实例
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    // 使用 trait 方法获取不同灯的持续时间
    println!("Duration of Red Light: {} seconds", red_light.duration());
    println!("Duration of Yellow Light: {} seconds", yellow_light.duration());
    println!("Duration of Green Light: {} seconds", green_light.duration());
}
