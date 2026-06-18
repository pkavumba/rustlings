// Start small: unit variants represent one of several named states. Use a
// `match` expression to turn each traffic light into the number of seconds it
// should stay on.

#[derive(Debug, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn duration_in_seconds(light: TrafficLight) -> u8 {
    match light {
        TrafficLight::Red => 60,
        TrafficLight::Yellow => 5,
        TrafficLight::Green => 45,
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_unit_variants() {
        assert_eq!(duration_in_seconds(TrafficLight::Red), 60);
        assert_eq!(duration_in_seconds(TrafficLight::Yellow), 5);
        assert_eq!(duration_in_seconds(TrafficLight::Green), 45);
    }
}
