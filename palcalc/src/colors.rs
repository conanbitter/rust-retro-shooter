#[derive(Debug, Clone, Copy)]
struct FloatColor {
    r: f64,
    g: f64,
    b: f64,
}

impl FloatColor {
    fn distance(&self, other: FloatColor) -> f64 {
        let dr = self.r - other.r;
        let dg = self.g - other.g;
        let db = self.b - other.b;

        return (dr * dr + dg * dg + db * db).sqrt();
    }

    fn new(r: i32, g: i32, b: i32) -> FloatColor {
        return FloatColor {
            r: (r as f64) / 255.0,
            g: (g as f64) / 255.0,
            b: (b as f64) / 255.0,
        };
    }

    const BLACK: FloatColor = FloatColor { r: 0.0, g: 0.0, b: 0.0 };
}
