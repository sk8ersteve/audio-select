use crate::pulsewrapper::{PulseWrapper, PulseWrapperError};
use druid::{Data, Lens, ExtEventSink};
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
    pub ready: bool,
    pub not_ready_string: String,
    sources: Arc<Vec<AudioDeviceState>>,
    sinks: Arc<Vec<AudioDeviceState>>,
    pub default_source: String,
    pub default_sink: String,
    pulsewrapper: Arc<RefCell<PulseWrapper>>,
    pub use_dark_theme: bool,
    pub close_on_leave: bool,
}

#[derive(Clone, Data, Lens)]
pub struct AudioDeviceState {
    pub label: String,
    pub name: String,
    pub device_type: AudioDeviceType,
    pub connected: bool, // true if device is recognized by pulseaudio
    pub hidden: bool, // true if user decides to hide device
    pub pulsewrapper: Arc<RefCell<PulseWrapper>>,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct AppConfig {
    pub use_dark_theme: bool,
    pub sources: Vec<AudioDeviceConfig>,
    pub sinks: Vec<AudioDeviceConfig>,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct AudioDeviceConfig {
    pub name: String,
    pub label: String,
    pub hidden: bool,
}

impl AppState {
    pub fn new() -> Self {
        let mut config: AppConfig = confy::load("audio-select", None).unwrap();

        let mut pulsewrapper = PulseWrapper::new();

        if let PulseWrapperError::Err = pulsewrapper.connect() {
            return AppState {
                ready: false,
                not_ready_string: String::from("Failed to connect to PulseAudio"),
                sources: Arc::new(Vec::new()),
                sinks: Arc::new(Vec::new()),
                default_source: String::new(),
                default_sink: String::new(),
                pulsewrapper: Arc::new(RefCell::new(pulsewrapper)),
                use_dark_theme: config.use_dark_theme,
                close_on_leave: true,
            };
        }

        let (default_source, default_sink) = pulsewrapper.get_defaults();
        let pa_sources = pulsewrapper.get_sources();
        let pa_sinks = pulsewrapper.get_sinks();
        let mut pa_source_map = pa_sources.into_iter().collect::<BTreeMap<_, _>>();
        let mut pa_sink_map = pa_sinks.into_iter().collect::<BTreeMap<_, _>>();

        let pulsewrapper = Arc::new(RefCell::new(pulsewrapper));

        let mut sources = Vec::new();
        let mut sinks = Vec::new();

        for source in &mut config.sources {
            let connected = pa_source_map.remove(&source.name).is_some();
            sources.push(AudioDeviceState {
                name: source.name.clone(),
                label: source.label.clone(),
                device_type: AudioDeviceType::SOURCE,
                connected: connected,
                hidden: source.hidden,
                pulsewrapper: pulsewrapper.clone(),
            });
        }
        for sink in &mut config.sinks {
            let connected = pa_sink_map.remove(&sink.name).is_some();
            sinks.push(AudioDeviceState {
                name: sink.name.clone(),
                label: sink.label.clone(),
                device_type: AudioDeviceType::SINK,
                connected: connected,
                hidden: sink.hidden,
                pulsewrapper: pulsewrapper.clone(),
            });
        }

        for (source, label) in pa_source_map {
            sources.push(AudioDeviceState {
                name: source.clone(),
                label: label.clone(),
                device_type: AudioDeviceType::SOURCE,
                connected: true,
                hidden: false,
                pulsewrapper: pulsewrapper.clone(),
            });
        }
        for (sink, label) in pa_sink_map {
            sinks.push(AudioDeviceState {
                name: sink.clone(),
                label: label.clone(),
                device_type: AudioDeviceType::SINK,
                connected: true,
                hidden: false,
                pulsewrapper: pulsewrapper.clone(),
            });
        }

        AppState {
            ready: true,
            not_ready_string: String::new(),
            sources: Arc::new(sources),
            sinks: Arc::new(sinks),
            default_source: default_source,
            default_sink: default_sink,
            pulsewrapper: pulsewrapper,
            use_dark_theme: config.use_dark_theme,
            close_on_leave: true,
        }
    }

    pub fn restart_async(&mut self, ext_ctx: ExtEventSink) {
        self.ready = false;
        self.not_ready_string = String::from("Restarting PulseAudio");
        ext_ctx.add_idle_callback(move |data: &mut Self| {
            data.restart();
        })
    }

    pub fn restart(&mut self) {
        self.pulsewrapper.borrow_mut().disconnect();
        let _ = Command::new("pulseaudio").arg("-k").status();
        *self = Self::new();
    }

    pub fn save_config(&mut self) {
        let config = AppConfig {
            use_dark_theme: self.use_dark_theme,
            sources: self
                .sources
                .iter()
                .map(|x| x.convert_to_config())
                .collect::<Vec<_>>(),
            sinks: self
                .sinks
                .iter()
                .map(|x| x.convert_to_config())
                .collect::<Vec<_>>(),
        };
        confy::store("audio-select", None, config).expect("FAIL");
    }
}

impl AudioDeviceState {
    fn convert_to_config(&self) -> AudioDeviceConfig {
        AudioDeviceConfig {
            name: self.name.clone(),
            label: self.label.clone(),
            hidden: self.hidden,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            // default_source_name: Some(String::from("test")),
            use_dark_theme: true,
            sources: Vec::new(),
            sinks: Vec::new(),
        }
    }
}
