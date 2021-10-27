use std::time::Duration;

fn main() {
    let light = SignalLight::Red;
    println!("red duration is {:?}", SignalLight::working_duration(light));
    let yellow = SignalLight::Yellow;
    println!(
        "yellow duration is {:?}",
        SignalLight::working_duration(yellow)
    );
    let green = SignalLight::Green;
    println!(
        "green duration is {:?}",
        SignalLight::working_duration(green)
    );
}

trait TSignal {
    fn working_duration(s: Self) -> Duration;
}

enum SignalLight {
    Red,
    Yellow,
    Green,
}

impl TSignal for SignalLight {
    fn working_duration(s: Self) -> Duration {
        let duration = match s {
            SignalLight::Red => 30,
            SignalLight::Yellow => 5,
            SignalLight::Green => 60,
        };
        Duration::from_secs(duration)
    }
}
