use super::buffers::{ArrayBuffer, IndexBuffer};
use super::color::*;
use super::point_system::*;
use crate::shader::{ShaderKind, ShaderSystem};
use camera::Camera;
use implicit_mesh::cell_keys::morton_keys::MortonKey;
use implicit_mesh::function_ir::Node;
use implicit_mesh::mesh_tree::*;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;
use web_sys::WebGlRenderingContext as GL;

pub struct PlotBuffers {
    points: PointSystem,

    edge_count: i32,
    edge_vertices_buffer: ArrayBuffer,
    edge_indices_buffer: IndexBuffer,

    bb_edge_count: i32,
    bb_vertices_buffer: ArrayBuffer,
    bb_indices_buffer: IndexBuffer,
}

impl PlotBuffers {
    pub fn new(
        gl_context: &WebGlRenderingContext,
        mtree: &MeshTree<MortonKey, Node>,
    ) -> Result<PlotBuffers, JsValue> {
        let points = {
            let mut points_builder = PointSystemBuilder::new(gl_context);

            points_builder.set_radius(1.0);
            points_builder.set_color(Color::from_floats(0.33, 0.86, 0.42, 1.0));

            for point in mtree.get_vertex_points() {
                points_builder.add_point(point);
            }

            points_builder.finish()?
        };

        let edge_float_vec = mtree.get_edge_floats();
        let edge_vertex_count = edge_float_vec.len() / 3;
        let edge_vertices_buffer = ArrayBuffer::new(gl_context, edge_float_vec)?;
        let edge_indices_buffer =
            IndexBuffer::new(gl_context, (0..edge_vertex_count as u16).collect())?;

        let bb_float_vec = mtree.get_bounding_box_floats();
        let bb_vertex_count = bb_float_vec.len() / 3;
        let bb_vertices_buffer = ArrayBuffer::new(gl_context, bb_float_vec)?;
        let bb_indices_buffer =
            IndexBuffer::new(gl_context, (0..bb_vertex_count as u16).collect())?;

        Ok(PlotBuffers {
            points,
            edge_count: (edge_vertex_count / 2) as i32,
            edge_vertices_buffer,
            edge_indices_buffer,
            bb_edge_count: (bb_vertex_count / 2) as i32,
            bb_vertices_buffer,
            bb_indices_buffer,
        })
    }

    pub fn render(
        &self,
        gl_context: &WebGlRenderingContext,
        shader_sys: &ShaderSystem,
        camera: &Camera,
        draw_edges: bool,
        draw_bb: bool,
        draw_points: bool,
    ) {
        shader_sys.use_program(gl_context, ShaderKind::Simple);

        let width = gl_context.drawing_buffer_width();
        let height = gl_context.drawing_buffer_height();
        gl_context.viewport(0, 0, width, height);

        // Load in the object transfrom
        let object_transform_uniform = &shader_sys.simple_shader.object_transform_uniform;

        let mut object_transform_matrix = camera.get_world_to_clipspace_transform();
        let object_transform_mut_ref: &mut [f32; 16] = object_transform_matrix.as_mut();

        gl_context.uniform_matrix4fv_with_f32_array(
            Some(object_transform_uniform),
            false,
            object_transform_mut_ref.as_mut(),
        );

        let color_uniform = &shader_sys.simple_shader.color_uniform;
        let position_attribute = shader_sys.simple_shader.position_attribute;

        if draw_edges {
            let mut edge_color = Color::from_floats(0.2, 0.33, 0.84, 1.0);
            gl_context.uniform4fv_with_f32_array(Some(color_uniform), &mut edge_color);

            gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&self.edge_vertices_buffer.gl_buffer));
            gl_context.bind_buffer(
                GL::ELEMENT_ARRAY_BUFFER,
                Some(&self.edge_indices_buffer.gl_buffer),
            );
            gl_context.vertex_attrib_pointer_with_i32(
                position_attribute,
                3,
                GL::FLOAT,
                false,
                0,
                0,
            );
            gl_context.enable_vertex_attrib_array(position_attribute);
            gl_context.draw_elements_with_i32(
                GL::LINES,
                self.edge_count * 2,
                GL::UNSIGNED_SHORT,
                0,
            );
        }

        if draw_bb {
            let mut edge_color = Color::from_floats(0.68, 0.04, 0.23, 1.0);
            gl_context.uniform4fv_with_f32_array(Some(color_uniform), &mut edge_color);

            gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&self.bb_vertices_buffer.gl_buffer));
            gl_context.bind_buffer(
                GL::ELEMENT_ARRAY_BUFFER,
                Some(&self.bb_indices_buffer.gl_buffer),
            );
            gl_context.vertex_attrib_pointer_with_i32(
                position_attribute,
                3,
                GL::FLOAT,
                false,
                0,
                0,
            );
            gl_context.enable_vertex_attrib_array(position_attribute);
            gl_context.draw_elements_with_i32(
                GL::LINES,
                self.bb_edge_count * 2,
                GL::UNSIGNED_SHORT,
                0,
            );
        }

        if draw_points {
            //self.points.render(gl_context, shader_sys, camera);
            // Setup GL State
            shader_sys.use_program(gl_context, ShaderKind::BillBoard);
            let width = gl_context.drawing_buffer_width();
            let height = gl_context.drawing_buffer_height();
            gl_context.viewport(0, 0, width, height);

            // Bind uniforms
            let camera_up_uniform = &shader_sys.billboard_shader.camera_up_uniform;
            let mut camera_up = camera.get_up();
            let camera_up_mut_ref: &mut [f32; 3] = camera_up.as_mut();
            gl_context
                .uniform3fv_with_f32_array(Some(camera_up_uniform), camera_up_mut_ref.as_mut());

            let camera_right_uniform = &shader_sys.billboard_shader.camera_right_uniform;
            let mut camera_right = camera.get_right();
            let camera_right_mut_ref: &mut [f32; 3] = camera_right.as_mut();
            gl_context.uniform3fv_with_f32_array(
                Some(camera_right_uniform),
                camera_right_mut_ref.as_mut(),
            );

            let worldspace_transform_uniform =
                &shader_sys.billboard_shader.worldspace_transform_uniform;
            let mut worldspace_transform = camera.get_world_to_clipspace_transform();
            let worldspace_transform_mut_ref: &mut [f32; 16] = worldspace_transform.as_mut();
            gl_context.uniform_matrix4fv_with_f32_array(
                Some(worldspace_transform_uniform),
                false,
                worldspace_transform_mut_ref.as_mut(),
            );

            let color_uniform = &shader_sys.billboard_shader.color_uniform;
            gl_context.uniform4fv_with_f32_array(Some(color_uniform), &mut BLACK.clone());

            let bpa = ArrayBuffer::new(
                gl_context,
                vec![
                    -2.0, 2.0, 2.0, 2.0, 2.0, -2.0, -2.0, -2.0, -2.0, 2.0, 2.0, 2.0, 2.0, -2.0,
                    -2.0, -2.0,
                ],
            )
            .unwrap();

            let bca = ArrayBuffer::new(
                gl_context,
                vec![
                    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 20.0, 20.0, 20.0,
                    20.0, 20.0, 20.0, 20.0, 20.0, 20.0, 20.0, 20.0, 20.0,
                ],
            )
            .unwrap();

            let indices =
                IndexBuffer::new(gl_context, vec![0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4]).unwrap();

            let board_position_attribute = shader_sys.billboard_shader.board_position_attribute;
            gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&bpa.gl_buffer));
            gl_context.vertex_attrib_pointer_with_i32(
                board_position_attribute,
                2,
                GL::FLOAT,
                false,
                0,
                0,
            );
            gl_context.enable_vertex_attrib_array(board_position_attribute);

            let board_center_attribute = shader_sys.billboard_shader.board_center_attribute;
            gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&bca.gl_buffer));
            gl_context.vertex_attrib_pointer_with_i32(
                board_center_attribute,
                3,
                GL::FLOAT,
                false,
                0,
                0,
            );
            gl_context.enable_vertex_attrib_array(board_center_attribute);

            gl_context.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&indices.gl_buffer));

            //draw
            gl_context.draw_elements_with_i32(GL::TRIANGLES, 8, GL::UNSIGNED_SHORT, 0);
        }
    }
}
