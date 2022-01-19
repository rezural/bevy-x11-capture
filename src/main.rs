use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use x11cap::{CaptureSource, Capturer};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(capture)
        .run();
}

#[derive(Default)]
struct Windows {
    pub windows: Vec<(Entity, Handle<Image>, Handle<StandardMaterial>)>,
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(Windows::default());

    // plane
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..Default::default()
    // });
    // // cube
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..Default::default()
    // });
    // // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn capture(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut textures: ResMut<Assets<Image>>,
    mut windows: ResMut<Windows>,
) {
    let mut capturer = Capturer::new(CaptureSource::Monitor(0)).unwrap();
    let ps = capturer.capture_frame().unwrap();
    let geom = capturer.get_geometry();
    let size = Extent3d {
        height: geom.height,
        width: geom.width,
        ..Default::default()
    };

    let texture: Vec<u8> = ps
        .as_slice()
        // .chunks(geom.width as usize)
        // .rev()
        // .flatten()
        .iter()
        .map(|v| [v.b, v.g, v.r, 255])
        .flatten()
        .collect();

    let image = Image::new(
        size,
        TextureDimension::D2,
        texture.clone(),
        TextureFormat::Bgra8Unorm,
    );

    if windows.windows.is_empty() {
        info!("empty");
        let texture_handle = textures.add(image);
        let width = 5.0;
        let height = width * (geom.height as f32 / geom.width as f32);
        let rotation = Quat::IDENTITY;
        let x_rot = Quat::from_axis_angle(Vec3::X, PI);
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            double_sided: true,
            ..Default::default()
        });
        let entity = commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Quad {
                    size: Vec2::new(width, height),
                    flip: false,
                })),
                material: material_handle.clone(),
                transform: Transform::from_rotation(rotation),
                ..Default::default()
            })
            .id();
        windows
            .windows
            .push((entity, texture_handle, material_handle));
    } else {
        // info!("getting new texture");
        let texture_handle = textures.add(image);

        let (entity, _, material_handle) = windows.windows.first().unwrap();
        let mut material = materials.get_mut(material_handle).unwrap();
        material.base_color_texture = Some(texture_handle);
    }

    // commands.entity(entity).commands.tex
}
