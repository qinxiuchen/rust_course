use std::time::Duration;

fn main() {
    let red = SignalLight::Red;
    println!("red duration is {:?}", red.working_duration());
    let yellow = SignalLight::Yellow;
    println!(
        "yellow duration is {:?}",
        yellow.working_duration()
    );
    let green = SignalLight::Green;
    println!(
        "green duration is {:?}",
        green.working_duration()
    );
}

trait TSignal {
    fn working_duration(&self) -> Duration;
}

enum SignalLight {
    Red,
    Yellow,
    Green,
}

impl TSignal for SignalLight {
    fn working_duration(&self) -> Duration {
        let duration = match self {
            SignalLight::Red => 30,
            SignalLight::Yellow => 5,
            SignalLight::Green => 60,
        };
        Duration::from_secs(duration)
    }
}
