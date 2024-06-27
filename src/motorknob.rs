use anyhow::bail;
use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;
use serde::Deserialize;

pub struct MotorKnob {
    position_file_handle: std::fs::File,
    end_position_file_handle: std::fs::File,
    start_position_file_handle: std::fs::File,
    detents_file_handle: std::fs::File,
}

// Attempt to use Typestate Pattern
// Turned out to not be very usefull here
// pub struct Linked;
// pub struct Unlinked;

// trait LinkState {}
// impl LinkState for Linked {}
// impl LinkState for Unlinked {}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub name: String,
    pub start_position: u16,
    pub end_position: u16,
    pub detents: u16,
    // motorknob: Option<Cell<MotorKnob>>,
    // _linked: PhantomData<LinkState>
}

// impl<L: LinkState> Profile<L> {
//     pub fn new(name: String, start_position: u16, end_position: u16, detents: u16) ->  Profile<Unlinked> {
//         Profile {
//             name,
//             start_position,
//             end_position,
//             detents,
//             motorknob: None,
//             _linked: PhantomData::<Unlinked>
//         }
//     }
// }

// impl Profile<Unlinked> {
//     pub fn link(self, motorknob: Cell<MotorKnob>) -> Profile<Linked> {
//         Profile {
//             name: self.name,
//             start_position: self.start_position,
//             end_position: self.end_position,
//             detents: self.detents,
//             motorknob: Some(motorknob),
//             _linked: PhantomData::<Linked>
//         }
//     }
// }

// impl Profile<Linked> {
//     pub fn sync(&self) -> anyhow::Result<()> {
//         println!("Syncing");

//         Ok(())
//     }
// }

impl MotorKnob {
    pub fn new() -> anyhow::Result<Self> {
        let position_file_handle = std::fs::OpenOptions::new()
            .read(true)
            .write(false)
            .open("/sys/motorknob/position")?;

        let end_position_file_handle = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/sys/motorknob/profile/end_position")?;

        let start_position_file_handle = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/sys/motorknob/profile/start_position")?;

        let detents_file_handle = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/sys/motorknob/profile/detents")?;

        Ok(Self {
            position_file_handle,
            end_position_file_handle,
            start_position_file_handle,
            detents_file_handle,
        })
    }

    pub fn read_position(&mut self) -> anyhow::Result<u16> {
        let mut raw_position = [0u8; 2];
        self.position_file_handle.seek(SeekFrom::Start(0))?;
        self.position_file_handle.read(&mut raw_position)?;

        let mut position = raw_position[0] as u16;
        position |= (raw_position[1] as u16) << 8;

        Ok(position)
    }

    pub fn read_detents(&mut self) -> anyhow::Result<u16> {
        let mut raw_detents = [0u8; 2];
        self.detents_file_handle.seek(SeekFrom::Start(0))?;
        self.detents_file_handle.read(&mut raw_detents)?;

        let mut detents = raw_detents[0] as u16;
        detents |= (raw_detents[1] as u16) << 8;

        Ok(detents)
    }

    pub fn read_start_pos(&mut self) -> anyhow::Result<u16> {
        let mut raw_start_pos = [0u8; 2];
        self.start_position_file_handle.seek(SeekFrom::Start(0))?;
        self.start_position_file_handle.read(&mut raw_start_pos)?;

        let mut start_pos = raw_start_pos[0] as u16;
        start_pos |= (raw_start_pos[1] as u16) << 8;

        Ok(start_pos)
    }

    pub fn read_end_pos(&mut self) -> anyhow::Result<u16> {
        let mut raw_end_pos = [0u8; 2];
        self.end_position_file_handle.seek(SeekFrom::Start(0))?;
        self.end_position_file_handle.read(&mut raw_end_pos)?;


        let mut end_pos = raw_end_pos[0] as u16;
        end_pos |= (raw_end_pos[1] as u16) << 8;

        Ok(end_pos)
    }

    pub fn write_start_pos(&mut self, start_pos: u16) -> anyhow::Result<()> {
        let count = self
            .start_position_file_handle
            .write(&[(start_pos >> 8) as u8, start_pos as u8])?;
        self.start_position_file_handle.flush()?;
        if count != 2 {
            bail!("Start Position not fully written");
        }
        Ok(())
    }

    pub fn write_end_pos(&mut self, end_pos: u16) -> anyhow::Result<()> {
        let count = self
            .end_position_file_handle
            .write(&[(end_pos >> 8) as u8, end_pos as u8])?;
        self.end_position_file_handle.flush()?;
        if count != 2 {
            bail!("End Position not fully written");
        }
        Ok(())
    }

    pub fn write_detents(&mut self, detents: u16) -> anyhow::Result<()> {
        let count = self
            .detents_file_handle
            .write(&[(detents >> 8) as u8, detents as u8])?;
        self.detents_file_handle.flush()?;
        if count != 2 {
            bail!("Detents not fully written");
        }
        Ok(())
    }

    pub fn read_current_profile(&mut self) -> anyhow::Result<Profile> {
        Ok(Profile {
            name: "-".into(),
            detents: self.read_detents()?,
            end_position: self.read_end_pos()?,
            start_position: self.read_start_pos()?,
        })
    }

    pub fn write_profile(&mut self, profile: &Profile) -> anyhow::Result<()> {
        self.write_start_pos(profile.start_position)?;
        self.write_end_pos(profile.end_position)?;
        self.write_detents(profile.detents)?;
        Ok(())
    }
}
