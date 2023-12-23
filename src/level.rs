use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_xpbd_3d::prelude::*;
use rand::Rng;

use crate::brick::parsing::parse_aoc_bricks;

pub const TEST_AOC_INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

pub const GRID_AOC_TEST: &str = "0,0,1~4,4,1
0,0,2~0,0,4
1,1,2~1,1,4
2,2,2~2,2,3
1,2,2~1,2,2
4,4,5~4,4,5
4,4,6~4,4,6
4,4,7~4,4,7
4,4,8~4,4,8
4,4,9~4,4,10
0,0,5~0,0,5
0,0,6~0,0,6
0,0,7~0,0,7
0,0,8~0,0,8
0,0,9~0,0,10";

#[derive(Resource)]
pub struct Input {
    pub raw_text: String,
}

pub fn destroy_level() {}
pub fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<CustomMaterial>>,
    mut materials_std: ResMut<Assets<StandardMaterial>>,
    input: Res<Input>,
) {
    let (_, bricks) = parse_aoc_bricks(&input.raw_text)
        .expect("should parse");

    commands.spawn((
        PbrBundle {
            mesh: meshes
                .add(shape::Plane::from_size(100.0).into()),

            material: materials_std.add(StandardMaterial {
                base_color: Color::hex("313244").unwrap(),
                perceptual_roughness: 1.0,
                ..default()
            }),
            transform: Transform::from_translation(
                Vec3::new(0., 0., 0.5),
            ),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(100.0, 0.002, 100.0),
    ));

    let mut rng = rand::thread_rng();

    for brick in bricks.iter() {
        let size = IVec3::new(
            brick.end.x - brick.start.x + 1,
            brick.end.z - brick.start.z + 1,
            brick.end.y - brick.start.y + 1,
        );
        let b = shape::Box {
            min_x: 0.,
            max_x: size.x as f32,
            min_y: 0.,
            max_y: size.y as f32,
            min_z: 0.,
            max_z: size.z as f32,
        };
        let hue: i32 = rng.gen_range(0..360);

        let color = Color::Lcha {
            lightness: 0.8,
            chroma: 1.0,
            hue: hue as f32,
            alpha: 1.0,
        };

        let translation = brick.start.as_vec3().xzy();
        let translation = Vec3::new(
            translation.x,
            translation.y,
            translation.z,
        );
        // info!(?translation, ?b);
        let material =
            materials_std.add(StandardMaterial {
                base_color: color,
                emissive: color,
                ..default()
            });
        let hovered = materials_std.add(StandardMaterial {
            base_color: Color::hex("313244").unwrap(),
            emissive: Color::hex("313244").unwrap(),
            ..default()
        });
        let pressed = materials_std.add(StandardMaterial {
            base_color: Color::Lcha {
                lightness: 1.,
                chroma: 1.0,
                hue: 60.,
                alpha: 1.0,
            },
            emissive: Color::Lcha {
                lightness: 1.,
                chroma: 1.0,
                hue: 60.,
                alpha: 1.0,
            },
            ..default()
        });
        commands
            .spawn((
                // Anchor::BottomLeft,
                MaterialMeshBundle {
                    mesh: meshes.add(Mesh::from(b)),
                    transform: Transform::from_translation(
                        translation,
                    ),
                    // material: materials.add(CustomMaterial {
                    //     color,
                    //     // color_texture: None,
                    //     alpha_mode: AlphaMode::Blend,
                    // }),
                    material,
                    ..default()
                },
                RigidBody::Dynamic,
                // Collider::
                LockedAxes::ROTATION_LOCKED
                    .lock_translation_x()
                    .lock_translation_z(),
                On::<Pointer<Click>>::target_commands_mut(
                    |_click, target_commands| {
                        target_commands
                            .despawn_descendants();
                        target_commands.despawn();
                    },
                ),
                PickHighlight,
                bevy_mod_picking::prelude::Highlight {
                    hovered: Some(HighlightKind::Fixed(
                        hovered,
                    )),
                    pressed: Some(HighlightKind::Fixed(
                        pressed,
                    )),
                    selected: None,
                },
                SleepingDisabled,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Restitution::new(0.0),
                    Collider::cuboid(
                        size.x as f32 - 0.02,
                        size.y as f32 - 0.02,
                        size.z as f32 - 0.02,
                    ),
                    Transform::from_xyz(
                        size.x as f32 / 2.0,
                        size.y as f32 / 2.0,
                        size.z as f32 / 2.0,
                    ),
                ));
            });
    }
}
