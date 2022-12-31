pub fn get_total(values: impl Iterator<Item = i32>) -> i32 {
    return values.reduce(|accum, value| accum + value).unwrap();
}
