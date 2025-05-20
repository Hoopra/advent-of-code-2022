pub fn split_string(s: &str) -> (&str, &str) {
    let half = ((s.len() as f32) / 2.0).round() as usize;

    s.split_at(half)
}
