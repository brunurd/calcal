use calcal;

fn input(prompt: &str) -> f32 {
    let mut value = String::new();
    println!("\n{}", prompt);
    std::io::stdin().read_line(&mut value).unwrap();
    return value.trim().parse::<f32>().unwrap();
}

fn main() {
    let hours = input("How many hours did you run?");
    let minutes = input("How many minutes did you run?");
    let km = input("How many kilometers?");
    let body_weight = input("What is your current body weight?");

    let pace = calcal::calculate_pace(hours, minutes, km);
    let speed = calcal::calculate_speed(hours, minutes, km);
    let cal_per_min = calcal::calculate_calories_per_minute(speed, body_weight);
    let cal = calcal::calculate_calories(hours, minutes, cal_per_min);

    println!("\nResults:\n--------");
    println!("Time: {}h{}m", hours, minutes);
    println!("Distance: {:.1$} km", km, 2);
    println!("Body weight: {:.1$} kg", body_weight, 2);
    println!("Pace: {:.1$} m/km", pace, 2);
    println!("Speed: {:.1$} km/h", speed, 2);
    println!("Calories per minute: {:.1$} cal", cal_per_min, 2);
    println!("Calories lost: {:.1$} cal", cal, 2);
}