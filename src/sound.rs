use bevy::prelude::*;

#[derive(Component)]
pub struct GunShotSound;
#[derive(Component)]
pub struct GunShotDeepSound;

#[derive(Component)]
pub struct TankHitSound;
#[derive(Component)]
pub struct TankHitDeepSound;

#[derive(Component)]
pub struct WallHitSound;
#[derive(Component)]
pub struct WallHitDeepSound;

pub fn play_gunshot(
	// Used to spawn new sounds
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	// Used to delete previous sounds
	mid: Query<(Entity, With<GunShotSound>)>,
	deep: Query<(Entity, With<GunShotDeepSound>)>,
) {
	// Even though sounds automatically get despawned after their completion, multiple of the same sound playing
	// at the same time can corrupt the sound. We therefore delete any previously played (but unfinished) sounds.
	for (entity, _sound) in mid.iter() {
		commands.entity(entity).despawn();
	}
	for (entity, _sound) in deep.iter() {
		commands.entity(entity).despawn();
	}

	// Here, we spawn the new sound
	commands.spawn((
		AudioBundle {
			source: asset_server.load("ShotsFired.ogg"),
			settings: PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::new_relative(0.5))
		},
		GunShotSound,
	));
	commands.spawn((
		AudioBundle {
			source: asset_server.load("ShotsFiredDeep.ogg"),
			settings: PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::new_relative(0.25))
		},
		GunShotDeepSound,
	));
}

pub fn play_tankhit(
	// Used to spawn new sounds
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	// Used to delete previous sounds
	mid: Query<(Entity, With<TankHitSound>)>,
	deep: Query<(Entity, With<TankHitDeepSound>)>,
) {
	// Even though sounds automatically get despawned after their completion, multiple of the same sound playing
	// at the same time can corrupt the sound. We therefore delete any previously played (but unfinished) sounds.
	for (entity, _sound) in mid.iter() {
		commands.entity(entity).despawn();
	}
	for (entity, _sound) in deep.iter() {
		commands.entity(entity).despawn();
	}

	// Here, we spawn the new sound
	commands.spawn((
		AudioBundle {
			source: asset_server.load("TankHit.ogg"),
			settings: PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::new_relative(0.5))
		},
		TankHitSound,
	));
	commands.spawn((
		AudioBundle {
			source: asset_server.load("TankHitDeep.ogg"),
			settings: PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::new_relative(0.25))
		},
		TankHitDeepSound,
	));
}

pub fn play_wallhit(
	// Used to spawn new sounds
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	// Used to delete previous sounds
	mid: Query<(Entity, With<WallHitSound>)>,
	deep: Query<(Entity, With<WallHitDeepSound>)>,
) {
	// Even though sounds automatically get despawned after their completion, multiple of the same sound playing
	// at the same time can corrupt the sound. We therefore delete any previously played (but unfinished) sounds.
	for (entity, _sound) in mid.iter() {
		commands.entity(entity).despawn();
	}
	for (entity, _sound) in deep.iter() {
		commands.entity(entity).despawn();
	}

	// Here, we spawn the new sound
	commands.spawn((
		AudioBundle {
			source: asset_server.load("WallHit.ogg"),
			settings: PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::new_relative(0.25))
		},
		WallHitSound,
	));
	commands.spawn((
		AudioBundle {
			source: asset_server.load("WallHitDeep.ogg"),
			settings: PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::new_relative(0.125))
		},
		WallHitDeepSound,
	));
}