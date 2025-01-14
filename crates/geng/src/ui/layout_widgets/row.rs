use super::*;

#[derive(Deref, DerefMut)]
pub struct Row<'a> {
    #[deref]
    #[deref_mut]
    children: Vec<Box<dyn Widget + 'a>>,
}

pub fn row<'a>(widgets: Vec<Box<dyn Widget + 'a>>) -> Row<'a> {
    Row { children: widgets }
}

impl<'a> Widget for Row<'a> {
    fn calc_constraints(&mut self, children: &ConstraintsContext) -> Constraints {
        Constraints {
            min_size: Vec2 {
                x: self
                    .children
                    .iter()
                    .map(|child| children.get_constraints(child.deref()).min_size.x)
                    .sum(),
                y: self
                    .children
                    .iter()
                    .map(|child| children.get_constraints(child.deref()).min_size.y)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(0.0),
            },
            flex: Vec2 {
                x: self
                    .children
                    .iter()
                    .map(|child| children.get_constraints(child.deref()).flex.x)
                    .sum(),
                y: self
                    .children
                    .iter()
                    .map(|child| children.get_constraints(child.deref()).flex.y)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(0.0),
            },
        }
    }
    fn layout_children(&mut self, cx: &mut LayoutContext) {
        let total_flex = self
            .children
            .iter()
            .map(|child| cx.get_constraints(child.deref()).flex.x)
            .sum::<f64>();
        let size_per_flex = if total_flex == 0.0 {
            0.0
        } else {
            (cx.position.width()
                - self
                    .children
                    .iter()
                    .map(|child| cx.get_constraints(child.deref()).min_size.x)
                    .sum::<f64>())
                / total_flex
        };
        let mut pos = cx.position.x_min;
        for child in &self.children {
            let child = child.deref();
            let width = cx.get_constraints(child).min_size.x
                + cx.get_constraints(child).flex.x * size_per_flex;
            cx.set_position(
                child,
                AABB::point(vec2(pos, cx.position.y_min))
                    .extend_positive(vec2(width, cx.position.height())),
            );
            pos += width;
        }
    }
    fn walk_children_mut<'b>(&mut self, mut f: Box<dyn FnMut(&mut dyn Widget) + 'b>) {
        for child in &mut self.children {
            f(child.deref_mut());
        }
    }
}

mod ext {
    use super::*;

    macro_rules! impl_for_tuple {
        ($($a:ident),*) => {
            impl<'a, $($a: Widget + 'a),*> TupleExt<'a> for ($($a,)*) {
                fn row(self) -> Row<'a> {
                    let ($($a,)*) = self;
                    row![$($a),*]
                }
            }
        };
    }
    impl_tuples!(impl_for_tuple);

    pub trait TupleExt<'a> {
        fn row(self) -> Row<'a>;
    }
}

pub use ext::TupleExt as _;
