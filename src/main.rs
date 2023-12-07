#![windows_subsystem = "windows"]

// TODO: Ai only dodge when there are player bullets in the scene?
// TODO: Ai Rush player when health is low?
// TODO: Ai only shoot when player is in sight?
// TODO: Ai run away when their health is low?
// TODO: Add respawn button when you die https://github.com/bevyengine/bevy/blob/main/examples/ui/button.rs
// TODO: Add k/d ratio at the top of the screen
// TODO: Rounded corners UI

use bevy::{
    prelude::*, 
    window::*, 
    sprite::MaterialMesh2dBundle,
    ecs::system::RunSystemOnce,
};

use std::env; // Detect OS for OS specific keybinds
use dot32_intro::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use rand::Rng;
// use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable, WorldInspectorParams};
mod utils;
use crate::utils::Health;

mod tanks;
use tanks::*;

mod sound;
use sound::*;

mod healthbars;
use healthbars::*;
pub const MAX_HEALTH: u8 = 5;

const TIME_STEP: f64 = 1.0 / 60.0; // FPS
const MUTE: bool = false;

const BULLET_SIZE: f32 = 6.0; 
const KNOCKBACK: f32 = 5.0;

fn main() {
    App::new()
    .add_plugins(
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tiny Tank (Bevy Edition)".into(),
                resolution: WindowResolution::new(800., 600.),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        })
        .build()
    )
    .add_plugins(EmbeddedAssetPlugin::default())
    .insert_resource(ClearColor(Color::rgb(0.7, 0.55, 0.41)))
    .insert_resource(AiKilled { score: 0})

    .add_systems(Startup, (
        create_player,
        create_enemy,
        setup,
    ))
    
    .add_systems(Update, (
        mouse_button_input,
        ai_rotate,
        keep_tanks_on_screen,
        keep_healthbars_on_screen,
        kill_bullets,
        hurt_tanks,
        collide_tanks,
        update_kills_text,
        update_healthbar,
        update_healthbar_border,
    ))

    .add_systems(FixedUpdate, (
        update_bullets,
        movement,
        ai_movement,
    ))
    .insert_resource(Time::<Fixed>::from_seconds(TIME_STEP))

    .add_plugins(Intro)
    .run();
}

fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    println!("{}", env::consts::OS); // Prints the current OS.

    // commands.spawn_bundle(ButtonBundle {
    //     style: Style {
    //         size: Size::new(Val::Px(200.0), Val::Px(45.0)),
    //         // center button
    //         margin: Rect::all(Val::Px(20.0)),
    //         // horizontally center child text
    //         justify_content: JustifyContent::Center,
    //         // vertically center child text
    //         align_items: AlignItems::Center,
    //         ..default()
    //     },
    //     color: Color::ORANGE_RED.into(),
    //     ..default()
    // })
    // .insert(Name::new("Spawn player button"))
    // .with_children(|parent| {
    //     parent.spawn_bundle(TextBundle {
    //         text: Text::with_section(
    //             "Spawn Player",
    //             TextStyle {
    //                 font: asset_server.load("fonts/PT_Sans/PTSans-Regular.ttf"),
    //                 font_size: 40.0,
    //                 color: Color::rgb(0.9, 0.9, 0.9),
    //             },
    //             Default::default(),
    //         ),
    //         ..default()
    //     });
    // });
    // commands.spawn_bundle(ButtonBundle {
    //     style: Style {
    //         size: Size::new(Val::Px(150.0), Val::Px(45.0)),
    //         // center button
    //         margin: Rect::all(Val::Px(20.0)),
    //         // horizontally center child text
    //         justify_content: JustifyContent::Center,
    //         // vertically center child text
    //         align_items: AlignItems::Center,
    //         ..default()
    //     },
    //     color: Color::ORANGE_RED.into(),
    //     ..default()
    // })
    // .insert(Name::new("Spawn AI button"))
    // .with_children(|parent| {
    //     parent.spawn_bundle(TextBundle {
    //         text: Text::with_section(
    //             "Spawn AI",
    //             TextStyle {
    //                 font: asset_server.load("fonts/PT_Sans/PTSans-Regular.ttf"),
    //                 font_size: 40.0,
    //                 color: Color::rgb(0.9, 0.9, 0.9),
    //             },
    //             Default::default(),
    //         ),
    //         ..default()
    //     });
    // });

    // commands.spawn_bundle(TextBundle {
    //     text: Text::with_section(
    //         "Kills: 0",
    //         TextStyle {
    //             font: asset_server.load("fonts/PT_Sans/PTSans-Regular.ttf"),
    //             font_size: 40.0,
    //             color: Color::rgb(0.9, 0.9, 0.9),
    //         },
    //         Default::default(),
    //     ),
    //     ..default()
    // }).insert(KillsText);
    
}

enum TurretOf {
	Player,
    Ai
}

#[derive(Component)]
struct Bullet {
    from: TurretOf,
}

#[derive(Component)]
struct KillsText;

#[derive(Component)]
struct Direction {
    dir: Vec2,
}

#[derive(Resource)]
struct AiKilled{ 
	score: u8,
}

fn create_player(
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(TankBundle::new(&mut meshes, &mut materials, 4)) // "4" is the amount of health we spawn the tank with
    .insert(Player)
    .insert(Name::new("Player"))
    .with_children(|parent| {
        parent.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(16.0).into()).into(),
            material: materials.add(Color::rgb(0.35, 0.6, 0.99).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.1),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(BearingBundle::new())
            .with_children(|parent| {
                parent.spawn(TurretBundle::new());
            });
        });
        parent.spawn(HealthbarBundle::new(4)); // "4" is the max health 
        parent.spawn(HealthbarBorderBundle::new());
    });
}

fn create_enemy(
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in 0..2 {
        commands.spawn(TankBundle::new(&mut meshes, &mut materials, 4)) // "4" is the amount of health we spawn the tank with
        .insert(AiBundle::new())
        .insert(Name::new("Enemy"))
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(16.0).into()).into(),
                material: materials.add(Color::rgb(0.89, 0.56, 0.26).into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.1),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(BearingBundle::new())
                .with_children(|parent| {
                    parent.spawn(TurretBundle::new());
                });
            });
            parent.spawn(HealthbarBundle::new(4)); // "4" is the max health
            parent.spawn(HealthbarBorderBundle::new());
        });
    }
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut positions: Query<(&mut Transform,
    &mut Velocity),
    With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity) in positions.iter_mut() {
        if (keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A)) && LENGTH + FADE + 1.0 < time.elapsed_seconds() as f32  {
            velocity.value.x -= TANK_SPEED;
        }
        if (keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D)) && LENGTH + FADE + 1.0 < time.elapsed_seconds() as f32  {
            velocity.value.x += TANK_SPEED;
        }
        if (keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S)) && LENGTH + FADE + 1.0 < time.elapsed_seconds() as f32  {
            velocity.value.y -= TANK_SPEED;
        }
        if (keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W)) && LENGTH + FADE + 1.0 < time.elapsed_seconds() as f32  {
            velocity.value.y += TANK_SPEED;
        }

        velocity.value *= 0.9;

        transform.translation += velocity.value.extend(0.0);
    }
}

fn keep_tanks_on_screen(
    mut tanks: Query<(&mut Transform, &mut Velocity, Option<&mut DirectionAi>), With<Tank>>,
    // primary_window: Query<&Window, With<PrimaryWindow>>
    primary_window: Query<&Window, With<PrimaryWindow>>
) {
    let Ok(window) = primary_window.get_single() else {
        return;
    };
    for (mut tank, mut velocity, direction) in tanks.iter_mut() {

        let mut tempdir = 5;

        if tank.translation.x + TANK_SIZE > window.width() - window.width()/2.0 {
            velocity.value.x = 0.0;
            tank.translation.x = window.width()/2.0 - TANK_SIZE;
            tempdir = 0;
        }
        if tank.translation.x - TANK_SIZE < -window.width()/2.0 {
            velocity.value.x = 0.0;
            tank.translation.x = -window.width()/2.0 + TANK_SIZE;
            tempdir = 1;
        }
        if tank.translation.y + TANK_SIZE > window.height() - window.height()/2.0 {
            velocity.value.y = 0.0;
            tank.translation.y = window.height()/2.0 - TANK_SIZE;
            tempdir = 2;
        }
        if tank.translation.y - TANK_SIZE < -window.height()/2.0 {
            velocity.value.y = 0.0;
            tank.translation.y = -window.height()/2.0 + TANK_SIZE;
            tempdir = 3;
        }

        match direction {
            Some(mut x) => {
                if tempdir < 5 {
                    x.value = tempdir;
                }
            },
            None    => (),
        }
    }
}

fn collide_tanks(
    mut tanks: Query<&mut Transform, With<Tank>>
) {
    // Create a vector that is as long as the number of tanks
    let mut movements = vec![Vec2::new(0.0, 0.0); tanks.iter().count()];
    // Find the movement of each tank
    for (i, tank) in tanks.iter().enumerate() {
        for (j, sibling) in tanks.iter().enumerate() {
            if tank != sibling {
                let distance = distance_between(&tank.translation.truncate(), &sibling.translation.truncate());
                if distance < TANK_SIZE * 2.0 {
                    // Gets the direction and how far it should move
                    let direction = (tank.translation.truncate() - sibling.translation.truncate()).normalize();
                    let move_len = (TANK_SIZE * 2.0) - distance;

                    // Adds required movement into the vector
                    movements[i] = direction * move_len * 0.5;
                    movements[j] = direction * move_len * -0.5;
                }
            }
        }
    }

    // Apply the movement to the tanks
    for (i, mut tank) in tanks.iter_mut().enumerate() {
        tank.translation += movements[i].extend(0.0);
    }
}

fn ai_movement(
    time: Res<Time>,
    mut positions: Query<(&mut Transform, &mut Velocity, &mut Steps, &mut DirectionAi, &Active), With<Ai>>,
) {
    for (mut transform, mut velocity, mut steps, mut direction, active) in positions.iter_mut() {
        if steps.value < 0.0 {
            direction.value = rand::thread_rng().gen_range(0 ..= 4) as u8;
            steps.value = rand::thread_rng().gen_range(0 ..= 110) as f32 / 110.0;
        }
        if direction.value == 0 && LENGTH + FADE + 1.0 < time.elapsed_seconds() as f32 && active.value == true {
            velocity.value.x -= TANK_SPEED;
        }
        if direction.value == 1 && LENGTH + FADE + 1.0 < time.elapsed_seconds() as f32 && active.value == true {
            velocity.value.x += TANK_SPEED;
        }
        if direction.value == 2 && LENGTH + FADE + 1.0 < time.elapsed_seconds() as f32 && active.value == true {
            velocity.value.y -= TANK_SPEED;
        }
        if direction.value == 3 && LENGTH + FADE + 1.0 < time.elapsed_seconds() as f32 && active.value == true {
            velocity.value.y += TANK_SPEED;
        }

        velocity.value *= 0.9;

        transform.translation += velocity.value.extend(0.0);

        steps.value -= time.delta_seconds();
    }
}


fn mouse_button_input( // Shoot bullets and rotate turret to point at mouse
    buttons: Res<Input<MouseButton>>, 
    primary_window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    
    mut commands: Commands,
    // world: &mut World,
    mut positions: Query<(&mut Transform, &mut AttackTimer, &Children), With<Player>>,
    mut tank_child_query: Query<&Children, (Without<Player>, Without<Turret>, Without<Bearing>)>,
    mut bearings: Query<(&mut Transform, &Children), (With<Bearing>, Without<Player>, Without<Turret>)>,
    mut transform_query: Query<&mut Transform, (With<Turret>, Without<Player>)>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok(window) = primary_window.get_single() else {
        return;

    };
    if let Some(_position) = window.cursor_position() {
        match Some(_position) {
            Some(vec) => {
                for (player, mut attack_timer, children) in positions.iter_mut() {
                    let window_size = Vec2::new(window.width(), window.height());
                    // let diff = Vec3::new(vec.x - window.width()/2.0, vec.y - window.height()/2.0, 0.) - player.translation;
                    let mut mouse_coords = vec;
                    // flip the mouse Y position
                    mouse_coords.y = mouse_coords.y*-1.0 + window_size.y;
                    let diff = mouse_coords.extend(0.0) - window_size.extend(0.0)/2.0 - player.translation;
                    let angle = diff.y.atan2(diff.x); // Add/sub FRAC_PI here optionally

                    for child in children.iter() {
                        if let Ok(tank_child) = tank_child_query.get_mut(*child) {
                            for bearing in tank_child.iter() {
                                if let Ok((mut joint, turrets)) = bearings.get_mut(*bearing) {
                                    joint.rotation = Quat::from_rotation_z(angle);
                                    for turret in turrets.iter() {
                                        if let Ok(mut transform) = transform_query.get_mut(*turret) {
                                            transform.translation.x += ((TANK_SIZE+4.0)-transform.translation.x)*0.1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if buttons.pressed(MouseButton::Left) && attack_timer.value > 0.4  && LENGTH + FADE + 1.0 < time.elapsed_seconds() as f32  {
                        attack_timer.value = 0.0;
                        if !MUTE {
                            // Goofy ahh work around to world being exclusive
                            commands.add( |world: &mut World| {
                                world.run_system_once(play_gunshot)
                            })
                        }

                        for child in children.iter() {
                            if let Ok(tank_child) = tank_child_query.get_mut(*child) {
                                for bearing in tank_child.iter() {
                                    if let Ok((mut joint, turrets)) = bearings.get_mut(*bearing) {
                                        joint.rotation = Quat::from_rotation_z(angle);
                                        for turret in turrets.iter() {
                                            if let Ok(mut transform) = transform_query.get_mut(*turret) {
                                                transform.translation.x = TANK_SIZE+4.0 - 10.0;
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        commands.spawn((
                            MaterialMesh2dBundle {
                                mesh: meshes.add(shape::Circle::new(BULLET_SIZE).into()).into(),
                                material: materials.add(ColorMaterial::from(Color::BLACK)),
                                transform: Transform::from_translation(Vec3::new(player.translation.x, player.translation.y, 0.0)),
                                ..default()
                            },
                            Name::new("Bullet"),
                            Bullet {from: TurretOf::Player},
                            Direction{dir:(mouse_coords - player.translation.truncate() - window_size/2.0).normalize()},
                        ));
                    }

                    attack_timer.value += time.delta_seconds()
                }

            },
            None => println!("Cursor outside of screen, but window is still in focus?"),
        }
    }
}

fn ai_rotate( // Shoot bullets and rotate turret to point at mouse
    time: Res<Time>,
    players: Query<&Transform, (Without<Ai>, With<Player>)>,
    mut commands: Commands,
    mut positions: Query<(&mut Transform, &mut AttackTimer, &Children, &mut Active), With<Ai>>,
    mut tank_child_query: Query<&Children, (Without<Ai>, Without<Turret>, Without<Bearing>)>,
    mut bearings: Query<(&mut Transform, &Children), (With<Bearing>, Without<Player>, Without<Ai>, Without<Turret>)>,
    mut transform_query: Query<&mut Transform, (With<Turret>, Without<Ai>, Without<Player>)>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (ai, mut attack_timer, children, mut active) in positions.iter_mut() {
        if active.value == true {
            let mut player_count = 0;
            for player in players.iter() {
                // let window_size = Vec2::new(window.width(), window.height());
                let diff = Vec3::new(player.translation.x, player.translation.y, 0.) - ai.translation;
                // let diff = vec.extend(0.0) - window_size.extend(0.0)/2.0 - ai.translation;
                let angle = diff.y.atan2(diff.x); // Add/sub FRAC_PI here optionally
                // ai.rotation = Quat::from_rotation_z(angle);
                for child in children.iter() {
                    if let Ok(tank_child) = tank_child_query.get_mut(*child) {
                        for bearing in tank_child.iter() {
                            if let Ok((mut joint, turrets)) = bearings.get_mut(*bearing) {
                                joint.rotation = Quat::from_rotation_z(angle);
                                for turret in turrets.iter() {
                                    if let Ok(mut transform) = transform_query.get_mut(*turret) {
                                        transform.translation.x += ((TANK_SIZE+4.0)-transform.translation.x)*0.1;
                                    }
                                }
                            }
                        }
                    }
                }

                if attack_timer.value < 0.0 && LENGTH + FADE + 1.0 < time.elapsed_seconds() as f32 {
                    attack_timer.value =rand::thread_rng().gen_range(5 ..= 14) as f32 /10.0 ;
                    if !MUTE {
                        // Goofy ahh work around to world being exclusive
                        commands.add( |world: &mut World| {
                            world.run_system_once(play_gunshot)
                        })
                    }
                    for child in children.iter() {
                        if let Ok(tank_child) = tank_child_query.get_mut(*child) {
                            for bearing in tank_child.iter() {
                                if let Ok((mut joint, turrets)) = bearings.get_mut(*bearing) {
                                    joint.rotation = Quat::from_rotation_z(angle);
                                    for turret in turrets.iter() {
                                        if let Ok(mut transform) = transform_query.get_mut(*turret) {
                                            transform.translation.x = TANK_SIZE+4.0 - 10.0;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Circle::new(BULLET_SIZE).into()).into(),
                            material: materials.add(ColorMaterial::from(Color::BLACK)),
                            transform: Transform::from_translation(Vec3::new(ai.translation.x, ai.translation.y, 0.0)),
                            ..default()
                        },
                        Name::new("Bullet"),
                        Bullet {from: TurretOf::Ai},
                        Direction{dir:(player.translation.truncate() - ai.translation.truncate()).normalize()},
                    ));
                }

                if LENGTH + FADE + 1.0 < time.elapsed_seconds() as f32 {
                    attack_timer.value -= time.delta_seconds();
                }
                player_count += 1;
            }
            if player_count == 0 {
                active.value = false;
            }
        } else {
            for child in children.iter() {
                if let Ok(tank_child) = tank_child_query.get_mut(*child) {
                    for bearing in tank_child.iter() {
                        if let Ok((_joint, turrets)) = bearings.get_mut(*bearing) {
                            for turret in turrets.iter() {
                                if let Ok(mut transform) = transform_query.get_mut(*turret) {
                                    transform.translation.x += ((TANK_SIZE+4.0)-transform.translation.x)*0.1;
                                }
                            }
                        }
                    }
                }
            }
            for _ in players.iter() {
                active.value = true
            }
        }
    }

}

fn keep_healthbars_on_screen(
    mut healthbar: Query<(&mut Transform, &GlobalTransform), (With<Healthbar>, Without<HealthbarBorder>)>,
    mut healthbar_border: Query<(&mut Transform, &GlobalTransform), (With<HealthbarBorder>, Without<Healthbar>)>,
    primary_window: Query<&Window, With<PrimaryWindow>>
) {
    let Ok(window) = primary_window.get_single() else {
        return;

    };

    for (mut transform, global_transform) in healthbar.iter_mut() {
        let ceiling = window.height()/2.0 - 18.0/2.0;
        let player_height = global_transform.translation().y - transform.translation.y;
        transform.translation.y = (ceiling - player_height).min(HEALTHBAR_Y_OFFSET);
    }
    for (mut transform, global_transform) in healthbar_border.iter_mut() {
        let ceiling = window.height()/2.0 - 18.0/2.0;
        let player_height = global_transform.translation().y - transform.translation.y;
        transform.translation.y = (ceiling - player_height).min(HEALTHBAR_Y_OFFSET);
    }
}

fn hurt_tanks(
    mut commands: Commands,
    bullets: Query<(&Transform, Entity, &Bullet), (Without<Player>, Without<Ai>, With<Bullet>)>,
    mut ais: Query<(&Transform, Entity, &mut Health, &mut Velocity), (Without<Player>, With<Ai>, Without<Bullet>)>,
    mut players: Query<(&mut Transform, Entity, &mut Health, &mut Velocity), (With<Player>, Without<Ai>, Without<Bullet>)>,
    mut ai_killed: ResMut<AiKilled>, 
) {
    for (bullet_transform, bullet_entity, bullet_type) in bullets.iter() {
        match bullet_type.from {
            TurretOf::Player => {
                for (ai_transform, ai_entity, mut ai_health, mut velocity) in ais.iter_mut() {
                    if distance_between(&ai_transform.translation.truncate(), &bullet_transform.translation.truncate()) < TANK_SIZE+BULLET_SIZE {
                        let knockback = (ai_transform.translation - bullet_transform.translation).truncate().normalize()*KNOCKBACK;
                        velocity.value += knockback;

                        if ai_health.value > 1 {
                            ai_health.value -= 1;
                        } else {
                            commands.entity(ai_entity).despawn_recursive(); 
                            ai_killed.score += 1;
                        }
                        commands.entity(bullet_entity).despawn(); 
                        if !MUTE {
                            // Goofy ahh work around to world being exclusive
                            commands.add( |world: &mut World| {
                                world.run_system_once(play_tankhit)
                            })
                        }
                    }
                }
            }
            TurretOf::Ai => {
                for (player_transform, player_entity, mut player_health, mut velocity) in players.iter_mut() {
                    if distance_between(&player_transform.translation.truncate(), &bullet_transform.translation.truncate()) < TANK_SIZE+BULLET_SIZE {
                        let knockback = (player_transform.translation - bullet_transform.translation).truncate().normalize()*KNOCKBACK;
                        velocity.value += knockback;

                        if player_health.value > 1 {
                            player_health.value -= 1;
                        } else {
                            commands.entity(player_entity).despawn_recursive(); 
                            ai_killed.score += 0;
                        }
                        commands.entity(bullet_entity).despawn(); 
                        if !MUTE {
                            // Goofy ahh work around to world being exclusive
                            commands.add( |world: &mut World| {
                                world.run_system_once(play_tankhit)
                            })
                        }
                    }
                }
            }
        }
    }
}

fn update_bullets(mut bullets: Query<(&mut Transform, &Direction), With<Bullet>>,) {
    for (mut transform, direction) in bullets.iter_mut() {
        transform.translation.x += direction.dir.x*10.;
        transform.translation.y += direction.dir.y*10.;
    }
}

fn kill_bullets(
    mut commands: Commands,
    mut bullets: Query<((&mut Transform, Entity), With<Bullet>)>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = primary_window.get_single() else {
        return;

    };

    for ((transform, bullet_entity), _bullet) in bullets.iter_mut() {
        if transform.translation.x.abs() > window.width()/2. || transform.translation.y.abs() > window.height()/2. { 
            commands.entity(bullet_entity).despawn(); 
            if !MUTE {
                // Goofy ahh work around to world being exclusive
                commands.add( |world: &mut World| {
                    world.run_system_once(play_wallhit)
                })
            }
        }
    }
}

// fn toggle_inspector(
//     input: ResMut<Input<KeyCode>>,
//     mut window_params: ResMut<WorldInspectorParams>,
// ) {
//     if input.just_pressed(KeyCode::Grave) {
//         window_params.enabled = !window_params.enabled
//     }
// }

// fn button_system(
//     mut interaction_query: Query<(&Interaction, &mut UiColor, &Children), (Changed<Interaction>, With<Button>),>,
//     active_ai: Query<&mut Active>,
//     mut commands: Commands,
//     text_query: Query<&mut Text>,
// ) {
//     for (interaction, mut color, children) in interaction_query.iter_mut() {
//         let text = text_query.get(children[0]).unwrap();
//         match *interaction {
//             Interaction::Clicked => {
//                 if text.sections[0].value == "Spawn Player" {
//                     let mut no_players = false;
//                     for active in active_ai.iter() {
//                         // If an AI is innactive, then there must be no players
//                         if !active.value {
//                             no_players = true;
//                         }
//                     }
//                     if no_players {
//                         // Spawn player
//                         println!("Spawn Player");
//                         commands.spawn_bundle(TankBundle::new(Color::rgb(0.35, 0.6, 0.99)))
//                         .insert(Player)
//                         .insert(Name::new("Player"))
//                         .with_children(|parent| {
//                             parent.spawn_bundle(BearingBundle::new())
//                             .with_children(|parent| {
//                                 parent.spawn_bundle(TurretBundle::new());
//                             });
//                             parent.spawn_bundle(HealthbarBundle::new());
//                             parent.spawn_bundle(HealthbarBorderBundle::new());
//                         });
//                     }
//                 } else if text.sections[0].value == "Spawn AI" {
//                     println!("Spawning AI");

//                     commands.spawn_bundle(TankBundle::new(Color::ORANGE))
//                     .insert_bundle(AiBundle::new())
//                     .insert(Name::new("Enemy"))
//                     .with_children(|parent| {
//                         parent.spawn_bundle(BearingBundle::new())
//                         .with_children(|parent| {
//                             parent.spawn_bundle(TurretBundle::new());
//                         });
//                         parent.spawn_bundle(HealthbarBundle::new());
//                         parent.spawn_bundle(HealthbarBorderBundle::new());
//                     });
//                 }
//                 *color = Color::MAROON.into();
//             }
//             Interaction::Hovered => {
//                 *color = Color::RED.into();
//             }
//             Interaction::None => {
//                 *color = Color::ORANGE_RED.into();
//             }
//         }
//     }
// }

fn update_kills_text(
    ai_killed: ResMut<AiKilled>,
    mut kills: Query<&mut Text, With<KillsText>>,
) {
    for mut kills_text in kills.iter_mut() {
        // println!("{}", ai_killed.score);
        kills_text.sections[0].value = format!("Kills: {}", ai_killed.score);
    }
}

fn distance_between(point1: &Vec2, point2: &Vec2) -> f32 {
    let diff = *point1 - *point2; // (Your assumption *was* correct btw, but this works)
    // (diff.x.powf(2.0) + diff.y.powf(2.0)).sqrt()
    diff.length()
}
