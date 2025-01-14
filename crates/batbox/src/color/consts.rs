use super::*;

impl<T: ColorComponent> Rgba<T> {
    pub const WHITE: Self = Self {
        r: T::MAX,
        g: T::MAX,
        b: T::MAX,
        a: T::MAX,
    };
    pub const BLACK: Self = Self {
        r: T::ZERO,
        g: T::ZERO,
        b: T::ZERO,
        a: T::MAX,
    };
    pub const GRAY: Self = Self {
        r: T::HALF,
        g: T::HALF,
        b: T::HALF,
        a: T::MAX,
    };
    pub const TRANSPARENT_WHITE: Self = Self {
        r: T::MAX,
        g: T::MAX,
        b: T::MAX,
        a: T::ZERO,
    };
    pub const TRANSPARENT_BLACK: Self = Self {
        r: T::ZERO,
        g: T::ZERO,
        b: T::ZERO,
        a: T::ZERO,
    };
    pub const RED: Self = Self {
        r: T::MAX,
        g: T::ZERO,
        b: T::ZERO,
        a: T::MAX,
    };
    pub const GREEN: Self = Self {
        r: T::ZERO,
        g: T::MAX,
        b: T::ZERO,
        a: T::MAX,
    };
    pub const BLUE: Self = Self {
        r: T::ZERO,
        g: T::ZERO,
        b: T::MAX,
        a: T::MAX,
    };
    pub const CYAN: Self = Self {
        r: T::ZERO,
        g: T::MAX,
        b: T::MAX,
        a: T::MAX,
    };
    pub const MAGENTA: Self = Self {
        r: T::MAX,
        g: T::ZERO,
        b: T::MAX,
        a: T::MAX,
    };
    pub const YELLOW: Self = Self {
        r: T::MAX,
        g: T::MAX,
        b: T::ZERO,
        a: T::MAX,
    };
}

#[test]
fn test_consts_stable() {
    macro_rules! test_stable {
        ($($name:ident,)*) => {
            $(
                assert_eq!(Rgba::<f32>::$name.convert::<u8>(), Rgba::<u8>::$name);
                assert!(Rgba::<f32>::$name.convert::<f64>().approx_eq(&Rgba::<f64>::$name));
                assert_eq!(Rgba::<f64>::$name.convert::<u8>(), Rgba::<u8>::$name);
                assert!(Rgba::<f64>::$name.convert::<f32>().approx_eq(&Rgba::<f32>::$name));
                assert!(Rgba::<u8>::$name.convert::<f32>().approx_eq_eps(&Rgba::<f32>::$name, 1.0 / 255.0));
                assert!(Rgba::<u8>::$name.convert::<f64>().approx_eq_eps(&Rgba::<f64>::$name, 1.0 / 255.0));
            )*
        };
    }
    test_stable!(
        WHITE,
        BLACK,
        GRAY,
        TRANSPARENT_WHITE,
        TRANSPARENT_BLACK,
        RED,
        GREEN,
        BLUE,
        CYAN,
        MAGENTA,
        YELLOW,
    );
}
