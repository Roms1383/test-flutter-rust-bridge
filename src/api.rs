use std::sync::Mutex;

use kira::{
    manager::{AudioManager, AudioManagerSettings},
    sound::static_sound::StaticSoundSettings,
};
use kira_cpal::CpalBackend;
use lazy_static::lazy_static;
use anyhow::anyhow;

lazy_static! {
    pub static ref AUDIO: Mutex<AudioManager<CpalBackend>> = {
        Mutex::new(
            AudioManager::new(
                CpalBackend::new().expect("audio cpal backend"),
                AudioManagerSettings::default(),
            )
            .expect("audio manager"),
        )
    };
}

#[allow(dead_code)]
pub enum Screen {
    Splash,
    Home,
}

#[allow(dead_code)]
pub fn play(screen: Screen) -> anyhow::Result<()> {
    let filename = match screen {
        Screen::Home => "assets/sound/home-screen.mp3",
        Screen::Splash => "assets/sound/splash-screen.mp3",
    };
    let sound = kira_loaders::load(filename, StaticSoundSettings::default());
    if let Ok(sound) = sound {
        match AUDIO.lock().expect("static audio manager").play(sound) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(anyhow!("failed to play sound")),
        }
    }
    Err(anyhow!("failed to load sound"))
}
