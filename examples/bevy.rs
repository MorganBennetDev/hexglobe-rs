use bevy::{
    prelude::*,
    render::{
        mesh::Indices,
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
};
use bevy::asset::embedded_asset;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use hexglobe::HexGlobe;

fn main() {
    let mut app = App::new();
    
    app.add_plugins((
            DefaultPlugins,
            PanOrbitCameraPlugin
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_debug);
    
    embedded_asset!(app, "", "assets/uv.png");
    
    app.run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let uv_texture = asset_server.load("embedded://bevy/assets/uv.png");
    
    // Create and save a handle to the mesh.
    let mesh_handle: Handle<Mesh> = meshes.add(create_mesh());
    
    // Render the mesh with the custom texture, and add the marker.
    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(uv_texture.clone()),
            ..default()
        })),
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
        PanOrbitCamera::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn draw_debug(
    mut gizmos: Gizmos
) {
    /*
    x - red
    y - green
    z - blue
    */
    gizmos.axes(
        Transform::IDENTITY,
        2.0
    );
}

fn create_mesh() -> Mesh {
    let globe = HexGlobe::<3>::new();
    let centroids = globe.centroids(None);
    let vertices = globe.mesh_vertices(&centroids);
    let faces = globe.mesh_faces();
    let triangles = globe.mesh_triangles(&faces);
    let normals = globe.mesh_normals(&vertices);
    let uvs = globe.mesh_uvs();
    
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vertices
        )
        .with_inserted_indices(Indices::U32(
            triangles
        ))
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            normals
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            uvs
        )
}
