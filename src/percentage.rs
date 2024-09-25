pub fn from_total_and_value(total: i32, value: i32) ->  i32 {
    (value as f32 / total as f32 * 100.0) as i32
}

pub fn to_value(total: i32, percentage: i8) -> i32 {
    let value = total as f32 / 100.0 * percentage as f32;
    value as i32
}
