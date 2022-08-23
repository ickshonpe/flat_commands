use bevy::prelude::*;
use flat_commands::*;

fn spawn_text_box(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_root(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect { left: Val::Px(100.0), bottom: Val::Px(100.0), ..Default::default() },
            padding: UiRect::all(Val::Px(2.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .with_child(NodeBundle {
        color: UiColor (Color::DARK_GRAY),
        style: Style {
            padding: UiRect::all(Val::Px(4.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .with_child(TextBundle {
        text: Text::from_section(
            "Hello, world!",
            TextStyle {
                font: asset_server.load("FiraMono-Regular.ttf"),
                font_size: 16.0,
                color: Color::ANTIQUE_WHITE,
            },
        ),
        ..Default::default()
    });
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(
        |mut commands: Commands| { commands.spawn_bundle(Camera2dBundle::default()); }
    )    
    .add_startup_system(spawn_text_box)
    .run();
}