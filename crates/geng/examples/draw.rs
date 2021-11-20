use geng::prelude::*;

struct State {
    geng: Geng,
    texture: ugli::Texture,
}

impl State {
    fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            texture: ugli::Texture::new_with(geng.ugli(), vec2(2, 2), |pos| {
                Color::rgb(pos.x as f32, pos.y as f32, 0.0)
            }),
        }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
        let camera = geng::Camera2d {
            center: Vec2::ZERO,
            rotation: 0.0,
            fov: 10.0,
        };
        let mut objects = Vec::<Box<dyn geng::draw_2d::DrawTransform2d>>::new();
        objects.push(Box::new(
            draw_2d::Quad::unit(Color::WHITE)
                .transformed(Mat3::rotate(0.5) * Mat3::scale_uniform(0.5)),
        ));
        objects.push(Box::new(draw_2d::TexturedQuad::unit(&self.texture)));
        objects.push(Box::new(draw_2d::Ellipse::unit(Color::RED)));
        objects.push(Box::new(
            draw_2d::Ellipse::unit_with_cut(0.5, Color::RED)
                .transformed(Mat3::rotate(f32::PI / 4.0) * Mat3::scale(vec2(1.0, 0.5))),
        ));
        objects.push(Box::new(draw_2d::Polygon::new_gradient(vec![
            draw_2d::ColoredVertex {
                a_pos: vec2(-1.0, -1.0),
                a_color: Color::RED,
            },
            draw_2d::ColoredVertex {
                a_pos: vec2(1.0, -1.0),
                a_color: Color::GREEN,
            },
            draw_2d::ColoredVertex {
                a_pos: vec2(0.0, 1.0),
                a_color: Color::BLUE,
            },
        ])));

        let mut x = -5.0;
        for drawable in objects {
            self.geng.draw_2d(
                framebuffer,
                &camera,
                &drawable.transformed(Mat3::translate(vec2(x, 0.0))),
            );
            x += 2.0;
        }
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Let's draw!");
    let state = State::new(&geng);
    geng::run(&geng, state)
}