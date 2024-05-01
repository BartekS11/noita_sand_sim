use bevy::{prelude::*, window::PrimaryWindow};

use crate::sand_sim_constants::{VELOCITY_DEFAULT};

// const GRID_SIZE: Vec2 = Vec2::new(GRID_WIDTH / SCALE, GRID_WIDTH / SCALE);
const BACKGROUND_DEFAULT_COLOR: Color = Color::DARK_GRAY;

pub struct SandsimPlugin;

#[derive(Component)]
struct Sandbox {
    grid: Vec<FieldState>,
    particles: Vec<ParticleEntity>,
}

impl Sandbox {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![FieldState::Empty; width * height],
            particles: vec![],
        }
    }
}

#[derive(Component)]
struct ParticleEntity {
    pos: Vec2,
    updated: bool,
    particle_type: FieldState,
    color: Color,
    velocity: f32,
}

impl ParticleEntity {
    pub fn new(pos: Vec2, particle_type: FieldState) -> Self {
        Self {
            pos,
            updated: false,
            particle_type,
            color: set_color(particle_type),
            velocity: VELOCITY_DEFAULT,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FieldState {
    Sand(usize),
    Water(usize),
    Wood(usize),
    Empty,
}

fn set_color(particle_type: FieldState) -> Color {
    match particle_type {
        FieldState::Sand(_) => Color::GOLD,
        FieldState::Water(_) => Color::BLUE,
        FieldState::Wood(_) => Color::BEIGE,
        _ => BACKGROUND_DEFAULT_COLOR,
    }
}

pub fn spawn_sandbox(mut commands: Commands, width: usize, height: usize) {
    commands.spawn(Sandbox::new(width, height));
}

fn mouse_button_input(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = q_window.single();
    if let Some(world_position) = window.cursor_position() {
        if buttons.just_pressed(MouseButton::Left) {
            println!("mouse pos X: {}, pos Y: {} ", world_position.x, world_position.y);
            commands.spawn(ParticleEntity::new(world_position, FieldState::Sand(1)));
        }
    }
}

impl Plugin for SandsimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_button_input);
    }
}
