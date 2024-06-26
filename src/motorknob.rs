use std::io::Read;

pub struct MotorKnob {
    position_file_handle: std::fs::File,
    end_position_file_handle: std::fs::File,
    start_position_file_handle: std::fs::File,
    detent_position_file_handle: std::fs::File,
}

struct Profile {
    name: String,
    start_position: u16,
    end_position: u16,
    detent_count: u16
}

impl MotorKnob {
    pub fn new() -> anyhow::Result<Self> {
        let mut position_file_handle = std::fs::OpenOptions::new()
            .read(true)
            .write(false)
            .open("/sys/motorknob/position")?;

        let mut position_file_handle = std::fs::OpenOptions::new()
            .read(true)
            .write(false)
            .open("/sys/motorknob/profile/end_position")?;

        let mut position_file_handle = std::fs::OpenOptions::new()
            .read(true)
            .write(false)
            .open("/sys/motorknob/profile/start_position")?;

        let mut position_file_handle = std::fs::OpenOptions::new()
            .read(true)
            .write(false)
            .open("/sys/motorknob/profile/detent_position")?;

        Ok(Self {
            position_file_handle,
            end_position_file_handle,
            start_position_file_handle,
            detent_position_file_handle
        })
    }

    pub fn get_position(&mut self) -> anyhow::Result<u16> {
        let mut raw_position = [0u8; 2];
        self.position_file_handle.read_exact(&mut raw_position)?;

        let mut position = raw_position[0] as u16;
        position |= (raw_position[1] as u16) << 8;

        Ok(position)
    }

}