use bevy::prelude::*;

pub const HEALTHBAR_WIDTH: f32 = 50.0;
pub const HEALTHBAR_Y_OFFSET: f32 = 40.0;

#[derive(Component)]
pub struct Healthbar;

#[derive(Component)]
pub struct HealthbarBorder;

#[derive(Component)]
pub struct MaxHealth(pub u8);

#[derive(Bundle)]
pub struct HealthbarBundle {
    sprite_bundle: SpriteBundle, // Sprite Bundle gives the healthbar its "rectangle"
    healthbar: Healthbar,
	max_health: MaxHealth,
}

impl HealthbarBundle {
    // This creates a new default healthbar
    // Usefull because we can just do commands.spawn(HealthbarBundle.new())
    pub fn new(max_health: u8) -> HealthbarBundle {
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
			max_health: MaxHealth(max_health),
        }
    }
}

// For the background/border of the healthbar
#[derive(Bundle)]
pub struct HealthbarBorderBundle {
    sprite_bundle: SpriteBundle,
    healthbar_border: HealthbarBorder,
}

impl HealthbarBorderBundle {
    // Creates a default healthbar border
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