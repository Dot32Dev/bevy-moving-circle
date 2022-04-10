use bevy::prelude::*;
use bevy::window::*;
use bevy::app::AppExit; // For MacOS CMD+W to quit keybind
use bevy::core::FixedTimestep;

use bevy_prototype_lyon::prelude::*; // Draw circles with ease
use std::env; // Detect OS for OS specific keybinds

const TIME_STEP: f32 = 1.0 / 120.0;

fn main() {
    App::new()
    .add_startup_system(setup_camera)
    // .insert_resource(Msaa { samples: 4 })
    .insert_resource(WindowDescriptor {
            title: "Tiny Tank (bevy edition)".to_string(),
            width: 800.,
            height: 600.,
            vsync: true,
            ..Default::default()
        })
    .insert_resource(ClearColor(Color::rgb(0.7, 0.55, 0.41)))
    .add_startup_system(create_player)
    .add_plugins(DefaultPlugins)
    .add_plugin(ShapePlugin)
    .add_system(quit_and_resize)
    .add_system(mouse_button_input)
    .add_system(kill_bullets)
    .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(update_bullets)
                .with_system(movement)
        )
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    println!("{}", env::consts::OS); // Prints the current OS.
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity {
    value: Vec2,
}

#[derive(Component)]
struct Turret;

fn create_player(mut commands: Commands) {
    let shape = shapes::RegularPolygon { // Define circle
        sides: 30,
        feature: shapes::RegularPolygonFeature::Radius(18.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::rgb(0.35, 0.6, 0.99)),
            outline_mode: StrokeMode::new(Color::BLACK, 4.0),
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..Default::default()
        },
    ))
    .insert(Player)
    .insert(Velocity { value: Vec2::new(2.0, 0.0) } )
    .with_children(|parent| { // Add turret to player
            parent.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0., 0., 0.),
                    ..Default::default()
                },
                transform: Transform {
                    scale: Vec3::new(16.0, 16.0, 0.),
                    translation: Vec3::new(24.0, 0.0, -1.0),
                    ..Default::default()
                },
                ..Default::default()
            }).insert(Turret);
        });
}

fn movement(keyboard_input: Res<Input<KeyCode>>, mut positions: Query<(&mut Transform, &mut Velocity), With<Player>>,) {
    for (mut transform, mut velocity) in positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            velocity.value.x -= 0.37;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            velocity.value.x += 0.37;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            velocity.value.y -= 0.37;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            velocity.value.y += 0.37;
        }

        velocity.value *= 0.9;

        transform.translation += velocity.value.extend(0.0);
    }
}

fn quit_and_resize(keyboard_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>, mut windows: ResMut<Windows>,) {
    let window = windows.get_primary_mut().unwrap();

    if env::consts::OS == "macos" {
        if keyboard_input.pressed(KeyCode::LWin) && keyboard_input.just_pressed(KeyCode::W) {
            exit.send(AppExit);
            window.set_mode(WindowMode::Windowed);
        }
        if keyboard_input.pressed(KeyCode::LWin) && keyboard_input.pressed(KeyCode::LControl) && keyboard_input.just_pressed(KeyCode::F) {
            println!("{:?}", window.mode());
            if window.mode() == WindowMode::Windowed {
                window.set_mode(WindowMode::BorderlessFullscreen);
            } else if window.mode() == WindowMode::BorderlessFullscreen {
                window.set_mode(WindowMode::Windowed);
            }
        }
    }
    if env::consts::OS == "windows" {
        if keyboard_input.just_pressed(KeyCode::F11) {
            if window.mode() == WindowMode::Windowed {
                window.set_mode(WindowMode::BorderlessFullscreen);
            } else if window.mode() == WindowMode::BorderlessFullscreen {
                window.set_mode(WindowMode::Windowed);
            }
        }
    }
}

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Direction {
    dir: Vec2,
}

fn mouse_button_input( // Shoot bullets and rotate turret to point at mouse
    buttons: Res<Input<MouseButton>>, 
    windows: Res<Windows>, 
    mut commands: Commands,
    mut positions: Query<&mut Transform, With<Player>>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(_position) = window.cursor_position() {
        match Some(_position) {
            Some(vec) => {
                for mut player in positions.iter_mut() {
                    // let window_size = (window.width(), window.height());
                    let diff = Vec3::new(vec.x - window.width()/2.0, vec.y - window.height()/2.0, 0.) - player.translation;
                    let angle = diff.y.atan2(diff.x); // Add/sub FRAC_PI here optionally
                    player.rotation = Quat::from_rotation_z(angle);

                    if buttons.just_pressed(MouseButton::Left) {
                        println!("x{}, y{}", vec.x, vec.y);
                        let shape = shapes::RegularPolygon {
                            sides: 30,
                            feature: shapes::RegularPolygonFeature::Radius(6.0),
                            ..shapes::RegularPolygon::default()
                        };
                        commands.spawn_bundle(GeometryBuilder::build_as(
                            &shape,
                            DrawMode::Fill (
                                FillMode::color(Color::BLACK),
                            ),
                            Transform {
                                translation: Vec3::new(player.translation.x, player.translation.y, 0.0),
                                ..Default::default()
                            },
                        )).insert(Bullet)
                        .insert(Direction { dir: Vec2::new(vec.x - player.translation.x - window.width()/2.0, vec.y - player.translation.y - window.height()/2.0).normalize() });
                    }
                }

            },
            None => println!("Cursor outside of screen, but window is still in focus?"),
        }
    }
}

fn update_bullets(mut bullets: Query<(&mut Transform, &Direction), With<Bullet>>,) {
    for (mut transform, direction) in bullets.iter_mut() {
        transform.translation.x += direction.dir.x*10.;
        transform.translation.y += direction.dir.y*10.;
    }
}

fn kill_bullets(mut commands: Commands, mut bullets: Query<((&mut Transform, Entity), With<Bullet>)>, windows: Res<Windows>,) {
    let window = windows.get_primary().unwrap();

    for ((transform, bullet_entity), _bullet) in bullets.iter_mut() {
        if transform.translation.x.abs() > window.width()/2. || transform.translation.y.abs() > window.height()/2. { 
            commands.entity(bullet_entity).despawn(); 
        }
    }
}