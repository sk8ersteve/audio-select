use crate::data::{AppState, AudioDeviceState, AudioDeviceType};
use druid::widget::LensWrap;
use druid::widget::{
    Button, Checkbox, CrossAxisAlignment, Either, Flex, Label, List, Scroll, SizedBox, Split, TextBox,
};
use druid::Size;
use druid::WindowConfig;
use druid::{Env, EventCtx, Widget, WidgetExt};

pub fn build_ui() -> impl Widget<AppState> {
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
    let restart_buton = Button::new("Restart").on_click(|_ctx, data: &mut AppState, _env| {
        data.restart();
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
        |data, _env| data.1.hidden,
        SizedBox::empty(),
        Button::new(|data: &(String, AudioDeviceState), _: &Env| {
            let len = data.1.label.len();
            if len > 35 {
                format!("{}...{}", &data.1.label[..20], &data.1.label[len - 15..])
            } else {
                format!("{}", &data.1.label)
            }
        })
        .on_click(
            |_: &mut EventCtx, data: &mut (String, AudioDeviceState), _: &Env| {
                let mut pulsewrapper = data.1.pulsewrapper.borrow_mut();
                match data.1.device_type {
                    AudioDeviceType::SOURCE => {
                        pulsewrapper.set_default_source(&data.1.name);
                    }
                    AudioDeviceType::SINK => {
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
    Flex::column()
        .with_child(Label::new("Settings"))
        // .with_child(TextBox::new().lens(AppState::default_source))
        .with_child(LensWrap::new(
            List::new(build_source_config),
            AppState::sources,
        ))
}

fn build_source_config() -> impl Widget<AudioDeviceState> {
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
                .fix_width(590.0)
                .padding(5.0),
        )
        .with_child(
            Checkbox::new("Hide").lens(AudioDeviceState::hidden)
        )
}
