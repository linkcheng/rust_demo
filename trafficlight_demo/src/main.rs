fn main() {
    let time = TrafficLight::Green.lighten();
    println!("green lightens {:?}s", time);

    let time = TrafficLight::Yellow.lighten();
    println!("yellow lightens {:?}s", time);

    let time = TrafficLight::Red.lighten();
    println!("red lightens {:?}s", time);
}

enum TrafficLight {
    Green,
    Yellow,
    Red,
}

trait TrafficLighter {
    fn lighten(&self) -> u8;
}


impl TrafficLighter for TrafficLight {
    fn lighten(&self) -> u8 {
        match self {
            TrafficLight::Green => 30u8,
            TrafficLight::Yellow => 3u8,
            TrafficLight::Red => 20u8,
        }
    }
}