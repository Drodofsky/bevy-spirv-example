//! A shader and a material that uses it.

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::InnerMeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey, Material2dPlugin, MaterialMesh2dBundle},
    utils::Hashed,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update_material)
        .run();
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // timer
    commands.insert_resource(ColorChangeTimer(Timer::from_seconds(
        1.0,
        TimerMode::Repeating,
    )));

    // camera
    commands.spawn(Camera2dBundle::default());

    // quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(CustomMaterial {
            color: Color::GREEN,
        }),
        ..default()
    });
}

fn update_material(
    mut material: ResMut<Assets<CustomMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<ColorChangeTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for m in material.iter_mut() {
            if m.1.color == Color::GREEN {
                m.1.color = Color::RED
            } else {
                m.1.color = Color::GREEN
            }
        }
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    pub color: Color,
}
#[derive(Resource)]
struct ColorChangeTimer(Timer);

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/shader.spv".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &Hashed<InnerMeshVertexBufferLayout>,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main_fs".into();
        Ok(())
    }
}
