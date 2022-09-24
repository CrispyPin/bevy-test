//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Component)]
struct MouseRotation;

#[derive(Component)]
struct FlightMovement {
	speed: f32,
	modifier: f32,
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system(grab_mouse)
		.add_system(mouse_rot)
		.add_system(flight_mov)
		.run();
}

fn mouse_rot(
	mut query: Query<&mut Transform, With<MouseRotation>>,
	// time: Res<Time>,
	mut mouse: EventReader<MouseMotion>,
) {
	let mut delta = Vec2::new(0., 0.);
	for event in mouse.iter() {
		delta += event.delta;
	}
	for mut transform in query.iter_mut() {
		transform.rotate_y(delta.x * -0.002);

		transform.rotate_local_x(delta.y * -0.002);
	}
}

fn flight_mov(
	mut query: Query<(&mut Transform, &FlightMovement)>,
	kb: Res<Input<KeyCode>>,
	time: Res<Time>,
) {
	for (mut transform, flight) in query.iter_mut() {
		let mut dir = Vec3::ZERO;
		if kb.pressed(KeyCode::W) {
			dir += transform.forward();
		}
		if kb.pressed(KeyCode::S) {
			dir += transform.back();
		}
		if kb.pressed(KeyCode::A) {
			dir += transform.left();
		}
		if kb.pressed(KeyCode::D) {
			dir += transform.right();
		}

		if kb.pressed(KeyCode::LShift) {
			dir *= flight.modifier;
		}
		transform.translation += dir * time.delta_seconds() * flight.speed;
	}
}

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key is pressed
fn grab_mouse(
	mut windows: ResMut<Windows>,
	mouse: Res<Input<MouseButton>>,
	key: Res<Input<KeyCode>>,
) {
	let window = windows.get_primary_mut().unwrap();
	if mouse.just_pressed(MouseButton::Left) {
		window.set_cursor_visibility(false);
		window.set_cursor_lock_mode(true);
	}
	if key.just_pressed(KeyCode::Escape) {
		window.set_cursor_visibility(true);
		window.set_cursor_lock_mode(false);
	}
}

/// set up a simple 3D scene
fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// plane
	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
		material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
		..default()
	});
	// cube
	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
		material: materials.add(StandardMaterial {
			base_color: Color::hex("bbffff").unwrap(),
			metallic: 0.0,
			perceptual_roughness: 0.2,
			base_color_texture: Some(asset_server.load("imag.png")),
			flip_normal_map_y: true,
			..default()
		}),
		transform: Transform::from_xyz(0.0, 0.5, 0.0),
		..default()
	});
	// light
	commands.spawn_bundle(PointLightBundle {
		point_light: PointLight {
			intensity: 1500.0,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..default()
	});
	// camera
	commands
		.spawn_bundle(Camera3dBundle {
			transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		})
		.insert(MouseRotation)
		.insert(FlightMovement {
			speed: 4.0,
			modifier: 2.0,
		});
}
