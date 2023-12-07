use bevy::prelude::*;
use crate::utils::Health;

pub const HEALTHBAR_Y_OFFSET: f32 = 40.0;

pub const HEALTHBAR_WIDTH: f32 = 60.0;
pub const HEALTHBAR_BORDER_HEIGHT: f32 = 15.0;

pub const HEALTHBAR_HEIGHT: f32 = HEALTHBAR_BORDER_HEIGHT/2.0;
pub const HEALTHBAR_BORDER_THICKNESS: f32 = HEALTHBAR_BORDER_HEIGHT/4.0; // The width of the "outline" around the inner border

#[derive(Component)]
pub struct Healthbar;

#[derive(Component)]
pub struct HealthbarBorder;

#[derive(Component)]
pub struct MaxHealth(pub u8);

// A system to automatically find parents of the healthbar, read their health and update the healthbar
pub fn update_healthbar(
	healthbar_parents: Query<&Health>,
	mut healthbars: Query<(&mut Transform, &mut Sprite, &MaxHealth, &Parent), With<Healthbar>>
) {
	for (mut transform, mut sprite, max_health, parent) in healthbars.iter_mut() {
		// We have the healthbar's components
		if let Ok(health) = healthbar_parents.get(parent.get()) {
			// We have the health of the healthbar's parent
			let health_percentage = health.value as f32 / max_health.0 as f32;
			let inner_healthbar_width = health_percentage * HEALTHBAR_WIDTH;

			// Set the width to the calculated width
			transform.scale.x = inner_healthbar_width;
			// Transforms in Bevy are centred. We find the left side of the healthbar, 
			// and add half of the healthbar's width to find its centre.
			transform.translation.x = -HEALTHBAR_WIDTH/2.0 + inner_healthbar_width/2.0;
			// Set the colour. 150 is the hue for green.
			sprite.color = Color::hsl(health_percentage * 150.0, 0.98, 0.58);
		}
    }
}

// Update the border backgrounds's colour
pub fn update_healthbar_border(
	healthbar_parents: Query<(&Health, &Children)>,
	mut healthbar_borders: Query<(&mut Sprite, &Parent), With<HealthbarBorder>>,
    healthbars: Query<&MaxHealth, With<Healthbar>>,
) {
	for (mut sprite, parent) in healthbar_borders.iter_mut() {
		// We have the healthbar's components
		if let Ok((health, children)) = healthbar_parents.get(parent.get()) {
            // We got the parent's health and children
            // In order to calculate the border background's colour, we must know the maximum health.
            // Unfortunately, only the healthbar entity has this component. We must search through the 
            // border's parent's children to find the healthbar entity and its max-health! Yes, this is probably stupid.
            for &child in children.iter() {
                if let Ok(max_health) = healthbars.get(child) {
                    // We have the max health!
                    let health_percentage = health.value as f32 / max_health.0 as f32;
                    // Set the colour. 150 is the hue for green.
                    sprite.color = Color::hsl(health_percentage * 150.0, 0.73, 0.48);
                }
            }
		}
    }
}

#[derive(Bundle)]
pub struct HealthbarBundle {
    healthbar: Healthbar,
    sprite_bundle: SpriteBundle, // Sprite Bundle gives the healthbar its "rectangle"
	max_health: MaxHealth,
}

impl HealthbarBundle {
    // This creates a new default healthbar
    // Usefull because we can just do commands.spawn(HealthbarBundle.new(5))
    pub fn new(max_health: u8) -> HealthbarBundle {
        HealthbarBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::hsl(150.0, 0.98, 0.58),
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(HEALTHBAR_WIDTH, HEALTHBAR_HEIGHT, 0.),
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
                    scale: Vec3::new(
						HEALTHBAR_WIDTH  + HEALTHBAR_BORDER_THICKNESS * 2.0, 
						HEALTHBAR_HEIGHT + HEALTHBAR_BORDER_THICKNESS * 2.0, 
						0.0,
					),
                    translation: Vec3::new(0.0, HEALTHBAR_Y_OFFSET, 0.5),
                    ..default()
                },
                ..default()
            },
            healthbar_border: HealthbarBorder,
        }
    }
}