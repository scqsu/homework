pub trait GetTime {
    fn getTime(&self) -> u32;
}

pub enum TrafficLight {
    RedLight,
    GreenLight,
    YelloLight,
}

impl GetTime for TrafficLight {
    fn getTime(&self) -> u32 {
        match self {
            TrafficLight::GreenLight => 100,
            TrafficLight::RedLight => 50,
            TrafficLight::YelloLight => 5,
        }
    }
}
