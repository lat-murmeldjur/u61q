use std::f32::consts::PI;
use std::time::SystemTime;

pub trait Groupable {
    fn group_with_nothing(&self) -> String;
}

impl Groupable for u128 {
    fn group_with_nothing(&self) -> String {
        self.to_string() // lol
            .as_bytes() // this is
            .rchunks(3) // how
            .rev() // we
            .map(std::str::from_utf8) // format large numbers
            .collect::<Result<Vec<&str>, _>>() // to visually readable formats
            .unwrap() // in rust
            .join(" ") // and nobody minds this
    }
}

pub fn record_nanos() -> u128 {
    return SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
}

pub fn oclock() -> f32 {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    return (((now / 60 / 60) % 24) as f32) * PI / 24.0 - PI / 2.0;
}
