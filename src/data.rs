use crate::pulsewrapper::PulseWrapper;
use druid::{Data, Lens};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::process::Command;
use std::sync::Arc;
use std::vec::Vec;

#[derive(Clone, PartialEq, Data)]
pub enum AudioDeviceType {
    SOURCE,
    SINK,
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    sources: Arc<Vec<AudioDeviceState>>,
    sinks: Arc<Vec<AudioDeviceState>>,
    pub default_source: String,
    pub default_sink: String,
    pulsewrapper: Arc<RefCell<PulseWrapper>>,
    pub use_dark_theme: bool,
}

#[derive(Clone, Data, Lens)]
pub struct AudioDeviceState {
    pub label: String,
    pub name: String,
    pub device_type: AudioDeviceType,
    pub hidden: bool,
    pub pulsewrapper: Arc<RefCell<PulseWrapper>>,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct AppConfig {
    pub sources: Vec<AudioDeviceConfig>,
    pub sinks: Vec<AudioDeviceConfig>,
    pub use_dark_theme: bool,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct AudioDeviceConfig {
    pub name: String,
    pub label: String,
    pub hidden: bool,
}

impl AppState {
    pub fn new() -> Self {
        let mut pulsewrapper = PulseWrapper::new();
        pulsewrapper.connect();

        let (default_source, default_sink) = pulsewrapper.get_defaults();
        let pa_sources = pulsewrapper.get_sources();
        let pa_sinks = pulsewrapper.get_sinks();
        let mut pa_source_map = pa_sources.into_iter().collect::<BTreeMap<_, _>>();
        let mut pa_sink_map = pa_sinks.into_iter().collect::<BTreeMap<_, _>>();

        let pulsewrapper = Arc::new(RefCell::new(pulsewrapper));

        let mut config: AppConfig = confy::load_path("AudioSelect.toml").unwrap();
        let mut sources = Vec::new();
        let mut sinks = Vec::new();

        for source in &mut config.sources {
            let not_found = pa_source_map.remove(&source.name).is_none();
            sources.push(AudioDeviceState {
                name: source.name.clone(),
                label: source.label.clone(),
                device_type: AudioDeviceType::SOURCE,
                hidden: source.hidden || not_found,
                pulsewrapper: pulsewrapper.clone(),
            });
        }
        for sink in &mut config.sinks {
            let not_found = pa_sink_map.remove(&sink.name).is_none();
            sinks.push(AudioDeviceState {
                name: sink.name.clone(),
                label: sink.label.clone(),
                device_type: AudioDeviceType::SINK,
                hidden: sink.hidden || not_found,
                pulsewrapper: pulsewrapper.clone(),
            });
        }

        for (source, label) in pa_source_map {
            sources.push(AudioDeviceState {
                name: source.clone(),
                label: label.clone(),
                device_type: AudioDeviceType::SOURCE,
                hidden: false,
                pulsewrapper: pulsewrapper.clone(),
            });
        }
        for (sink, label) in pa_sink_map {
            sinks.push(AudioDeviceState {
                name: sink.clone(),
                label: label.clone(),
                device_type: AudioDeviceType::SINK,
                hidden: false,
                pulsewrapper: pulsewrapper.clone(),
            });
        }

        AppState {
            sources: Arc::new(sources),
            sinks: Arc::new(sinks),
            default_source: default_source,
            default_sink: default_sink,
            pulsewrapper: pulsewrapper,
            use_dark_theme: config.use_dark_theme,
        }
    }

    pub fn restart(&mut self) {
        self.pulsewrapper.borrow_mut().disconnect();
        let _ = Command::new("pulseaudio").arg("-k").status();
        *self = Self::new();
    }

    pub fn save_config(&mut self) {
        // save config
        // *self = Self::new();
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            // default_source_name: Some(String::from("test")),
            sources: Vec::new(),
            sinks: Vec::new(),
            use_dark_theme: true,
        }
    }
}
