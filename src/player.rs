use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .add_startup_system(setup_player)
            .add_startup_system(initial_grab_cursor)
            .add_system(player_move)
            .add_system(player_look)
            .add_system(cursor_grab);
    }
}

#[derive(Default)]
struct InputState {
    pitch: f32,
    yaw: f32,
}

fn setup_player(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Player);
}

fn toggle_grab_cursor(window: &mut Window) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}

fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    toggle_grab_cursor(windows.get_primary_mut().unwrap());
}

#[derive(Component)]
pub struct Player;

fn player_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut();
    let mut direction = Vec3::ZERO;

    let mut x = transform.local_x();
    let mut z = transform.local_z();
    x.y = 0.0;
    z.y = 0.0;

    if keyboard_input.pressed(KeyCode::A) {
        direction -= x;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction += x;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction += z;
    }
    if keyboard_input.pressed(KeyCode::W) {
        direction -= z;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        direction += Vec3::Y;
    }
    if keyboard_input.pressed(KeyCode::LShift) {
        direction -= Vec3::Y;
    }

    transform.translation += direction.normalize_or_zero() * 0.1;
}

fn player_look(
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let window = windows.get_primary().unwrap();
    for mut transform in query.iter_mut() {
        for ev in mouse_motion_events.iter() {
            if window.cursor_locked() {
                let window_scale = window.height().min(window.width());

                state.pitch -= (0.00012 * ev.delta.y * window_scale).to_radians();
                state.yaw -= (0.00012 * ev.delta.x * window_scale).to_radians();
            }

            state.pitch = state.pitch.clamp(-1.54, 1.54);

            transform.rotation = Quat::from_axis_angle(Vec3::Y, state.yaw)
                * Quat::from_axis_angle(Vec3::X, state.pitch);
        }
    }
}

fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if keys.just_pressed(KeyCode::Escape) {
        toggle_grab_cursor(window);
    }
}
