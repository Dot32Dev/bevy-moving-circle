use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_prototype_lyon::prelude::*;


fn main() {
    App::new()
    .add_startup_system(setup_camera)
    // .insert_resource(Msaa { samples: 4 })
    .add_startup_system(create_player)
    .add_plugins(DefaultPlugins)
    .add_plugin(ShapePlugin)
    .add_system(movement)
    .add_system(quit).add_system(mouse_button_input)
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Component)]
struct Player;

fn create_player(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 30,
        feature: shapes::RegularPolygonFeature::Radius(30.0),
        ..shapes::RegularPolygon::default()
    };

    // commands.spawn_bundle(SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::rgb(0.7, 0.7, 0.7),
    //         ..Default::default()
    //     },
    //     transform: Transform {
    //         scale: Vec3::new(30.0, 30.0, 30.0),
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // }).insert(Player);

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Fill (
            FillMode::color(Color::YELLOW),
            // outline_mode: StrokeMode::new(Color::BLACK, 5.0),
        ),
        Transform::default(),
    )).insert(Player);
}

// fn movement(mut positions: Query<(&Player, &mut Transform)>) {
//     for (_head, mut transform) in positions.iter_mut() {
//         transform.translation.x += 2.;
//     }
// }

fn movement(keyboard_input: Res<Input<KeyCode>>, mut positions: Query<&mut Transform, With<Player>>,) {
    for mut transform in positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= 3.;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += 3.;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= 3.;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += 3.;
        }
    }
}

fn quit(keyboard_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::LWin) && keyboard_input.pressed(KeyCode::W) {
        exit.send(AppExit);
    }
}

#[derive(Component)]
struct Bullet;

fn mouse_button_input(buttons: Res<Input<MouseButton>>, windows: Res<Windows>, mut commands: Commands) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(_position) = window.cursor_position() {
            // println!("{:?}", window.cursor_position());
            match Some(_position) {
                Some(vec) => {
                    // let window_size = (window.width(), window.height());
                    println!("x{}, y{}", vec.x, vec.y);
                    let shape = shapes::RegularPolygon {
                        sides: 30,
                        feature: shapes::RegularPolygonFeature::Radius(10.0),
                        ..shapes::RegularPolygon::default()
                    };
                    commands.spawn_bundle(GeometryBuilder::build_as(
                        &shape,
                        DrawMode::Fill (
                            FillMode::color(Color::ORANGE),
                        ),
                        Transform {
                            translation: Vec3::new(vec.x-window.width()/2.0, vec.y-window.height()/2.0, 0.0),
                            ..Default::default()
                        },
                    )).insert(Bullet);

                },
                None => println!("Cursor outside of screen, but screen is still in focus?"),
            }
        }
    }
}

// fn create_player(mut commands: Commands) {
//     let shape = shapes::RegularPolygon {
//         sides: 30,
//         feature: shapes::RegularPolygonFeature::Radius(30.0),
//         ..shapes::RegularPolygon::default()
//     };

//     // commands.spawn_bundle(SpriteBundle {
//     //     sprite: Sprite {
//     //         color: Color::rgb(0.7, 0.7, 0.7),
//     //         ..Default::default()
//     //     },
//     //     transform: Transform {
//     //         scale: Vec3::new(30.0, 30.0, 30.0),
//     //         ..Default::default()
//     //     },
//     //     ..Default::default()
//     // }).insert(Player);

//     commands.spawn_bundle(GeometryBuilder::build_as(
//         &shape,
//         DrawMode::Fill (
//             FillMode::color(Color::YELLOW),
//             // outline_mode: StrokeMode::new(Color::BLACK, 5.0),
//         ),
//         Transform::default(),
//     )).insert(Player);
// }
