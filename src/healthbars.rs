use bevy::prelude::*;
use bevy::window::*;
use bevy::sprite::Mesh2dHandle;
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

pub enum Side {
	Right,
    Left
}

#[derive(Component)]
pub struct HealthbarSide(pub Side);

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
			// The keep_healthbars_on_screen system handles the healthbar's translation, so we don't need to update it here
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

// Update the side-circles of the healthbar/healthbarborder
pub fn update_healthbar_sides(
    mut materials: ResMut<Assets<ColorMaterial>>,
	bars: Query<(&Transform, &Sprite), Without<HealthbarSide>>,
	mut sides: Query<(&mut Transform, &Handle<ColorMaterial>, &Parent, &HealthbarSide)>
) {
    for (mut transform, material_handle, parent, side) in sides.iter_mut() {
		// We have the side circle's components
		if let Ok((parent_transform, parent_sprite)) = bars.get(parent.get()) {
			// We now have the transform and sprite of the side circle's parent
            // We make the circle just as wide as it is high
            transform.scale.x = parent_transform.scale.y / parent_transform.scale.x;
            // We move the circle to the left or the right of its parent
            match side.0 {
                Side::Right => transform.translation.x = 0.5,
                Side::Left => transform.translation.x = -0.5,
            }
            // We update the circle's colour
            let material = materials.get_mut(material_handle.id()).unwrap();
            material.color = parent_sprite.color;
		}
    }
}

// This system handles the relative location of the healthbar to the player, such as ensuring the
// healthbar doesn't go off-screen, and ensuring the inner healthbar stays left-aligned.
pub fn keep_healthbars_on_screen(
    mut healthbar: Query<(&mut Transform, &GlobalTransform), (With<Healthbar>, Without<HealthbarBorder>)>,
    mut healthbar_border: Query<(&mut Transform, &GlobalTransform), (With<HealthbarBorder>, Without<Healthbar>)>,
    primary_window: Query<&Window, With<PrimaryWindow>>
) {
    let Ok(window) = primary_window.get_single() else {
        return;

    };
    // Rather than using the constant HEALTHBAR_BORDER_HEIGHT, from which these constants are
    // calculated from, we use these constants themselves. This means they could changed independently
    // of the border height constant, and this system would continue to function. This height is
    // calculated as being the height of the healthbars border, and is used for both the border
    // and the inner health bar's calculations, as we want the inner healthbar to stop moving when the
    // border stops moving as well. 
    let healthbar_height = HEALTHBAR_HEIGHT + HEALTHBAR_BORDER_THICKNESS*2.0;
    // The point at which the healthbars must update their transform is given by half of the screen
    // height and subtracting half of the healthbar's height. Transforms in Bevy are centred.
    let ceiling = window.height()/2.0 -healthbar_height/2.0;
    // Now we calculate the same for the right edge. The left edge can be inferred by taking the
    // negative of the right edge.
    let total_healthbar_width = HEALTHBAR_WIDTH + healthbar_height;
    let right_edge = window.width()/2.0 - total_healthbar_width/2.0;

    // Loop over all the inner healthbars
    for (mut transform, global_transform) in healthbar.iter_mut() {
        // Calculate the healthbar parents's Y position by subtracting the relative position from the global position
        let parent_y = global_transform.translation().y - transform.translation.y;
        // `(ceiling - parent_y)` calculates the maximum height the healthbar could go to, relative to the parent.
        // The min function returns the smallest of its two inputs. If the parent + HEALTHBAR_Y_OFFSET is less than the
        // top of the screen, the healthbar will go there, otherwise the healthbar will go to the top of the screen. This
        // essentially snaps the healthbar to the top of the screen whenever the parent goes too high. 
        transform.translation.y = (HEALTHBAR_Y_OFFSET).min(ceiling - parent_y);
        
        // As the healthbar loses health, the "inner healthbar" shrinks in width. If we want it to remain left-aligned
        // rather than centred, we have to apply an offset to its X position.
        let x_offset = (HEALTHBAR_WIDTH - transform.scale.x)/2.0;
        // Here, something similar is done, except with the X. `right_edge-parent_x` calculates the distance from the 
        // parent to the right edge, and `-right_edge-parent_x` calculates the difference to the left edge. (The left 
        // edge is the opposite (negative) of the right edge). Normally, we want the X offset from the parent to be 0,
        // but if the distance to the right edge os negative, then we would rather be there. If the distance to the 
        // left egde is positive, then we would rather be there. Finally we subtract the x_offset to position the bar
        // appropriately. The reason that the x_offset isn't placed within the brackets to be .min()'ed and .max()'ed
        // is that we want this inner bar to stay put relative to the healthbar border. We shift it as if it were the
        // same as the border.
        let parent_x = global_transform.translation().x - transform.translation.x;
        transform.translation.x = (0.0_f32).min(right_edge-parent_x).max(-right_edge-parent_x) - x_offset;
    }
    // Loop over all the healthbar borders
    for (mut transform, global_transform) in healthbar_border.iter_mut() {
        // This is exactly the same as with the inner healthbar
        let parent_y = global_transform.translation().y - transform.translation.y;
        transform.translation.y = (HEALTHBAR_Y_OFFSET).min(ceiling - parent_y);

        // Similar to the inner healthbar, but without an X offset
        let parent_x = global_transform.translation().x - transform.translation.x;
        transform.translation.x = (0.0_f32).min(right_edge-parent_x).max(-right_edge-parent_x);
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
                    color: Color::hsl(150.0, 0.73, 0.48),
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(
						HEALTHBAR_WIDTH, 
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

#[derive(Bundle)]
pub struct HealthbarSideBundle<M: bevy::sprite::Material2d> {
    material_bundle: bevy::sprite::MaterialMesh2dBundle<M>,
    healthbar_side: HealthbarSide,
}

impl HealthbarSideBundle<ColorMaterial> {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        side: Side,
    ) -> HealthbarSideBundle<ColorMaterial> {
        HealthbarSideBundle {
            material_bundle: bevy::sprite::MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle { radius: 0.5 })),
                material: materials.add(Color::NONE),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.1),
                    ..default()
                },
                ..default()
            },
            healthbar_side: HealthbarSide(side),
        }
    }
}