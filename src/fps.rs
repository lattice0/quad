

//! Shows various text layout options.

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

#[derive(Component)]
pub struct TextChanges;

pub fn infotext_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "This text changes in the bottom right",
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    "\nThis text changes in the bottom right - ",
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::RED,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::ORANGE_RED,
                }),
                TextSection::new(
                    " fps, ",
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::YELLOW,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::GREEN,
                }),
                TextSection::new(
                    " ms/frame",
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::BLUE,
                    },
                ),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(TextChanges);
    commands.spawn_bundle(
        TextBundle::from_section(
            "This\ntext has\nline breaks and also a set width in the bottom left",
            TextStyle {
                font,
                font_size: 50.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            size: Size {
                width: Val::Px(200.0),
                ..default()
            },
            ..default()
        }),
    );
}

pub fn change_text_system(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<TextChanges>>,
) {
    for mut text in &mut query {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diagnostic.average() {
                fps = fps_avg;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_avg) = frame_time_diagnostic.average() {
                frame_time = frame_time_avg;
            }
        }

        text.sections[0].value = format!(
            "This text changes in the bottom right - {:.1} fps, {:.3} ms/frame",
            fps,
            frame_time * 1000.0,
        );

        text.sections[2].value = format!("{:.1}", fps);

        text.sections[4].value = format!("{:.3}", frame_time * 1000.0);
    }
}

