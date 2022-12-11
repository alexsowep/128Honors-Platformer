use crate::components::*;
use bevy::{prelude::*, ecs::query};
use bevy_ecs_ldtk::prelude::*;

use std::collections::{HashMap, HashSet};

use bevy_rapier2d::{prelude::*, rapier::prelude::Cuboid};
const TIME_STEP: f32 = 1.0 / 60.0;
struct Texture {
    player_laser: Handle<Image>,
}
const BASE_SPEED: f32 = 500.;
#[derive(Component)]
pub struct Laser;

pub fn laser_system(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>
) {
    if let Ok(player_tf) = query.get_single() {
        if mouse.pressed(MouseButton::Left) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);

            let game_textures = Texture {
                player_laser: asset_server.load("PlayerLaser.png")
            };
            commands
                .spawn_bundle(SpriteBundle {
                texture: game_textures.player_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(VelocityLaser {x: 1., y: 0.});
            //.insert(Movable {auto_despawn: true})
        }
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &VelocityLaser), With<Laser>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        transform.translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}


       
        // Use timer to determine the amount of time that the sprite is not on the ground
        // If the timer exceeeds two seconds then reset the game state -> This signifies that
        // the player has fell off the map


        // This is so that the player sprite cant infinitely jump -> Ray Casting would be better
        // as in the future I could implement like a double jump feauture. 
        
   

pub fn movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &GroundDetection), With<Player>>,
) {
    for (mut velocity,  ground_detection) in &mut query {
        let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::A) { 1. } else { 0. };

        velocity.linvel.x = (right - left) * 200.;

       
        // Use timer to determine the amount of time that the sprite is not on the ground
        // If the timer exceeeds two seconds then reset the game state -> This signifies that
        // the player has fell off the map


        // This is so that the player sprite cant infinitely jump -> Ray Casting would be better
        // as in the future I could implement like a double jump feauture. 
        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground) {
            velocity.linvel.y = 500.;
        } 
    }
}


pub fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    
    
    
    // THE LOGIC BELOW IS USED TO IMPLEMENT A COLLSIONS AND GROUND DETECTION SYSTEM
    // USED FROM THE BEVY_ECS_LDTK EXAMPLE FOLDER

    
    
    
    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    wall_query.for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_insert(HashSet::new())
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        level_query.for_each(|(level_entity, level_handle)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let level = levels
                    .get(level_handle)
                    .expect("Level should be loaded by this point");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level
                    .level
                    .layer_instances
                    .clone()
                    .expect("Level asset should have layers")[0];

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right
                    // edge
                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut wall_rects: Vec<Rect> = Vec::new();
                let mut previous_rects: HashMap<Plate, Rect> = HashMap::new();

                // an extra empty row so the algorithm "terminates" the rects that touch the top
                // edge
                plate_stack.push(Vec::new());

                for (y, row) in plate_stack.iter().enumerate() {
                    let mut current_rects: HashMap<Plate, Rect> = HashMap::new();
                    for plate in row {
                        if let Some(previous_rect) = previous_rects.remove(plate) {
                            current_rects.insert(
                                *plate,
                                Rect {
                                    top: previous_rect.top + 1,
                                    ..previous_rect
                                },
                            );
                        } else {
                            current_rects.insert(
                                *plate,
                                Rect {
                                    bottom: y as i32,
                                    top: y as i32,
                                    left: plate.left,
                                    right: plate.right,
                                },
                            );
                        }
                    }

                    // Any plates that weren't removed above have terminated
                    wall_rects.append(&mut previous_rects.values().copied().collect());
                    previous_rects = current_rects;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for wall_rect in wall_rects {
                        level
                            .spawn()
                            .insert(Collider::cuboid(
                                    (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                        * grid_size as f32
                                        / 2.,
                                    (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                        * grid_size as f32
                                        / 2.,
                            ))
                            .insert(RigidBody::Fixed)
                            .insert(Friction{
                                coefficient: 0.1,
                                combine_rule:
                                    CoefficientCombineRule::Min,
                            })
                            .insert(Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                    / 2.,
                                0.,
                            ))
                            .insert(GlobalTransform::default());
                    }
                });
            }
        });
    }
}


const ASPECT_RATIO: f32 = 21. / 9.;

pub fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        Without<Player>,
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    
    {
        let player_translation = *player_translation;

        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in &level_query {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, level) {
                    let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;

                    orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
                    orthographic_projection.bottom = 0.;
                    orthographic_projection.left = 0.;
                     if level_ratio > ASPECT_RATIO {
                        // level is wider than the screen
                        orthographic_projection.top = (level.px_hei as f32 / 9.).round() * 9.;
                        orthographic_projection.right = orthographic_projection.top * ASPECT_RATIO;
                        camera_transform.translation.x = (player_translation.x
                            - level_transform.translation.x
                            - orthographic_projection.right / 2.)
                            .clamp(0., level.px_wid as f32 - orthographic_projection.right);
                        camera_transform.translation.y = 0.;
                    } else {
                        // level is taller than the screen
                        orthographic_projection.right = (level.px_wid as f32 / 16.).round() * 16.;
                        orthographic_projection.top = orthographic_projection.right / ASPECT_RATIO;
                        camera_transform.translation.y = (player_translation.y
                            - level_transform.translation.y
                            - orthographic_projection.top / 2.)
                            .clamp(0., level.px_hei as f32 - orthographic_projection.top);
                        camera_transform.translation.x = 0.;
                    }

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                }
            }
        }
    }
}
    


pub fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<
        (Entity, &Collider, &Transform),
        Added<GroundDetection>,
    >,
) {
    for (entity, shape, transform) in
        detect_ground_for.iter()
    {
        if let Some(Cuboid { half_extents }) =
            shape.raw.0.as_cuboid()
        {
            commands.entity(entity).with_children(
                |builder| {
                    builder
                        .spawn()
                        .insert(Sensor)
                        .insert(Collider::cuboid(
                            half_extents.x / 2.,
                            2.,
                        ))
                        .insert(
                            ActiveEvents::COLLISION_EVENTS,
                        )
                        .insert(
                            Transform::from_translation(
                                Vec3::new(
                                    0.,
                                    -half_extents.y,
                                    0.,
                                ) / transform.scale,
                            ),
                        )
                        .insert(GlobalTransform::default())
                        .insert(GroundSensor {
                            ground_detection_entity: entity,
                            intersecting_ground_entities:
                                HashSet::new(),
                        });
                },
            );
        }
    }
}

pub fn ground_detection(
    mut ground_sensors: Query<&mut GroundSensor>,
    mut collisions: EventReader<CollisionEvent>,
    collidables: Query<With<Collider>, Without<Sensor>>,
) {
    for collision_event in collisions.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e2) {
                        sensor.intersecting_ground_entities.insert(*e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e1) {
                        sensor.intersecting_ground_entities.insert(*e2);
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e2) {
                        sensor.intersecting_ground_entities.remove(e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e1) {
                        sensor.intersecting_ground_entities.remove(e2);
                    }
                }
            }
        }
    }
}

pub fn update_on_ground(
    mut ground_detectors: Query<&mut GroundDetection>,
    ground_sensors: Query<&GroundSensor, Changed<GroundSensor>>,
) {
    for sensor in &ground_sensors {
        if let Ok(mut ground_detection) = ground_detectors.get_mut(sensor.ground_detection_entity) {
            ground_detection.on_ground = !sensor.intersecting_ground_entities.is_empty();
        }
    }
}