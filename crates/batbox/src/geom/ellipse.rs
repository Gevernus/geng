use super::*;

pub struct Ellipse {
    matrix: Mat3<f32>,
}

impl Ellipse {
    pub fn new(center: Vec2<f32>, size: Vec2<f32>) -> Self {
        Self::unit().transform(Mat3::translate(center) * Mat3::scale(size))
    }
    pub fn circle(center: Vec2<f32>, radius: f32) -> Self {
        Self::unit().transform(Mat3::translate(center) * Mat3::scale_uniform(radius))
    }
    pub fn unit() -> Self {
        Self {
            matrix: Mat3::identity(),
        }
    }
    pub fn matrix(&self) -> Mat3<f32> {
        self.matrix
    }
}

impl Transform2d for Ellipse {
    fn bounding_quad(&self) -> Quad<f32> {
        Quad::from_matrix(self.matrix)
    }
    fn apply_transform(&mut self, transform: Mat3<f32>) {
        self.matrix = transform * self.matrix;
    }
}

impl FitTarget2d for Ellipse {
    fn make_fit(&self, object: &mut impl Transform2d) {
        let inversed_matrix = self.matrix().inverse();
        let quad_in_circle = object.bounding_quad().transform(inversed_matrix);
        let center = (quad_in_circle.matrix() * vec3(0.0, 0.0, 1.0)).into_2d();
        let corner = (quad_in_circle.matrix() * vec3(1.0, 1.0, 1.0)).into_2d();
        let local_transform =
            Mat3::scale_uniform(1.0 / (corner - center).len()) * Mat3::translate(-center);
        object.apply_transform(self.matrix() * local_transform * inversed_matrix)
    }
}