pub mod arrow;

use bevy::{
    math::{
        Vec2, Vec3
    },
    render::{
        mesh::{
            shape, Indices, Mesh, VertexAttributeValues
        }, render_resource::PrimitiveTopology
    },
};

pub use arrow::{
    BevyTridentCone,
    BevyTridentArrow
};


#[derive(Debug, Clone, Copy, Default)]
pub struct BevyTridentAxis {
    pub axises: [BevyTridentArrow; 3],
}

impl BevyTridentAxis {
    pub const TRIDENT_10: BevyTridentAxis = BevyTridentAxis {
        axises: [
            BevyTridentArrow::TRIDENT_ARROW_10,
            BevyTridentArrow::TRIDENT_ARROW_10,
            BevyTridentArrow::TRIDENT_ARROW_10,
        ]
    };
    pub const TRIDENT_100: BevyTridentAxis = BevyTridentAxis {
        axises: [
            BevyTridentArrow::TRIDENT_ARROW_100,
            BevyTridentArrow::TRIDENT_ARROW_100,
            BevyTridentArrow::TRIDENT_ARROW_100,
        ],
    };

}

impl From<BevyTridentAxis> for Mesh {
    fn from(trident: BevyTridentAxis) -> Self {
        let (x_points, x_normals, x_uvs, x_indices, x_colors) = trident.gen_axis(0);
        let (y_points, y_normals, y_uvs, y_indices, y_colors) = trident.gen_axis(1);
        let (z_points, z_normals, z_uvs, z_indices, z_colors) = trident.gen_axis(2);
        let (o_points, o_normals, o_uvs, o_indices, o_colors) = trident.gen_origin(
            x_points.len() + y_points.len() + z_points.len()
        );

        let positions = [x_points, y_points, z_points, o_points].concat();
        let normals = [x_normals, y_normals, z_normals, o_normals].concat();
        let uvs = [x_uvs, y_uvs, z_uvs, o_uvs].concat();
        let indices = [x_indices, y_indices, z_indices, o_indices].concat();
        let colors = [x_colors, y_colors, z_colors, o_colors].concat();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh
    }
}

impl BevyTridentAxis {
    fn gen_axis(&self, dir: usize)
    -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>, Vec<[f32; 4]>) {
        let d = self.axises[dir].cone.subdivisions;
        let n_vertices = 3 * (self.axises[dir].cone.subdivisions + 1) + 2;
        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(n_vertices);
        let mut indices: Vec<u32> = Vec::with_capacity(n_vertices);

        positions.resize_with(n_vertices, Default::default);
        normals.resize_with(n_vertices, Default::default);
        uvs.resize_with(n_vertices, Default::default);

        let uv_stride = match dir {
            0 => Vec2::ZERO,
            1 => Vec2::new(0.5, 0.0),
            2 => Vec2::new(0.0, 0.5),
            _ => panic!("Invalid axis"),
        };

        let cr = self.axises[dir].cone.radius;
        let tr = self.axises[dir].tail_radius;
        // top
        let mut top = Vec3::ZERO;
        top[dir] = self.axises[dir].tail_length + self.axises[dir].cone.height;
        positions[0] = top.into();
        normals[0] = top.normalize().into();
        normals[1] = (-top.normalize()).into();
        uvs[0] = (Vec2::new(0.0, 0.25) + uv_stride).into();
        uvs[1] = (Vec2::new(0.5, 0.25) + uv_stride).into();

        // Cone circular
        let stride = 2.0 * std::f32::consts::PI / self.axises[dir].cone.subdivisions as f32;
        (0..=self.axises[dir].cone.subdivisions).for_each(|i| {
            let phi = i as f32 * stride;
            let vertice = match dir {
                0 => Vec3::new(0.0, phi.cos(), phi.sin()),
                1 => Vec3::new(phi.sin(), 0.0, phi.cos()),
                2 => Vec3::new(phi.cos(), phi.sin(), 0.0),
                _ => panic!("Invalid axis"),
            };

            let mut cone_vertice = vertice * cr;
            let mut converge_vertice = vertice * tr;
            let cylinder_vertice = vertice * tr;
            cone_vertice[dir] = self.axises[dir].tail_length;
            converge_vertice[dir] = self.axises[dir].tail_length;

            let i1 = i + 2;
            let i2 = i + (self.axises[dir].cone.subdivisions + 1) + 2;
            let i3 = i + 2 * (self.axises[dir].cone.subdivisions + 1) + 2;

            // position
            positions[i1] = cone_vertice.into();
            positions[i2] = converge_vertice.into();
            positions[i3] = cylinder_vertice.into();

            // normal
            let unit_to_top = top.normalize();
            let unit_converge_to_cone = (cone_vertice - converge_vertice).normalize();
            let unit_cylinder = (cylinder_vertice - converge_vertice).normalize();
            // normal cone
            let tmp = unit_converge_to_cone.cross(unit_to_top).normalize();
            let cone_normal = ((top - cone_vertice).normalize()).cross(tmp).normalize();
            let converge_normal = (unit_converge_to_cone + unit_cylinder).normalize();
            let cylinder_normal = (unit_cylinder + cylinder_vertice.normalize()).normalize();
            normals[i1] = cone_normal.into();
            normals[i2] = converge_normal.into();
            normals[i3] = cylinder_normal.into();

            // uv
            uvs[i1] = (Vec2::new(0.25, i as f32 / d as f32) / 2.0 + uv_stride).into();
            uvs[i2] = (Vec2::new(0.50, i as f32 / d as f32) / 2.0 + uv_stride).into();
            uvs[i3] = (Vec2::new(0.75, i as f32 / d as f32) / 2.0 + uv_stride).into();
        });

        // indices
        for point in 2..d + 2 {
            let top = 0;
            let bottom = 1;

            let top_left = point + 1;
            let top_right = point;

            let bottom_left =  2 * (d + 1) + point + 1;
            let bottom_right = 2 * (d + 1) + point;

            indices.push(top as u32);
            indices.push(top_right as u32);
            indices.push(top_left as u32);

            indices.push(bottom as u32);
            indices.push(bottom_right as u32);
            indices.push(bottom_left as u32);
        }
        for sub in 0..d {
            let i_cone = sub + 2;
            let i_cone_next = i_cone + 1;
            let i_converge = i_cone + (d + 1);
            let i_converge_next = i_converge + 1;
            let i_cylinder = i_cone + 2 * (d + 1);
            let i_cylinder_next = i_cylinder + 1;

            indices.push(i_cone as u32);
            indices.push(i_converge_next as u32);
            indices.push(i_cone_next as u32);

            indices.push(i_converge as u32);
            indices.push(i_converge_next as u32);
            indices.push(i_cone as u32);

            indices.push(i_converge as u32);
            indices.push(i_cylinder_next as u32);
            indices.push(i_converge_next as u32);

            indices.push(i_converge as u32);
            indices.push(i_cylinder as u32);
            indices.push(i_cylinder_next as u32);
        }
        if dir != 0 {
            let axis_stride = dir * n_vertices;
            for index in indices.iter_mut() {
                *index += axis_stride as u32;
            }
        }
        let mut colors: Vec<[f32; 4]> = vec![];
        colors.resize_with(
            positions.len(),
            || {
                match dir {
                    0 => [1.0, 0.0, 0.0, 1.0],
                    1 => [0.0, 1.0, 0.0, 1.0],
                    2 => [0.0, 0.0, 1.0, 1.0],
                    _ => panic!("Invalid axis"),
                }
            }
        );

        (
            positions,
            normals,
            uvs,
            indices,
            colors
        )
    }

    fn gen_origin(&self, shift: usize)
    -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>, Vec<[f32; 4]>)  {
        let mut radius = self.axises[0].tail_radius;
        self.axises.iter().skip(1).for_each(|axis| {
            if axis.tail_radius > radius {
                radius = axis.tail_radius;
            }
        });
        // Choose largest radius for sphere
        let sphere = shape::UVSphere {
            radius,
            sectors: 8,
            stacks: 8,
        };
        let mesh = Mesh::from(sphere);

        let positions = if let Some(VertexAttributeValues::Float32x3(vert_positions)) =
            &mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            vert_positions.clone()
        } else {
            panic!("no positions")
        };
        let normals = if let Some(VertexAttributeValues::Float32x3(normals)) =
            &mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
        {
            normals.clone()
        } else {
            panic!("no normals")
        };
        let uvs = if let Some(VertexAttributeValues::Float32x2(uvs)) =
            &mesh.attribute(Mesh::ATTRIBUTE_UV_0)
        {
            uvs.clone()
        } else {
            panic!("no uvs")
        };

        let shift = shift as u32;
        let indices: Vec<u32> = if let Some(Indices::U32(indices)) =
            &mesh.indices()
        {
            indices.iter().map(|i| *i + shift).collect()
        } else {
            panic!("no indices")
        };
        let mut colors: Vec<[f32; 4]> = vec![];
        colors.resize_with(positions.len(), || [1.0, 1.0, 1.0, 1.0]);

        (
            positions,
            normals,
            uvs,
            indices,
            colors
        )
    }
}

