use std::sync::OnceLock;

use enigo::{Direction, Key, Keyboard};
use parking_lot::Mutex;

pub struct Input {
    enigo: Mutex<Option<enigo::Enigo>>,
}

impl Input {
    pub fn global() -> &'static Self {
        static INPUT: OnceLock<Input> = OnceLock::new();

        INPUT.get_or_init(|| Input {
            enigo: Mutex::new(None),
        })
    }

    pub fn init() -> eyre::Result<()> {
        *Self::global().enigo.lock() = Some(enigo::Enigo::new(&enigo::Settings::default())?);
        Ok(())
    }

    pub fn input_text(&self, content: &str) -> eyre::Result<()> {
        let mut enigo = self.enigo.lock();
        enigo.as_mut().unwrap().text(content)?;
        Ok(())
    }

    pub fn input_key(&self, key: Key) -> eyre::Result<()> {
        let mut enigo = self.enigo.lock();
        enigo.as_mut().unwrap().key(key, Direction::Press)?;
        Ok(())
    }
}

// pub fn input_text(content: &str) -> eyre::Result<()> {

//     Ok(())
// }
