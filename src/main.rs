//! A shader and a material that uses it.

use bevy::{
    core_pipeline::prepass::NormalPrepass,
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, convert_materials)
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("scenes/fox.glb#Scene0"),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // DepthPrepass,
        NormalPrepass,
    ));
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

impl Default for CustomMaterial {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            alpha_mode: AlphaMode::Opaque,
        }
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    alpha_mode: AlphaMode,
}

fn convert_materials(
    mut cmds: Commands,
    mut handles: Query<(Entity, With<Handle<StandardMaterial>>)>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
    for (entity, _) in handles.iter_mut() {
        let custom_material = custom_materials.add(CustomMaterial::default());
        cmds.entity(entity)
            .insert(custom_material)
            .remove::<Handle<StandardMaterial>>();
    }
}
