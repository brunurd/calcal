#[no_mangle]
pub extern fn calculate_pace(hours: f32, minutes: f32, km: f32) -> f32 {
    let time = hours * 60.0 + minutes;
    return time / km;
}

#[no_mangle]
pub extern fn calculate_speed(hours: f32, minutes: f32, km: f32) -> f32 {
    let time = hours + minutes / 60.0;
    return km/ time;
}

#[no_mangle]
pub extern fn calculate_calories_per_minute(speed: f32, body_weight: f32) -> f32 {
    return  speed * body_weight * 0.0175;
}

#[no_mangle]
pub extern fn calculate_calories(hours: f32, minutes: f32, calories_per_minute: f32) -> f32 {
    let time = hours * 60.0 + minutes;
    return time * calories_per_minute;
}