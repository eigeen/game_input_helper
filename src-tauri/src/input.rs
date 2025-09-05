use core::iter::Iterator;
use std::sync::OnceLock;

use enigo::{Direction, Key, Keyboard};
use tokio::sync::Mutex;

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

    pub async fn init(&self) -> eyre::Result<()> {
        *self.enigo.lock().await = Some(enigo::Enigo::new(&enigo::Settings::default())?);
        Ok(())
    }

    pub async fn input_text_chunked(&self, content: &str) -> eyre::Result<()> {
        // 最大5个char为一组，按组调用
        let slices = content
            .chars()
            .collect::<Vec<_>>()
            .chunks(5)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>();

        let mut enigo = self.enigo.lock().await;
        for slice in slices {
            enigo.as_mut().unwrap().text(&slice)?;
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }

        Ok(())
    }

    pub async fn input_key(&self, key: Key) -> eyre::Result<()> {
        let mut enigo = self.enigo.lock().await;
        enigo.as_mut().unwrap().key(key, Direction::Click)?;
        Ok(())
    }
}
