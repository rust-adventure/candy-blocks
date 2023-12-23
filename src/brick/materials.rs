use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_xpbd_3d::components::{
    CollidingEntities, LinearVelocity,
};

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    // #[texture(1)]
    // #[sampler(2)]
    // color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

/// change the color of cubes depending on the speed at
/// which they fall
pub fn highlight_colliding_cubes(
    query: Query<(
        Entity,
        &CollidingEntities,
        &LinearVelocity,
    )>,
    standard_materials: Query<
        &mut Handle<StandardMaterial>,
    >,
    mut materials_std: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, _colliding_entities, v) in &query {
        // println!(
        //     "{:?} is colliding with the following entities: {:?}",
        //     entity,
        //     colliding_entities
        // );
        if let Ok(mat) = standard_materials.get(entity) {
            let m =
                materials_std.get_mut(mat.id()).unwrap();
            m.emissive.as_lcha().set_l(v.y.abs());
            m.base_color.as_lcha().set_l(v.y.abs());
        }
    }
}
