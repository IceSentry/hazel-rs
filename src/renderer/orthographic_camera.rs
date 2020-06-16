use glam::{Mat4, Vec3};
use lazy_static::lazy_static;

lazy_static! {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    static ref OPENGL_TO_WGPU_MATRIX: glam::Mat4 = glam::Mat4::from_cols_array(&[
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0
    ]);
}

pub struct OrthographicCamera {
    pub projection_matrix: Mat4,
    pub view_projection_matrix: Mat4,
    pub view_matrix: Mat4,
    pub rotation: f32,
    pub position: Vec3,
}

impl OrthographicCamera {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        let projection_matrix = glam::Mat4::orthographic_rh_gl(left, right, bottom, top, -1.0, 1.0);
        let view_matrix = glam::Mat4::identity();
        let view_projection_matrix = *OPENGL_TO_WGPU_MATRIX * projection_matrix * view_matrix;

        Self {
            projection_matrix,
            view_projection_matrix,
            view_matrix,
            rotation: Default::default(),
            position: Default::default(),
        }
    }

    pub fn recalculate_view_matrix(&mut self) {
        let transform_matrix = glam::Mat4::from_translation(self.position)
            * glam::Mat4::from_rotation_ypr(0.0, 0.0, self.rotation);

        self.view_matrix = transform_matrix.inverse();
        self.view_projection_matrix =
            *OPENGL_TO_WGPU_MATRIX * self.projection_matrix * self.view_matrix;
    }
}
