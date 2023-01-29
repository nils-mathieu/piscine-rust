/// A color, represented by its red, green, and blue components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// The amount of red light.
    pub red: u8,
    /// The amount of green light.
    pub green: u8,
    /// The amount of blue light.
    pub blue: u8,
}

impl Color {
    /// The color white.
    pub const WHITE: Self = Self::new(0xFF, 0xFF, 0xFF);
    /// The color red.
    pub const RED: Self = Self::new(0xFF, 0x00, 0x00);
    /// The color green.
    pub const GREEN: Self = Self::new(0x00, 0xFF, 0x00);
    /// The color blue.
    pub const BLUE: Self = Self::new(0x00, 0x00, 0xFF);

    /// Creates a new [`Color`] with the specified color components.
    #[inline(always)]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    /// Computes `self` over `other`.
    ///
    /// `self` has an opacity of `alpha`.
    fn over(self, alpha: u8, other: Self) -> Self {
        #[inline(always)]
        fn over_component(a: u8, alpha: u8, b: u8) -> u8 {
            ((a as u16 * alpha as u16 + b as u16 * (255 - alpha) as u16) / 255) as u8
        }

        Self {
            red: over_component(self.red, alpha, other.red),
            green: over_component(self.green, alpha, other.green),
            blue: over_component(self.blue, alpha, other.blue),
        }
    }

    /// Computes a "distance" between `self` and `other`.
    fn distance(self, other: Self) -> u32 {
        let dr = self.red as i32 - other.red as i32;
        let dg = self.green as i32 - other.green as i32;
        let db = self.blue as i32 - other.blue as i32;
        (dr * dr + dg * dg + db * db) as u32
    }

    /// Mixes up to `max` colors extracted from `palette` to produce `self`. The closest color is
    /// returned.
    pub fn closest_mix(self, palette: &[(Self, u8)], max: u32) -> Self {
        fn best_color(target: Color, palette: &[(Color, u8)], base: Color, max: u32) -> Color {
            if max == 0 {
                return base;
            }

            let mut best_so_far = Color::WHITE;
            let mut best_distance = u32::MAX;
            for &(p, o) in palette {
                let candidate = best_color(target, palette, p.over(o, base), max - 1);
                let dist = candidate.distance(target);
                if dist <= best_distance {
                    best_distance = dist;
                    best_so_far = candidate;
                }
            }

            best_so_far
        }

        best_color(self, palette, Self::WHITE, max)
    }
}

#[cfg(test)]
#[test]
fn empty_palette() {
    assert_eq!(Color::RED.closest_mix(&[], 100), Color::WHITE);
    assert_eq!(
        Color::RED.closest_mix(
            &[(Color::RED, 255), (Color::GREEN, 255), (Color::BLUE, 255)],
            0
        ),
        Color::WHITE,
    );
}

#[cfg(test)]
#[test]
fn make_red() {
    assert_eq!(
        Color::RED.closest_mix(&[(Color::RED, 1)], 1),
        Color::new(255, 254, 254)
    );
    assert_eq!(
        Color::RED.closest_mix(&[(Color::RED, 10)], 1),
        Color::new(255, 245, 245)
    );
    assert_eq!(
        Color::RED.closest_mix(&[(Color::RED, 100)], 1),
        Color::new(255, 155, 155)
    );
}

#[cfg(test)]
#[test]
fn example() {
    let color = Color::new(254, 23, 102).closest_mix(
        &[(Color::RED, 100), (Color::GREEN, 100), (Color::BLUE, 100)],
        5,
    );
    assert_eq!(color, Color::new(218, 20, 57));
}
