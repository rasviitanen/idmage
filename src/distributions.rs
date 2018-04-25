pub fn log_spiral_spray() -> Vec<(f64, f64)> {
    let mut coordinates = Vec::new();
    for t in 1..200 {
        coordinates.push(polar((t*2) as f64/1.2));
    }
    coordinates
}

fn polar(t: f64) -> (f64, f64) {
    (t.cos()*(t*0.02).exp(), t.sin()*(t*0.02).exp())
}
