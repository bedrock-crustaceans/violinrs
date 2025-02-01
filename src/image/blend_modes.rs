#[derive(Clone)]
pub enum BlendMode {
    Overlay,
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

pub fn overlay_blend_mode(a: f64, b: f64, b_a: f64) -> f64 {
    let result = if a < 0.5 {
        2.0 * a * b
    } else {
        1.0 - 2.0 * (1.0 - a) * (1.0 - b)
    };

    println!("a: {}, b: {}, res: {}", a, b, lerp(a, result, b_a));

    lerp(a, result, b_a)
}
