use bevy::input::common_conditions::{input_just_pressed, input_just_released, input_pressed};
use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::window::PrimaryWindow;
use bevy::{
    prelude::*,
    window::PresentMode,
    // winit::WinitSettings
};

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        // .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1920.0, 1080.0).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_system)
        .add_systems(Update, (zoom, arrow_key_pan))
        .add_systems(
            Update,
            (
                start_drag.run_if(input_just_pressed(MouseButton::Left)),
                drag.run_if(input_pressed(MouseButton::Left)),
                end_drag.run_if(input_just_released(MouseButton::Left)),
            )
                .chain(),
        )
        .run();
}

/// The current drag operation including the offset with which we grabbed the Bevy logo.
#[derive(Resource)]
struct LastPos(Vec2);

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d::default(),
        Projection::from(OrthographicProjection::default_2d()),
    ));
    let shape = meshes.add(Rectangle::new(300.0, 300.0));
    let color = Color::Srgba(Srgba::BLUE);
    commands.spawn((Mesh2d(shape), MeshMaterial2d(materials.add(color))));
}

fn arrow_key_pan(
    mut cam_t: Single<&mut Transform, With<Camera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for key in keyboard.get_pressed() {
        match key {
            KeyCode::ArrowLeft => cam_t.translation.x -= 20.0,
            KeyCode::ArrowRight => cam_t.translation.x += 20.0,
            KeyCode::ArrowUp => cam_t.translation.y += 20.0,
            KeyCode::ArrowDown => cam_t.translation.y -= 20.0,
            _ => return,
        }
    }
}

fn zoom(
    camera: Single<&mut OrthographicProjection, With<Camera>>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
) {
    // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
    let delta_zoom = -mouse_wheel_input.delta.y * 0.02;
    // When changing scales, logarithmic changes are more intuitive.
    // To get this effect, we add 1 to the delta, so that a delta of 0
    // results in no multiplicative effect, positive values result in a multiplicative increase,
    // and negative values result in multiplicative decreases.
    let multiplicative_zoom = 1. + delta_zoom;

    let mut camera = camera.into_inner();

    camera.scale = camera.scale * multiplicative_zoom;

    println!("scale: {}", camera.scale);
}

fn start_drag(mut commands: Commands, primary_window: Single<&Window, With<PrimaryWindow>>) {
    let Some(cursor_pos) = primary_window.cursor_position() else {
        return;
    };
    println!("setting");
    commands.insert_resource(LastPos(cursor_pos));
}

/// Stop the current drag operation
fn end_drag(mut commands: Commands) {
    println!("removing");
    commands.remove_resource::<LastPos>();
}

/// Drag the Bevy logo
fn drag(
    mut last_pos: ResMut<LastPos>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
    cam_q: Single<(&mut Transform, &OrthographicProjection)>,
) {
    // If the cursor is not within the primary window skip this system
    let Some(cursor_pos) = primary_window.cursor_position() else {
        return;
    };

    let (mut cam_t, proj) = cam_q.into_inner();

    let delta = cursor_pos - last_pos.0;

    // Get the cursor position in the world
    let world_space_delta = (delta * proj.scale).extend(cam_t.translation.z);

    let old_translation = cam_t.translation.truncate();

    // Update the translation of Bevy logo transform to new translation
    cam_t.translation.x -= world_space_delta.x;
    cam_t.translation.y += world_space_delta.y;

    let new_translation = cam_t.translation.truncate();

    println!("world_space_delta {world_space_delta} delta {delta} old_translation {old_translation} new_translation {new_translation} last_pos {} cursor_pos {cursor_pos}", last_pos.0);

    last_pos.0 = cursor_pos;
}
