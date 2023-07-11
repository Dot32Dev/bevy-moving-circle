use bevy::prelude::*;
// use bevy_prototype_lyon::prelude::*; // Draw circles with ease
// use bevy_inspector_egui::Inspectable;

pub const TANK_SPEED: f32 = 0.37;
pub const TANK_SIZE: f32 = 20.0; 

pub const HEALTHBAR_WIDTH: f32 = 50.0;
pub const MAX_HEALTH: u8 = 5;
pub const HEALTHBAR_Y_OFFSET: f32 = 40.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ai;

#[derive(Component)]
pub struct Tank;

#[derive(Component)]
pub struct Velocity {
    pub value: Vec2,
}

#[derive(Component)]
pub struct Active {
    pub value: bool,
}
#[derive(Component)]
pub struct AttackTimer {
    pub value: f32,
}

// #[derive(Inspectable, Component)]
#[derive(Component)]
pub struct Health {
    pub value: u8,
}

#[derive(Component)]
pub struct Steps {
    pub value: f32,
}

#[derive(Component)]
pub struct DirectionAi {
    pub value: u8,
}

#[derive(Component)]
pub struct Turret;

#[derive(Component)]
pub struct Bearing;

#[derive(Component)]
pub struct Healthbar;

#[derive(Component)]
pub struct HealthbarBorder;

#[derive(Bundle)]
pub struct TankBundle<M: bevy::sprite::Material2d> {
    // #[bundle]
    material_bundle: bevy::sprite::MaterialMesh2dBundle<M>,
    tank: Tank,
    attack_timer: AttackTimer,
    health: Health,
    velocity: Velocity,
}

#[derive(Bundle)]
pub struct AiBundle {
    ai: Ai,
    active: Active,
    steps: Steps,
    direction_ai: DirectionAi,
}

#[derive(Bundle)]
pub struct HealthbarBundle {
    // #[bundle]
    sprite_bundle: SpriteBundle,
    healthbar: Healthbar,
}

impl HealthbarBundle {
    pub fn new() -> HealthbarBundle {
        HealthbarBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::hsl(150.0, 0.98, 0.58),
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(HEALTHBAR_WIDTH, 10.0, 0.),
                    translation: Vec3::new(0.0, HEALTHBAR_Y_OFFSET, 1.0),
                    ..default()
                },
                ..default()
            },
            healthbar: Healthbar,
        }
    }
}

#[derive(Bundle)]
pub struct HealthbarBorderBundle {
    // #[bundle]
    sprite_bundle: SpriteBundle,
    healthbar_border: HealthbarBorder,
}

impl HealthbarBorderBundle {
    pub fn new() -> HealthbarBorderBundle {
        HealthbarBorderBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0., 0., 0., 0.5),
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(HEALTHBAR_WIDTH+8.0, 18.0, 0.),
                    translation: Vec3::new(0.0, HEALTHBAR_Y_OFFSET, 0.5),
                    ..default()
                },
                ..default()
            },
            healthbar_border: HealthbarBorder,
        }
    }
}

#[derive(Bundle)]
pub struct BearingBundle {
    // #[bundle]
    sprite_bundle: SpriteBundle,
    bearing: Bearing,
}

impl BearingBundle {
    pub fn new() -> BearingBundle {
        BearingBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    translation: Vec3::new(0.0, 0.0, 0.),
                    ..default()
                },
                ..default()
            },
            bearing: Bearing,
        }
    }
}

#[derive(Bundle)]
pub struct TurretBundle {
    // #[bundle]
    sprite_bundle: SpriteBundle,
    turret: Turret,
}

impl TurretBundle {
    pub fn new() -> TurretBundle {
        TurretBundle {
            sprite_bundle: SpriteBundle {
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
            },
            turret: Turret,
        }
    }
}

impl TankBundle<ColorMaterial> {
    pub fn new(
        colour: Color,

        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> TankBundle<ColorMaterial> {
        // let shape = shapes::RegularPolygon { // Define circle
        //     sides: 30,
        //     feature: shapes::RegularPolygonFeature::Radius(TANK_SIZE),
        //     ..shapes::RegularPolygon::default()
        // };

        // commands.spawn((
        //     MaterialMesh2dBundle {
        //         mesh: meshes.add(shape::Circle::new(BULLET_SIZE).into()).into(),
        //         material: materials.add(ColorMaterial::from(Color::PURPLE)),
        //         transform: Transform::from_translation(Vec3::new(ai.translation.x, ai.translation.y, 0.0)),
        //         ..default()
        //     },
        //     Name::new("Bullet"),
        //     Bullet {from: TurretOf::Player},
        //     Direction{dir:(player.translation.truncate() - ai.translation.truncate()).normalize()},
        // ));

        TankBundle {
            // geometry_builder: GeometryBuilder::build_as(
            //     &shape,
            //     DrawMode::Outlined {
            //         fill_mode: FillMode::color(colour),
            //         outline_mode: StrokeMode::new(Color::BLACK, 4.0),
            //     },
            //     Transform {
            //         translation: Vec3::new(0.0, 0.0, 1.0),
            //         ..default()
            //     },
            // ),
            material_bundle: bevy::sprite::MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(TANK_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(colour)),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..default()
                },
                ..default()
            },
            tank: Tank,
            attack_timer: AttackTimer {
                value: 0.0,
            },
            health: Health {
                value: MAX_HEALTH,
            },
            velocity: Velocity {
                value: Vec2::new(0.0, 0.0),
            },
        }
    }
}

impl AiBundle {
    pub fn new() -> AiBundle {
        AiBundle {
            active: Active {
                value: true,
            },
            steps: Steps {
                value: 0.0,
            },
            direction_ai: DirectionAi {
                value: 0,
            },
            ai: Ai,
        }
    }
}