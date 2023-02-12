use light_arrangements::Color;

/// Converts a vector to an array
pub fn vec_to_array<const N: usize>(vec: Vec<f64>) -> [f64; N] {
    vec.try_into().unwrap_or_else(|v: Vec<f64>| {
        panic!("Expected a Vec of length {} but it was {}", N, v.len())
    })
}

/// Converts the first 3 elements of a vector to a Color
pub fn vec_to_color(vec: Vec<u8>) -> Color {
    Color {
        red: vec[0],
        green: vec[1],
        blue: vec[2],
    }
}

pub fn color_to_tuple(color_opt: Option<Color>) -> Option<(u8, u8, u8)> {
    color_opt.map_or(None, |c| Some((c.red, c.green, c.blue)))
}
