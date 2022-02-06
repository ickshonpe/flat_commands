# Flat Commands

Extension trait on Bevy Commands for spawning entity hierarchies without nesting.

## Examples

### Before
```rust
fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    transform: Transform::from_xyz(1.0, 1.0, 1.0),
                    ..Default::default()
                })
                .spawn_bundle(PbrBundle {
                    transform: Transform::from_xyz(2.0, 2.0, 2.0),
                    ..Default::default()
                })
        });
}
```
### or
```rust
fn setup(mut commands: Commands) {
    let child_1 = commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .id();
    let child_2 = commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(2.0, 2.0, 2.0),
            ..Default::default()
        })
        .id();
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .push_child(child_1)
        .push_child(child_1);
}
```
### after
```rust
use flat_commands::*;

fn setup(mut commands: Commands) {
    commands
        .spawn_root(PbrBundle {
            transform: Transform::from_xyz(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .with_child(PbrBundle {
            transform: Transform::from_xyz(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .with_sibling(PbrBundle {
            transform: Transform::from_xyz(2.0, 2.0, 2.0),
            ..Default::default()
        });
}
```
#
### Before
```rust
pub fn spawn_text_box(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
 
    commands.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect { left: Val::Px(100.0), bottom: Val::Px(100.0), ..Default::default() },
            padding: Rect::all(Val::Px(2.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|builder| {
        builder.spawn_bundle(NodeBundle {
                color: UiColor (Color::DARK_GRAY),
                style: Style {
                    padding: Rect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            }
        )
        .with_children(|builder| {
            builder.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Hello, world!",
                    TextStyle {
                        font: asset_server.load("FiraMono-Regular.ttf"),
                        font_size: 16.0,
                        color: Color::ANTIQUE_WHITE,
                    },
                    TextAlignment::default()
                ),
                 ..Default::default()
            });
        });
    });
}
```
### after
```rust
use flat_commands::*;

pub fn spawn_text_box(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_root(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect { left: Val::Px(100.0), bottom: Val::Px(100.0), ..Default::default() },
            padding: Rect::all(Val::Px(2.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .with_child(NodeBundle {
        color: UiColor (Color::DARK_GRAY),
        style: Style {
            padding: Rect::all(Val::Px(4.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .with_child(TextBundle {
        text: Text::with_section(
            "Hello, world!",
            TextStyle {
                font: asset_server.load("FiraMono-Regular.ttf"),
                font_size: 16.0,
                color: Color::ANTIQUE_WHITE,
            },
            TextAlignment::default()
        ),
        ..Default::default()
    });
}
```
#
### Before
```rust
fn spawn_branching_hierachy(
    commands: &mut Commands
) -> Entity {
    let id = commands.spawn().id();

    commands.entity(id)
    .with_children(|builder| {
        builder
        .spawn()
        .with_children(|builder| {
            builder
            .spawn()
            .with_children(|builder| {
                builder
                .spawn();

                builder
                .spawn();
            });
        });

        builder
        .spawn()
        .with_children(|builder| {
            builder
            .spawn()
            .with_children(|builder| {
                builder
                .spawn();

                builder
                .spawn();
            });
        });

        builder
        .spawn()
        .with_children(|builder| {
            builder
            .spawn()
            .with_children(|builder| {
                builder
                .spawn();

                builder
                .spawn();
            });
        });
    });

    id
}
```
### after
```rust
use flat_commands::*;

fn spawn_branching_hierachy(
    commands: &mut Commands
) -> Entity {
    commands
    .spawn_empty_root()
    .with_descendants(|local_root| {
        local_root
        .with_empty_child()
        .with_empty_child()
        .with_empty_sibling()
    })
    .with_descendants(|local_root| {
        local_root
        .with_empty_child()
        .with_empty_child()
        .with_empty_sibling()
    })
    .with_descendants(|local_root| {
        local_root
        .with_empty_child()
        .with_empty_child()
        .with_empty_sibling()
    })
    .root_id()
}
```
### or
```rust
use flat_commands::*;

fn spawn_hierachy(
    mut commands: Commands
) -> Entity {
    let root = commands
    .spawn_empty_root();

    root
    .with_empty_child()
    .with_empty_child()
    .with_empty_sibling();
    
    root
    .with_empty_child()
    .with_empty_child()
    .with_empty_sibling();
    
    root
    .with_empty_child()
    .with_empty_child()
    .with_empty_sibling()
    .root_id()
}
```
#
### batched child spawning
```rust
use flat_commands::*;

fn spawn_brood(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn_root(NodeBundle { ..Default::default() })
    .with_child_batch((0..30).map(|i| {
        TextBundle {
            style: Style {
                flex_shrink: 0.,
                size: Size::new(Val::Undefined, Val::Px(20.)),
                margin: Rect {
                    left: Val::Auto,
                    right: Val::Auto,
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                format!("Item {}", i),
                TextStyle {
                    font: font.clone(),
                    font_size: 20.,
                    color: Color::RED,
                },
                Default::default(),
            ),
            ..Default::default()
        }
    );
}
```
#
## Other Info
* Undocumented.
* Untested, probably has bugs.
* Unprofiled, probably slow.
* Also has add_child and push_children.
* No despawning or component removal (use regular commands).
* No unsafe or macros.
