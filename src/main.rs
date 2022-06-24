use bevy::prelude::*;
use bevy::window::*;
use bevy::app::AppExit; // For MacOS CMD+W to quit keybind
use bevy::core::FixedTimestep;

use bevy_prototype_lyon::prelude::*; // Draw circles with ease
use std::env; // Detect OS for OS specific keybinds
use dot32_intro::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use rand::Rng;
use bevy_inspector_egui::{WorldInspectorPlugin, Inspectable, RegisterInspectable};

const TIME_STEP: f32 = 1.0 / 120.0; // FPS
const MUTE: bool = true;

const TANK_SPEED: f32 = 0.37;
const TANK_SIZE: f32 = 20.0; 

const BULLET_SIZE: f32 = 6.0; 

const HEALTHBAR_WIDTH: f32 = 50.0;
const MAX_HEALTH: u8 = 5;

fn main() {
    App::new()
    // .insert_resource(Msaa { samples: 4 })
    .insert_resource(WindowDescriptor {
            title: "Tiny Tank (bevy edition)".to_string(),
            width: 800.,
            height: 600.,
            present_mode: PresentMode::Fifo, // Vesync enabled, replace Fifo with Mailbox for no vsync
            ..default()
        })
    .insert_resource(ClearColor(Color::rgb(0.7, 0.55, 0.41)))
    .add_startup_system(create_player)
    .add_startup_system(create_enemy)
    .add_startup_system(setup)
    .add_plugins_with(DefaultPlugins, |group| {
        group.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
    })
    .add_plugin(WorldInspectorPlugin::new())
    .register_inspectable::<Health>() // tells bevy-inspector-egui how to display the struct in the world inspector
    .add_plugin(ShapePlugin)
    .add_system(quit_and_resize)
    .add_system(mouse_button_input)
    .add_system(ai_rotate)
    .add_system(collision)
    .add_system(kill_bullets)
    .add_system(hurt_tanks)
    .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(update_bullets)
                .with_system(movement)
                .with_system(ai_movement)
        )
    .add_plugin(Intro)
    .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    println!("{}", env::consts::OS); // Prints the current OS.
    
    let gunshot = asset_server.load("ShotsFired.ogg");
    commands.insert_resource(GunshotSound(gunshot));
    let gunshot_deep = asset_server.load("ShotsFiredDeep.ogg");
    commands.insert_resource(GunshotDeepSound(gunshot_deep));

    let tank_hit = asset_server.load("TankHit.ogg");
    commands.insert_resource(TankHitSound(tank_hit));
    let tank_hit_deep = asset_server.load("TankHitDeep.ogg");
    commands.insert_resource(TankHitDeepSound(tank_hit_deep));

    let wall_hit = asset_server.load("WallHit.ogg");
    commands.insert_resource(WallHitSound(wall_hit));
    let wall_hit_deep = asset_server.load("WallHitDeep.ogg");
    commands.insert_resource(WallHitDeepSound(wall_hit_deep));
}

struct GunshotSound(Handle<AudioSource>);
struct GunshotDeepSound(Handle<AudioSource>);

struct TankHitSound(Handle<AudioSource>);
struct TankHitDeepSound(Handle<AudioSource>);

struct WallHitSound(Handle<AudioSource>);
struct WallHitDeepSound(Handle<AudioSource>);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ai;

#[derive(Component)]
struct Tank;

#[derive(Component)]
struct Velocity {
    value: Vec2,
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
struct Direction {
    dir: Vec2,
}

#[derive(Component)]
struct AttackTimer {
    value: f32,
}


#[derive(Inspectable, Component)]
struct Health {
    value: u8,
}

#[derive(Component)]
struct Steps {
    value: f32,
}

#[derive(Component)]
struct DirectionAi {
    value: u8,
}

#[derive(Component)]
struct Turret;

#[derive(Component)]
struct Bearing;

#[derive(Component)]
struct Healthbar;

fn create_player(mut commands: Commands) {
    let shape = shapes::RegularPolygon { // Define circle
        sides: 30,
        feature: shapes::RegularPolygonFeature::Radius(TANK_SIZE),
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
            ..default()
        },
    ))
    .insert(Player)
    .insert(Tank)
    .insert(AttackTimer { value: 0.0 } ) 
    .insert(Health { value: MAX_HEALTH } ) 
    .insert(DirectionAi { value: 0 } ) // required so that the actual ai can update its direction upon collision
    .insert(Velocity { value: Vec2::new(2.0, 0.0) } )
    .with_children(|parent| { // Add turret to player
        parent.spawn_bundle(GeometryBuilder::build_as( // turret swivvel 
            &shape,
            DrawMode::Fill(FillMode::color(Color::NONE)),
            Transform {
                scale: Vec3::new(1.0, 1.0, 1.0),
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
        )).insert(Bearing).with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0., 0., 0.),
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(16.0, 16.0, 0.),
                    translation: Vec3::new(TANK_SIZE+4.0, 0.0, -1.0),
                    ..default()
                },
                ..default()
            }).insert(Turret);
        });
        parent.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(50.0, 10.0, 0.),
                translation: Vec3::new(0.0, 40.0, 0.0),
                ..default()
            },
            ..default()
        }).insert(Healthbar);
    });
}

fn create_enemy(mut commands: Commands) {
    let shape = shapes::RegularPolygon { // Define circle
        sides: 30,
        feature: shapes::RegularPolygonFeature::Radius(TANK_SIZE),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::ORANGE),
            outline_mode: StrokeMode::new(Color::BLACK, 4.0),
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        },
    ))
    .insert(Ai)
    .insert(Tank)
    .insert(AttackTimer { value: 0.0 } ) 
    .insert(Health { value: 5 } ) 
    .insert(Steps { value: 0.0 } ) 
    .insert(DirectionAi { value: 0 } ) 
    .insert(Velocity { value: Vec2::new(2.0, 0.0) } )
    // .insert(Target {value: Vec2::new(0.0, 0.0) } )
    .with_children(|parent| { // Add turret to player
        parent.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 0., 0.),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(16.0, 16.0, 0.),
                translation: Vec3::new(TANK_SIZE+4.0, 0.0, -1.0),
                ..default()
            },
            ..default()
        }).insert(Turret);
    });
}

fn movement(keyboard_input: Res<Input<KeyCode>>,
    mut positions: Query<(&mut Transform,
    &mut Velocity),
    With<Player>>,
) {
    for (mut transform, mut velocity) in positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            velocity.value.x -= TANK_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            velocity.value.x += TANK_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            velocity.value.y -= TANK_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            velocity.value.y += TANK_SPEED;
        }

        velocity.value *= 0.9;

        transform.translation += velocity.value.extend(0.0);
    }
}

fn collision(mut tanks: Query<(&mut Transform, &mut Velocity, &mut DirectionAi), With<Tank>>, mut windows: ResMut<Windows>,) {
    let window = windows.get_primary_mut().unwrap();
    for (mut tank, mut velocity, mut direction) in tanks.iter_mut() {
        // if tank.translation.x > window.width() - window.width()/2.0 || tank.translation.x < 0.0  - window.width()/2.0 {
        //     velocity.value.x = 0.0;
        // }
        // if tank.translation.y > window.height() - window.height()/2.0 || tank.translation.y < 0.0 - window.height()/2.0 {
        //     velocity.value.y = 0.0;
        // }
        // tank.translation.x = tank.translation.x.min(window.width() - window.width()/2.0).max(0.0 - window.width()/2.0);
        // tank.translation.y = tank.translation.y.min(window.height() - window.height()/2.0).max(0.0 - window.height()/2.0);
        if tank.translation.x > window.width() - window.width()/2.0 {
            velocity.value.x = 0.0;
            tank.translation.x = window.width()/2.0;
            direction.value = 0;
        }
        if tank.translation.x < -window.width()/2.0 {
            velocity.value.x = 0.0;
            tank.translation.x = -window.width()/2.0;
            direction.value = 1;
        }
        if tank.translation.y > window.height() - window.height()/2.0 {
            velocity.value.y = 0.0;
            tank.translation.y = window.height()/2.0;
            direction.value = 2;
        }
        if tank.translation.y < -window.height()/2.0 {
            velocity.value.y = 0.0;
            tank.translation.y = -window.height()/2.0;
            direction.value = 3;
        }
    }
}

fn ai_movement(
    time: Res<Time>,
    mut positions: Query<(&mut Transform, &mut Velocity, &mut Steps, &mut DirectionAi), With<Ai>>,
) {
    for (mut transform, mut velocity, mut steps, mut direction) in positions.iter_mut() {
        if steps.value < 0.0 {
            direction.value = rand::thread_rng().gen_range(0, 4) as u8;
            steps.value = rand::thread_rng().gen_range(1, 110) as f32 / 110.0;
        }
        if direction.value == 0  {
            velocity.value.x -= TANK_SPEED;
        }
        if direction.value == 1  {
            velocity.value.x += TANK_SPEED;
        }
        if direction.value == 2  {
            velocity.value.y -= TANK_SPEED;
        }
        if direction.value == 3  {
            velocity.value.y += TANK_SPEED;
        }

        velocity.value *= 0.9;

        transform.translation += velocity.value.extend(0.0);

        steps.value -= time.delta_seconds();
    }
}

fn quit_and_resize(keyboard_input: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();

    if env::consts::OS == "macos" {
        if keyboard_input.pressed(KeyCode::LWin) && keyboard_input.just_pressed(KeyCode::W) {
            exit.send(AppExit);
            window.set_mode(WindowMode::Windowed);
        }
        if keyboard_input.pressed(KeyCode::LWin) 
        && keyboard_input.pressed(KeyCode::LControl) 
        && keyboard_input.just_pressed(KeyCode::F) {
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


fn mouse_button_input( // Shoot bullets and rotate turret to point at mouse
    buttons: Res<Input<MouseButton>>, 
    windows: Res<Windows>, 
    time: Res<Time>,
    audio: Res<Audio>,
    gunshot: Res<GunshotSound>,
    gunshot_deep: Res<GunshotDeepSound>,
    mut commands: Commands,
    mut positions: Query<(&mut Transform, &mut AttackTimer), With<Player>>,
    mut bearing: Query<(&mut Transform, &Children), (With<Bearing>, Without<Player>, Without<Turret>)>,
    mut transform_query: Query<&mut Transform, (With<Turret>, Without<Player>)>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(_position) = window.cursor_position() {
        match Some(_position) {
            Some(vec) => {
                for (player, mut attack_timer) in positions.iter_mut() {
                    let window_size = Vec2::new(window.width(), window.height());
                    // let diff = Vec3::new(vec.x - window.width()/2.0, vec.y - window.height()/2.0, 0.) - player.translation;
                    let diff = vec.extend(0.0) - window_size.extend(0.0)/2.0 - player.translation;
                    let angle = diff.y.atan2(diff.x); // Add/sub FRAC_PI here optionally

                    for (mut joint, turrets) in bearing.iter_mut() {
                        joint.rotation = Quat::from_rotation_z(angle);
                        for turret in turrets.iter() {
                            if let Ok(mut transform) = transform_query.get_mut(*turret) {
                                transform.translation.x += ((TANK_SIZE+4.0)-transform.translation.x)*0.1;
                            }
                        }
                    }
                    if buttons.pressed(MouseButton::Left) && attack_timer.value > 0.4 {
                        attack_timer.value = 0.0;
                        if !MUTE {
                            audio.play_with_settings(gunshot.0.clone(), PlaybackSettings::ONCE.with_volume(0.2));
                            audio.play_with_settings(gunshot_deep.0.clone(), PlaybackSettings::ONCE.with_volume(0.2));
                        }

                        for (_, turrets) in bearing.iter_mut() {
                            for turret in turrets.iter() {
                                if let Ok(mut transform) = transform_query.get_mut(*turret) {
                                    transform.translation.x = TANK_SIZE+4.0 - 10.0;
                                }
                            }
                        }

                        // println!("x{}, y{}", vec.x, vec.y);
                        let shape = shapes::RegularPolygon {
                            sides: 30,
                            feature: shapes::RegularPolygonFeature::Radius(BULLET_SIZE),
                            ..shapes::RegularPolygon::default()
                        };
                        commands.spawn_bundle(GeometryBuilder::build_as(
                            &shape,
                            DrawMode::Fill (
                                FillMode::color(Color::BLACK),
                            ),
                            Transform {
                                translation: Vec3::new(player.translation.x, player.translation.y, 0.0),
                                ..default()
                            },
                        )).insert(Bullet {from: TurretOf::Player} )
                        // .insert(Direction { dir: Vec2::new(vec.x - player.translation.x - window.width()/2.0, vec.y - player.translation.y - window.height()/2.0).normalize() });
                        .insert(Direction{dir:(vec - player.translation.truncate() - window_size/2.0).normalize()});
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
    audio: Res<Audio>,
    gunshot: Res<GunshotSound>,
    gunshot_deep: Res<GunshotDeepSound>,
    players: Query<&Transform, (Without<Ai>, With<Player>)>,
    mut commands: Commands,
    mut positions: Query<(&mut Transform, &mut AttackTimer, &Children), With<Ai>>,
    mut transform_query: Query<&mut Transform, (With<Turret>, Without<Ai>, Without<Player>)>,
) {
    for player in players.iter() {
        for (mut ai, mut attack_timer, children) in positions.iter_mut() {
            // let window_size = Vec2::new(window.width(), window.height());
            let diff = Vec3::new(player.translation.x, player.translation.y, 0.) - ai.translation;
            // let diff = vec.extend(0.0) - window_size.extend(0.0)/2.0 - ai.translation;
            let angle = diff.y.atan2(diff.x); // Add/sub FRAC_PI here optionally
            ai.rotation = Quat::from_rotation_z(angle);

            for child in children.iter() {
                if let Ok(mut transform) = transform_query.get_mut(*child) {
                    transform.translation.x += ((TANK_SIZE+4.0)-transform.translation.x)*0.1;
                }
            }

            if attack_timer.value < 0.0 {
                attack_timer.value =rand::thread_rng().gen_range(5, 14) as f32 /10.0 ;
                if !MUTE {
                    audio.play_with_settings(gunshot.0.clone(), PlaybackSettings::ONCE.with_volume(0.2));
                    audio.play_with_settings(gunshot_deep.0.clone(), PlaybackSettings::ONCE.with_volume(0.2));
                }

                for child in children.iter() {
                    if let Ok(mut transform) = transform_query.get_mut(*child) {
                        transform.translation.x = TANK_SIZE+4.0 - 10.0;
                    }
                }

                // println!("x{}, y{}", player.translation.x, player.translation.y);
                let shape = shapes::RegularPolygon {
                    sides: 30,
                    feature: shapes::RegularPolygonFeature::Radius(BULLET_SIZE),
                    ..shapes::RegularPolygon::default()
                };
                commands.spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    DrawMode::Fill (
                        FillMode::color(Color::BLACK),
                    ),
                    Transform {
                        translation: Vec3::new(ai.translation.x, ai.translation.y, 0.0),
                        ..default()
                    },
                )).insert(Bullet {from: TurretOf::Ai} )
                // .insert(Direction { dir: Vec2::new(vec.x - ai.translation.x - window.width()/2.0, vec.y - ai.translation.y - window.height()/2.0).normalize() });
                .insert(Direction{dir:(player.translation.truncate() - ai.translation.truncate()).normalize()});
            }

            attack_timer.value -= time.delta_seconds()
        }
    }

}

fn hurt_tanks(
    audio: Res<Audio>,
    tank_hit: Res<TankHitSound>,
    tank_hit_deep: Res<TankHitDeepSound>,
    mut commands: Commands,
    bullets: Query<(&Transform, Entity, &Bullet), (Without<Player>, Without<Ai>, With<Bullet>)>,
    mut players: Query<(&mut Transform, Entity, &mut Health, &Children), (With<Player>, Without<Ai>, Without<Bullet>)>,
    mut ais: Query<(&Transform, Entity, &mut Health), (Without<Player>, With<Ai>, Without<Bullet>)>,
    mut transform_query: Query<&mut Transform, (With<Healthbar>, Without<Ai>, Without<Player>, Without<Bullet>)>,
) {
    for (bullet_transform, bullet_entity, bullet_type) in bullets.iter() {
        match bullet_type.from {
            TurretOf::Player => {
                for (ai_transform, ai_entity, mut ai_health) in ais.iter_mut() {
                    if distance_between(&ai_transform.translation.truncate(), &bullet_transform.translation.truncate()) < TANK_SIZE+BULLET_SIZE {
                        if ai_health.value > 0 {
                            ai_health.value -= 1;
                        } else {
                            commands.entity(ai_entity).despawn_recursive(); 
                        }
                        commands.entity(bullet_entity).despawn(); 
                        if !MUTE {
                            audio.play_with_settings(tank_hit.0.clone(), PlaybackSettings::ONCE.with_volume(0.2));
                            audio.play_with_settings(tank_hit_deep.0.clone(), PlaybackSettings::ONCE.with_volume(0.1));
                        }
                    }
                }
            }
            TurretOf::Ai => {
                for (player_transform, player_entity, mut player_health, children) in players.iter_mut() {
                    if distance_between(&player_transform.translation.truncate(), &bullet_transform.translation.truncate()) < TANK_SIZE+BULLET_SIZE {
                        if player_health.value > 0 {
                            player_health.value -= 1;
                                for healthbar in children.iter() {
                                    if let Ok(mut transform) = transform_query.get_mut(*healthbar) {
                                        transform.scale.x = player_health.value as f32 / MAX_HEALTH as f32 * HEALTHBAR_WIDTH ;
                                    }
                                }
                        } else {
                            commands.entity(player_entity).despawn_recursive(); 
                        }
                        commands.entity(bullet_entity).despawn(); 
                        if !MUTE {
                            audio.play_with_settings(tank_hit.0.clone(), PlaybackSettings::ONCE.with_volume(0.2));
                            audio.play_with_settings(tank_hit_deep.0.clone(), PlaybackSettings::ONCE.with_volume(0.1));
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
    audio: Res<Audio>,
    wall_hit: Res<WallHitSound>,
    wall_hit_deep: Res<WallHitDeepSound>,
    mut commands: Commands,
    mut bullets: Query<((&mut Transform, Entity), With<Bullet>)>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    for ((transform, bullet_entity), _bullet) in bullets.iter_mut() {
        if transform.translation.x.abs() > window.width()/2. || transform.translation.y.abs() > window.height()/2. { 
            commands.entity(bullet_entity).despawn(); 
            if !MUTE {
                audio.play_with_settings(wall_hit.0.clone(), PlaybackSettings::ONCE.with_volume(0.2));
                audio.play_with_settings(wall_hit_deep.0.clone(), PlaybackSettings::ONCE.with_volume(0.2));
            }
        }
    }
}

fn distance_between(point1: &Vec2, point2: &Vec2) -> f32 {
    let diff = *point1 - *point2; // (Your assumption *was* correct btw, but this works)
    // (diff.x.powf(2.0) + diff.y.powf(2.0)).sqrt()
    diff.length()
}
