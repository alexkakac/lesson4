//为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同

fn main() {
    let redlight = Trafficlight::Red;
    let yellowlight = Trafficlight::Yellow;
    let greenlight = Trafficlight::Green;
    println!("redlight is: {}",redlight.redtime());
    println!("yellowlight is: {}",yellowlight.yellowtime());
    println!("greenlight is: {}",greenlight.greentime());
}

pub trait Time {
    fn redtime(&self) -> u8;
    fn yellowtime(&self) -> u8;
    fn greentime(&self) -> u8;
}

enum Trafficlight {
    Red,
    Green,
    Yellow,
}

impl Time for Trafficlight {
    fn redtime(&self) -> u8 {
        60
    }
    fn yellowtime(&self) -> u8 {
        100
    }
    fn greentime(&self) -> u8 {
        80
    }
}