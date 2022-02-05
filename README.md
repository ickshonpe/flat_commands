# Flat Commands

Facade over Bevy Commands for spawning entity hierarchies without nesting.

## Examples
#
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

fn setup(mut flat_commands: FlatCommands) {
    flat_commands
        .spawn_root(PbrBundle {
            transform: Transform::from_xyz(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .spawn_child(PbrBundle {
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
    mut commands: FlatCommands,
    asset_server: Res<AssetServer>,
) {
    flat_commands.spawn_root(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect { left: Val::Px(100.0), bottom: Val::Px(100.0), ..Default::default() },
            padding: Rect::all(Val::Px(2.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .spawn_child(NodeBundle {
        color: UiColor (Color::DARK_GRAY),
        style: Style {
            padding: Rect::all(Val::Px(4.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .spawn_child(TextBundle {
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
    let id = commands.spawn_bundle(X::default()).id();

    commands.entity(id)
    .with_children(|builder| {
        builder
        .spawn_bundle(X::default())
        .with_children(|builder| {
            builder
            .spawn_bundle(X::default())
            .with_children(|builder| {
                builder
                .spawn_bundle(X::default());

                builder
                .spawn_bundle(X::default());
            });
        });

        builder
        .spawn_bundle(X::default())
        .with_children(|builder| {
            builder
            .spawn_bundle(X::default())
            .with_children(|builder| {
                builder
                .spawn_bundle(X::default());

                builder
                .spawn_bundle(X::default());
            });
        });

        builder
        .spawn_bundle(X::default())
        .with_children(|builder| {
            builder
            .spawn_bundle(X::default())
            .with_children(|builder| {
                builder
                .spawn_bundle(X::default());

                builder
                .spawn_bundle(X::default());
            });
        });
    });

    id
}
```
### after
```rust
fn spawn_branching_hierachy(
    flat_commands: &mut FlatCommands
) -> Entity {
    flat_commands
    .spawn_root(X::default())
    .with_descendants(|local_root| {
        local_root
        .spawn_child(X::default())
        .spawn_child(X::default())
        .with_sibling(X::default())
    })
    .with_descendants(|local_root| {
        local_root
        .spawn_child(X::default())
        .spawn_child(X::default())
        .with_sibling(X::default())
    })
    .with_descendants(|local_root| {
        local_root
        .spawn_child(X::default())
        .spawn_child(X::default())
        .with_sibling(X::default())
    })
    .root_id()
}
```
### or
```rust
fn spawn_hierachy(
    mut flat_commands: FlatCommands
) -> Entity {
    let root = flat_commands
    .spawn_root(X::default());

    root
    .spawn_child(X::default())
    .spawn_child(X::default())
    .with_sibling(X::default());
    
    root
    .spawn_child(X::default())
    .spawn_child(X::default())
    .with_sibling(X::default());
    
    root
    .spawn_child(X::default())
    .spawn_child(X::default())
    .with_sibling(X::default())
    .root_id()
}
```
#
### Commands access
```rust
fn access_commands(mut commands: Commands) {
    let player_entity = commands.flat_commands()
        .spawn_root(PlayerCharacter::default()
        .spawn_child(Sword::default())
        .with_sibling(Lantern::default())
        .parent_id();

    commands.insert_resource(PlayerEntity(player_entity));

    let flat_commands: FlatCommands = commands.flat_commands();
    flat_commands.remove_resource::<PlayerEntity>();

    let commands: Commands = flat_commands.take_commands();
    commands.entity(player_entity).insert(Hitpoints(25));
}
```
#
## Other Info
* Untested, probably has bugs.
* Unprofiled, probably slow.
* Has add_child and push_children.
* No removal or insertion functionality (but accessible by flat_commands.commands()).
