#![feature(generic_const_exprs)]
use std::collections::HashMap;
use bevy::{
    prelude::*,
    render::{
        mesh::Indices,
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
};
use hexglobe::projection::globe::{ExactFace, ExactGlobe};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Create and save a handle to the mesh.
    let mesh_handle: Handle<Mesh> = meshes.add(create_mesh());
    
    // Render the mesh with the custom texture, and add the marker.
    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));
    
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-1.5, 2.5, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn create_mesh() -> Mesh {
    let globe = ExactGlobe::<3>::new();
    let vertices_f32 = globe.vertices_f32(None);
    let vertex_data = vertices_f32.iter()
        .enumerate()
        .collect::<Vec<_>>();
    let index_map = vertex_data.iter()
        .map(|(i, (packed, _))| (**packed, *i as u32))
        .collect::<HashMap<_, _>>();
    
    let vertices = vertex_data.iter()
        .map(|(_, (_, v))| v.to_array())
        .collect::<Vec<_>>();
    
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vertices
        )
        .with_inserted_indices(Indices::U32(
            globe.faces.iter()
                .flat_map(|f| match f {
                    ExactFace::Hexagon(indices) => {
                        vec![
                            index_map.get(&indices[0]).unwrap(), index_map.get(&indices[1]).unwrap(), index_map.get(&indices[2]).unwrap(),
                            index_map.get(&indices[0]).unwrap(), index_map.get(&indices[2]).unwrap(), index_map.get(&indices[3]).unwrap(),
                            index_map.get(&indices[0]).unwrap(), index_map.get(&indices[3]).unwrap(), index_map.get(&indices[4]).unwrap(),
                            index_map.get(&indices[0]).unwrap(), index_map.get(&indices[4]).unwrap(), index_map.get(&indices[5]).unwrap(),
                        ]
                    },
                    ExactFace::Pentagon(indices) => vec![
                        index_map.get(&indices[0]).unwrap(), index_map.get(&indices[1]).unwrap(), index_map.get(&indices[2]).unwrap(),
                        index_map.get(&indices[0]).unwrap(), index_map.get(&indices[2]).unwrap(), index_map.get(&indices[3]).unwrap(),
                        index_map.get(&indices[0]).unwrap(), index_map.get(&indices[3]).unwrap(), index_map.get(&indices[4]).unwrap(),
                    ]
                })
                .cloned()
                .collect::<Vec<_>>()
        ))
        .with_computed_normals()
}
