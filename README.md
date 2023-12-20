# audio-select

Simple configurable GUI application to quickly switch default sink and source
for your PulseAudio deamon.

### Configurability

Audio-select uses `libpulse-binding` to get the lists of active pulseaudio sinks
and sources, so any virtual devices and their "`.monitor`"s will appear (some
apps don't show these monitor devices limiting overall configurability). 

There's also a UI for hiding or labeling some of these devices. Hitting save
saves a toml config to the default config folder (usually
~/.config/audio-select/default-config.toml).

### Windowing

The app opens up right under or above your mouse so it can easily integrate
with standalone toolbars like i3blocks.

### Screenshot

![Screenshot](audio-select-screenshot.png)

