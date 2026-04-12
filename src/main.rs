
use bevy::{
    asset::RenderAssetUsages, camera::{CameraProjection, RenderTarget, visibility::RenderLayers}, color::palettes::css::PURPLE, mesh::{Indices, MeshVertexAttribute, VertexAttributeValues, MeshVertexBufferLayoutRef, PrimitiveTopology, VertexFormat}, prelude::*, render::render_resource::{
    AsBindGroup, Extent3d, RenderPipelineDescriptor, SpecializedMeshPipelineError, TextureDimension, TextureFormat, TextureUsages, VertexAttribute}, shader::ShaderRef, sprite_render::{AlphaMode2d, Material2d, Material2dKey, Material2dPlugin}, transform::components::GlobalTransform, window::PrimaryWindow
};
use shader_demon::{
    materials::{light_material::{self, LightMaterial}, shadow_material::CustomMaterial},
    shadow_mesh::{self, mesh_attributes, mesh_builder},
};


pub const ATTRIBUTE_EDGE_N: MeshVertexAttribute =
    MeshVertexAttribute::new("EdgeN", 110, VertexFormat::Float32x2);
pub const ATTRIBUTE_EDGE_M: MeshVertexAttribute =
    MeshVertexAttribute::new("EdgeM", 111, VertexFormat::Float32x2);
pub const ATTRIBUTE_END_FLAG: MeshVertexAttribute =
    MeshVertexAttribute::new("EndFlag", 112, VertexFormat::Uint32);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
            Material2dPlugin::<LightMaterial>::default(),

        ))
        .add_systems(Startup, setup_game)
        .add_systems(Update, (update_material_uniforms, move_obj, update_light_uniforms))
        .run();
}



const LAYER_WORLD: RenderLayers = RenderLayers::layer(0);
const LAYER_SHADOW: RenderLayers = RenderLayers::layer(1);

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut light_materials: ResMut<Assets<LightMaterial>>,

    mut col_mats: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    windows: Query<&Window, With<PrimaryWindow>>,

)
{

    let win = windows.single().unwrap();
    let w = win.resolution.physical_width();
    let h = win.resolution.physical_height();

    let mask_handle = images.add(make_mask_image(w, h));

    commands.spawn((
        Camera2d,
        Camera {
            order: -1, 
            clear_color: ClearColorConfig::Custom(Color::BLACK), 
            ..default()
        },
        RenderTarget::Image(mask_handle.clone().into()),
        MaskCam,
        LAYER_SHADOW,
    ));

    commands.spawn((
        Camera2d,
        Camera { order: 0, ..default() },
        MainCam,
        LAYER_WORLD,
    ));


  
    let material_handle = materials.add(CustomMaterial {
    color: LinearRgba::WHITE,   
    time: 0.0,
    transform: Mat4::IDENTITY,
    view_proj: Mat4::IDENTITY,
    obj_pos: Vec3::ZERO,
    }
    );

    
let light_mesh = meshes.add(Rectangle::new(2.0, 2.0));
let light_mat = light_materials.add(LightMaterial {
    
    mask: mask_handle.clone(),
    shadow_color: LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 0.7 },
    light_color: LinearRgba { red: 1.0, green: 0.0, blue: 1.0, alpha: 0.7 },
    full_shadow: 0,
    light_radius: 0.5,
    intenisty: 1.25,
    shadow_uv: Vec2::new(0.5, 0.5),

});

    let h: Handle<Mesh> = meshes.add(Triangle2d::default());
    let base_mesh = meshes.get(&h).unwrap();

    let shadow_mesh = {
        mesh_builder::MeshBuilder2d::from_polygon_shadow_quads(base_mesh).build()
    }; 



commands.spawn((
    Mesh2d(light_mesh.clone()),
    MeshMaterial2d(light_mat.clone()),
    Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(156.0)),
    MoveSpeed(50.0),
));


    commands.spawn((
        Mesh2d(meshes.add(shadow_mesh)),
        MeshMaterial2d(material_handle),
        Transform::default().with_scale(Vec3::splat(126.0)),
        ParallelogramObjectTag,
        LAYER_SHADOW,
    ));
     commands.spawn((
        Mesh2d(h.clone()),
        MeshMaterial2d(col_mats.add(Color::from(PURPLE))),
        Transform::default().with_scale(Vec3::splat(126.0)),
        LAYER_WORLD,
    ));

    commands.spawn((
        Mesh2d(h),
        MeshMaterial2d(col_mats.add(Color::linear_rgb(0.5, 0.5, 0.5))),
        Transform::default().with_scale(Vec3::splat(20.0)),
        LAYER_WORLD
    ));
}


#[derive(Component)]
struct ParallelogramObjectTag;

#[derive(Component)]
struct MoveSpeed(f32);

#[derive(Component)]
struct MainCam;
#[derive(Component)]
struct MaskCam;


fn move_obj(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut q_obj: Query<&mut Transform, With<MoveSpeed>>,
    time: Res<Time>,
) {
    let mut movement = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        movement.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        movement.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {    
        movement.x += 1.0;
    }

    if movement.length_squared() > 0.0 {
        movement = movement.normalize();    }

    let delta = time.delta_secs();

    for mut transform in q_obj.iter_mut() {
        transform.translation += movement * 200.0 * delta;  
    }
}


fn update_material_uniforms(
    time: Res<Time>,
    q_mesh: Query<&GlobalTransform, With<ParallelogramObjectTag>>,
    q_circle_mesh: Query<&GlobalTransform, With<MoveSpeed>>,
    q_cam: Query<(&Projection, &GlobalTransform), With<MainCam>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let t = time.elapsed_secs();

    let mesh_gt = q_mesh.single().unwrap();
    let world_affine = mesh_gt.affine();
    let world = Mat4::from(world_affine);

    let (proj_enum, cam_gt) = q_cam.single().unwrap();

    let view = Mat4::from(cam_gt.affine()).inverse();

    let proj = match proj_enum {
        Projection::Orthographic(p) => p.get_clip_from_view(),
        Projection::Perspective(p)  => p.get_clip_from_view(),
        Projection::Custom(p)       => p.get_clip_from_view(),
    };

    let view_proj = proj * view;
 
    for (_, mat) in materials.iter_mut() {
        mat.time      = t;
        mat.transform = world;
        mat.view_proj = view_proj;
        mat.obj_pos = q_circle_mesh.single().unwrap().translation();
    }

}

fn update_light_uniforms(
    q_light_world: Query<&GlobalTransform, With<MoveSpeed>>, 
    q_cam: Query<(&Projection, &GlobalTransform), With<MainCam>>,
    mut light_materials: ResMut<Assets<LightMaterial>>,
) {
    let light_gt= q_light_world.single().unwrap();

    let (proj_enum, cam_gt) = q_cam.single().unwrap();

    let view = Mat4::from(cam_gt.affine()).inverse();

    let proj = match proj_enum {
        Projection::Orthographic(p) => p.get_clip_from_view(),
        Projection::Perspective(p)  => p.get_clip_from_view(),
        Projection::Custom(p)       => p.get_clip_from_view(),
    };

    let view_proj = proj * view;

    let world = light_gt.translation();
    let clip = view_proj * world.extend(1.0);
    let ndc = clip.truncate() / clip.w;

    let mut uv = Vec2::new(ndc.x * 0.5 + 0.5, ndc.y * 0.5 + 0.5);
    uv.y = 1.0 - uv.y;
    for (_, mat) in light_materials.iter_mut() {
        mat.shadow_uv = uv;
    }
}


fn make_mask_image(width: u32, height: u32) -> Image {
    let mut img = Image::new_fill(
        Extent3d { width, height, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &[0, 0, 0, 0],                 
        TextureFormat::R8Unorm,        
        RenderAssetUsages::default(),
    );
    img.texture_descriptor.usage = TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING;
    img
}

