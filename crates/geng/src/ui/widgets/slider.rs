use super::*;

pub struct Slider<'a> {
    sense: &'a mut Sense,
    pos: &'a mut Option<AABB<f64>>,
    tick_radius: &'a mut f32,
    value: f64,
    range: RangeInclusive<f64>,
    f: Box<dyn FnMut(f64) + 'a>,
}

impl<'a> Slider<'a> {
    const ANIMATION_SPEED: f32 = 5.0;

    pub fn new(
        cx: &'a Controller,
        value: f64,
        range: RangeInclusive<f64>,
        f: Box<dyn FnMut(f64) + 'a>,
    ) -> Self {
        Slider {
            sense: cx.get_state(),
            tick_radius: cx.get_state(),
            pos: cx.get_state(),
            value,
            range,
            f,
        }
    }
}

impl<'a> Widget for Slider<'a> {
    fn sense(&mut self) -> Option<&mut Sense> {
        Some(self.sense)
    }
    fn update(&mut self, delta_time: f64) {
        let target_tick_radius = if self.sense.is_hovered() || self.sense.is_captured() {
            1.0 / 2.0
        } else {
            1.0 / 6.0
        };
        *self.tick_radius += (target_tick_radius - *self.tick_radius)
            .clamp_abs(Self::ANIMATION_SPEED * delta_time as f32);
    }
    fn draw(&mut self, cx: &mut DrawContext) {
        *self.pos = Some(cx.position);
        let geng = cx.geng;
        let draw_2d = geng.draw_2d_helper();
        let position = cx.position.map(|x| x as f32);
        let line_width = position.height() / 3.0;
        let value_position = if self.range.end() == self.range.start() {
            *self.tick_radius
        } else {
            *self.tick_radius
                + ((self.value - *self.range.start()) / (*self.range.end() - *self.range.start()))
                    as f32
                    * (position.width() - line_width)
        };
        geng.draw_2d(
            cx.framebuffer,
            &PixelPerfectCamera,
            &draw_2d::Quad::new(
                AABB::from_corners(
                    position.bottom_left()
                        + vec2(value_position, (position.height() - line_width) / 2.0),
                    position.top_right()
                        - vec2(line_width / 2.0, (position.height() - line_width) / 2.0),
                ),
                cx.theme.usable_color,
            ),
        );
        draw_2d.circle(
            cx.framebuffer,
            &PixelPerfectCamera,
            position.top_right() - vec2(line_width / 2.0, position.height() / 2.0),
            line_width / 2.0,
            cx.theme.usable_color,
        );
        geng.draw_2d(
            cx.framebuffer,
            &PixelPerfectCamera,
            &draw_2d::Quad::new(
                AABB::from_corners(
                    position.bottom_left()
                        + vec2(line_width / 2.0, (position.height() - line_width) / 2.0),
                    position.bottom_left()
                        + vec2(value_position, (position.height() + line_width) / 2.0),
                ),
                cx.theme.hover_color,
            ),
        );
        geng.draw_2d(
            cx.framebuffer,
            &PixelPerfectCamera,
            &draw_2d::Ellipse::circle(
                position.bottom_left() + vec2(line_width / 2.0, position.height() / 2.0),
                line_width / 2.0,
                cx.theme.hover_color,
            ),
        );
        draw_2d.circle(
            cx.framebuffer,
            &PixelPerfectCamera,
            position.bottom_left() + vec2(value_position, position.height() / 2.0),
            *self.tick_radius * position.height(),
            cx.theme.hover_color,
        );
    }
    fn handle_event(&mut self, event: &Event) {
        let aabb = match *self.pos {
            Some(pos) => pos,
            None => return,
        };
        if self.sense.is_captured() {
            if let Event::MouseDown { position, .. } | Event::MouseMove { position, .. } = &event {
                let position = position.x - aabb.x_min;
                let new_value = *self.range.start()
                    + ((position - aabb.height() / 6.0) / (aabb.width() - aabb.height() / 3.0))
                        .clamp(0.0, 1.0)
                        * (*self.range.end() - *self.range.start());
                (self.f)(new_value);
            }
        }
    }

    fn calc_constraints(&mut self, _children: &ConstraintsContext) -> Constraints {
        Constraints::default()
    }
}
