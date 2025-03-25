use crate::data::{AppState, AudioDeviceState, AudioDeviceType};
use druid::widget::{
    Button, Checkbox, CrossAxisAlignment, Either, Flex, Label, LensWrap, List, Scroll, SizedBox,
    Split, TextBox,
};
use druid::{Env, EventCtx, Size, Widget, WidgetExt, WindowConfig};

pub fn build_ui() -> impl Widget<AppState> {
    Either::new(
        |data, _env| data.ready,
        build_devices_screen(),
        Label::new(|data: &String, _env: &_| format!("{}:", data))
            .lens(AppState::not_ready_string)
            .center(),
    )
}

fn build_devices_screen() -> impl Widget<AppState> {
    let body = Flex::column()
        .with_child(Label::new("Input").padding(5.0).center())
        .with_child(LensWrap::new(
            List::new(build_device_button),
            (AppState::default_source, AppState::sources),
        ))
        .with_child(Label::new("Output").padding(5.0).center())
        .with_child(LensWrap::new(
            List::new(build_device_button),
            (AppState::default_sink, AppState::sinks),
        ));
    let settings_button = Button::new("Settings").on_click(|ctx, data: &mut AppState, env| {
        data.close_on_leave = false;
        ctx.new_sub_window(
            WindowConfig::default()
                // .set_level(WindowLevel::AppWindow)
                .window_size(Size::new(600.0, 600.0))
                .resizable(false),
            build_config_menu(),
            data.clone(),
            env.clone(),
        );
    });
    let restart_buton = Button::new("Restart").on_click(|ctx, data: &mut AppState, _env| {
        data.restart_async(ctx.get_external_handle());
    });

    // Flex::column()
    //     .with_flex_child(Scroll::new(body).vertical(), 1.0)
    //     .with_child(Flex::row().with_child(settings_button).with_child(restart_buton))
    Split::rows(
        Scroll::new(body).vertical(),
        Flex::row()
            .with_child(settings_button)
            .with_child(restart_buton),
    )
    .split_point(0.9)
    .bar_size(3.0)
    .solid_bar(true)
}

fn build_device_button() -> impl Widget<(String, AudioDeviceState)> {
    Either::new(
        |data, _env| data.1.hidden || !data.1.connected,
        SizedBox::empty(),
        Button::new(|data: &(String, AudioDeviceState), _: &Env| {
            get_shortened_label(&data.1.label)
        })
        .on_click(
            |_: &mut EventCtx, data: &mut (String, AudioDeviceState), _: &Env| {
                let mut pulsewrapper = data.1.pulsewrapper.borrow_mut();
                match data.1.device_type {
                    AudioDeviceType::Source => {
                        pulsewrapper.set_default_source(&data.1.name);
                    }
                    AudioDeviceType::Sink => {
                        pulsewrapper.set_default_sink(&data.1.name);
                    }
                }
                data.0 = data.1.name.clone();
            },
        )
        .disabled_if(|data, _env| data.0 == data.1.name)
        .fix_size(290.0, 45.0)
        .padding(5.0),
    )
}

fn build_config_menu() -> impl Widget<AppState> {
    let body = Flex::column()
        .with_child(Label::new("Settings"))
        // .with_child(TextBox::new().lens(AppState::default_source))
        .with_child(Label::new("Input Devices:"))
        .with_child(List::new(build_device_config).lens(AppState::sources))
        .with_child(Label::new("Output Devices:"))
        .with_child(List::new(build_device_config).lens(AppState::sinks));
    let save_button =
        Button::new("Save").on_click(|_: &mut EventCtx, data: &mut AppState, _: &Env| {
            data.save_config();
        });
    Split::rows(
        Scroll::new(body).vertical(),
        Flex::row().with_child(save_button),
    )
    .split_point(0.9)
    .bar_size(3.0)
    .solid_bar(true)
}

fn build_device_config() -> impl Widget<AudioDeviceState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Label::new(|data: &String, _env: &_| format!("{}:", data))
                .lens(AudioDeviceState::name)
                .padding(5.0),
        )
        .with_child(
            TextBox::new()
                .lens(AudioDeviceState::label)
                .padding(5.0)
                .fix_width(590.0),
        )
        .with_child(
            Checkbox::new("Hide")
                .lens(AudioDeviceState::hidden)
                .padding(5.0),
        )
        .padding((0.0, 5.0))
        .disabled_if(|data, _env| !data.connected)
}

fn get_shortened_label(label: &String) -> String {
    if label.is_ascii() {
        let len = label.len();
        if len > 35 {
            format!("{}...{}", &label[..20], &label[len - 15..])
        } else {
            label.to_string()
        }
    } else {
        let mut i: usize = 0;
        let mut gap_start: usize = 0;
        let mut gap_end: usize = 0;
        let len = label.chars().count();
        if len > 35 {
            for (c_i, _) in label.char_indices() {
                if i == 20 {
                    gap_start = c_i;
                } else if i == len - 15 {
                    gap_end = c_i;
                    break;
                }
                i += 1;
            }
            format!("{}...{}", &label[..gap_start], &label[gap_end..])
        } else {
            label.to_string()
        }
    }
}
