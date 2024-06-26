mod motorknob;

use gpiod::{Chip, Bias, Options, EdgeDetect};
use motorknob::MotorKnob;

fn main() -> anyhow::Result<()> {
    let mut motor_knob = MotorKnob::new()?;

    println!("position: {}", motor_knob.get_position()?);

    // let chip = Chip::new("gpiochip0")?;

    // let opts = Options::input([26])
    //     .bias(Bias::PullUp)
    //     .edge(EdgeDetect::Falling)
    //     .consumer("change-profile-input");

    // let mut inputs = chip.request_lines(opts)?;

    // loop {
    //     let event = inputs.read_event()?;

    //     println!("event: {:?}", event);
    // }

    Ok(())
}