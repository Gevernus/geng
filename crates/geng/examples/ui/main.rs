use geng::prelude::*;

mod custom;
mod slider;
mod vec2slider;

use custom::CustomWidget;
use vec2slider::Vec2Slider;

#[derive(geng::Assets)]
pub struct Assets {
    texture: ugli::Texture,
    shader: ugli::Program,
}

struct State {
    geng: Geng,
    assets: Assets,
    counter: i32,
    vec2: Vec2<f32>,
}

impl State {
    fn new(geng: &Geng, assets: Assets) -> Self {
        Self {
            geng: geng.clone(),
            assets,
            counter: 0,
            vec2: Vec2::ZERO,
        }
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
    }
    fn ui<'a>(&'a mut self, cx: &'a geng::ui::Controller) -> Box<dyn geng::ui::Widget + 'a> {
        use geng::ui::*;

        let title = "Counter Example";

        let counter = {
            let row1 = {
                let minus_button = geng::ui::Button::new(cx, "-");
                let plus_button = geng::ui::Button::new(cx, "+");
                if minus_button.was_clicked() {
                    self.counter -= 1;
                }
                if plus_button.was_clicked() {
                    self.counter += 1;
                }
                let current_value = self.counter.to_string();
                (
                    minus_button.center(),
                    current_value
                        .padding_horizontal(cx.theme().text_size as f64)
                        .center(),
                    plus_button.center(),
                )
                    .row() // TODO: .row_every_widget_centered()
            };
            let row2 = {
                let slider =
                    slider::Slider::new(cx, (self.counter as f64).clamp(0.0, 100.0), 0.0..=100.0);
                if let Some(change) = slider.get_change() {
                    self.counter = change as _;
                }
                slider.fixed_size(vec2(5.0, 1.0) * cx.theme().text_size as f64)
            };
            (
                row1.center()
                    .padding_bottom(cx.theme().text_size as f64 * 0.3),
                row2.center(),
            )
                .column() // TODO: column_centered()
        };

        let custom_widget = CustomWidget::new(
            cx,
            &self.assets,
            (self.counter as f32 / 100.0).clamp(0.0, 1.0),
        );
        if let Some(change) = custom_widget.get_change() {
            self.counter = (change * 100.0) as _;
        }

        let vec2 = {
            let text = format!("({:.2}, {:.2})", self.vec2.x, self.vec2.y);
            let control = Vec2Slider::new(cx, self.vec2);
            if let Some(change) = control.get_change() {
                self.vec2 = change;
            }
            (text.center(), control.center()).column()
        };

        let window = (
            title
                .center()
                .uniform_padding(cx.theme().text_size as f64 * 0.3),
            ColorBox::divider(cx.theme().text_color, 1.0),
            counter
                .center()
                .uniform_padding(cx.theme().text_size as f64 * 0.3),
            custom_widget
                .center()
                .uniform_padding(cx.theme().text_size as f64 * 0.3),
            vec2.center()
                .uniform_padding(cx.theme().text_size as f64 * 0.3),
        )
            .column()
            .background_color(Rgba::new(0.1, 0.1, 0.1, 1.0))
            .center();

        // let ui = geng::ui::stack![custom_widget, window];
        let ui = window;
        ui.boxed()
    }
}

fn main() {
    logger::init().unwrap();
    let geng = Geng::new("Geng UI Demo!");
    geng::run(
        &geng,
        geng::LoadingScreen::new(
            &geng,
            geng::EmptyLoadingScreen,
            <Assets as geng::LoadAsset>::load(&geng, &static_path()),
            {
                let geng = geng.clone();
                move |assets| {
                    let assets = assets.unwrap();
                    State::new(&geng, assets)
                }
            },
        ),
    );
}
