fn main() {
    let g = 9.81_f64;
    let launch_speed = 20.0_f64;
    let launch_angle_deg = 45.0_f64;
    let launch_angle = launch_angle_deg.to_radians();

    let vx = launch_speed * launch_angle.cos();
    let vy = launch_speed * launch_angle.sin();

    let total_time = 2.0 * vy / g;
    let range = vx * total_time;
    let max_height = (vy * vy) / (2.0 * g);

    println!("Projectile motion summary");
    println!("launch speed: {launch_speed:.2} m/s");
    println!("launch angle: {launch_angle_deg:.2} deg");
    println!("flight time: {total_time:.2} s");
    println!("range: {range:.2} m");
    println!("max height: {max_height:.2} m");
}
