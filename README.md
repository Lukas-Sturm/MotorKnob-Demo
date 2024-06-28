# MotorKnob Demo
Simple Demo to interface with __MotorKnob-Driver__.  
Uses GPIO Button to switch Profiles and displays position on console.

## Usage
Create `Config.toml` next to binary.  
Define at least one profile.  
```
button_pin = 26

[[profiles]]
name = "Normal"
detents = 80
start_position = 80
end_position = 160
```
Attach a button to GPIO *button_pin*, connect button to ground.

Pressing the button will cycle through the defined `[[profiles]]`.

## Caveat
Only supposed to compile for Linux, `aarch64-unknown-linux-gnu` set as default Target.  
Hardcoded for __MotorKnob-Driver__.  
Required a __MotorKnob__.  