use bevy::asset::{Assets, Handle};
use bevy::math::*;
use bevy::prelude::{Color, Mesh, ResMut};
use bevy::render::mesh::{PrimitiveTopology, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;

pub struct MeshBuilder {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
    uv_coordinates: Option<Vec<[f32; 2]>>,
}

impl MeshBuilder {
    pub fn create() -> Self {
        Self {
            positions: Vec::new(),
            normals: Vec::new(),
            colors: Vec::new(),
            uv_coordinates: None,
        }
    }

    pub fn create_with_uv_coordinates() -> Self {
        Self {
            positions: Vec::new(),
            normals: Vec::new(),
            colors: Vec::new(),
            uv_coordinates: Some(Vec::new()),
        }
    }

    pub fn add_tree(&mut self, position: Vec3, height: u32) {
        for y in 0..height {
            self.add_cube(
                Vec3::new(position.x, position.y + y as f32, position.z),
                Color::rgb(0.647, 0.165, 0.165),
            );
        }
        let radius = 4;
        //  add a sphere of leaves at the top within a radius
        for x in -radius..=radius {
            for z in -radius..=radius {
                for y in -radius..=radius {
                    if Vec3::new(x as f32, y as f32, z as f32).length() <= radius as f32 {
                        self.add_cube(
                            Vec3::new(
                                position.x + x as f32,
                                position.y + y as f32 + height as f32,
                                position.z + z as f32,
                            ),
                            Color::rgb(0.0, 1.0, 0.0),
                        );
                    }
                }
            }
        }
    }

    pub fn add_cube(&mut self, position: Vec3, color: Color) {
        self.add_cube_internal(position, color, 1.);
    }

    pub fn add_cube_inverted(&mut self, position: Vec3, color: Color) {
        self.add_cube_internal(position, color, -1.);
    }

    fn add_cube_internal(&mut self, position: Vec3, color: Color, explode: f32) {
        //  positive z face
        self.add_face(
            position,
            color,
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.5, -0.5, 0.0),
            explode,
        );
        //  negative z face
        self.add_face(
            position,
            color,
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(-0.5, -0.5, 0.0),
            explode,
        );
        //  positive x face
        self.add_face(
            position,
            color,
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, -0.5, -0.5),
            explode,
        );
        //  negative x face
        self.add_face(
            position,
            color,
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, -0.5, 0.5),
            explode,
        );
        //  positive y face
        self.add_face(
            position,
            color,
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.5, 0.0, 0.5),
            explode,
        );
        //  negative y face
        self.add_face(
            position,
            color,
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(-0.5, 0.0, 0.5),
            explode,
        );
    }

    pub fn add_face(
        &mut self,
        position: Vec3,
        color: Color,
        normal: Vec3,
        //  bot_right_corner_tangent is a vector pointing from the center of the face to the bottom right corner.
        bot_right_corner_tangent: Vec3,
        explode: f32,
    ) {
        let position = position + normal * explode;
        let x = position.x;
        let y = position.y;
        let z = position.z;

        let top_right_corner_tangent = normal.cross(bot_right_corner_tangent);
        //  Vec3::new(0.5, -0.5, 0.0)
        let center = position + normal / 2.0;
        let top_left = center - bot_right_corner_tangent;
        let top_right = center + top_right_corner_tangent;
        let bot_right = center + bot_right_corner_tangent;
        let bot_left = center - top_right_corner_tangent;

        self.positions.extend(
            [
                top_left.to_array(),
                bot_left.to_array(),
                bot_right.to_array(),
                top_left.to_array(),
                bot_right.to_array(),
                top_right.to_array(),
            ]
                .iter(),
        );
        let normal_elements = [normal.x, normal.y, normal.z];
        self.normals.extend(
            [
                normal_elements,
                normal_elements,
                normal_elements,
                normal_elements,
                normal_elements,
                normal_elements,
            ]
                .iter(),
        );
        let color_elements = [color.r(), color.g(), color.b(), color.a()];
        self.colors.extend(
            [
                color_elements,
                color_elements,
                color_elements,
                color_elements,
                color_elements,
                color_elements,
            ]
                .iter(),
        );

        if let Some(uv_coordinates) = &mut self.uv_coordinates {
            let top_left_uv = Vec2::new(0., 0.);
            let top_right_uv = Vec2::new(1., 0.);
            let bot_left_uv = Vec2::new(0., 1.);
            let bot_right_uv = Vec2::new(1., 1.);
            uv_coordinates.extend(
                [
                    top_left_uv.to_array(),
                    bot_left_uv.to_array(),
                    bot_right_uv.to_array(),
                    top_left_uv.to_array(),
                    bot_right_uv.to_array(),
                    top_right_uv.to_array(),
                ]
                    .iter(),
            )
        }
    }

    pub fn to_mesh(self, mut meshes: ResMut<Assets<Mesh>>) -> Handle<Mesh> {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(self.positions),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float32x3(self.normals),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            VertexAttributeValues::Float32x4(self.colors),
        );

        if let Some(uv_coordinates) = self.uv_coordinates {
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_UV_0,
                VertexAttributeValues::Float32x2(uv_coordinates)
            );
        }

        let mesh_handle = meshes.add(mesh);
        mesh_handle
    }
}


