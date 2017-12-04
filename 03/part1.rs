const INPUT: f64 = 361527.0;

fn main () {
    let mut square = INPUT.sqrt().ceil();
    if square % 2.0 == 0.0 {
        square += 1.0;
    }

    let side = (INPUT - ((square - 2.0).powi(2))) / (square - 1.0);
    let position = INPUT - (square - 2.0).powi(2);
    let mid = ((square + 1.0) / 2.0) - 1.0;

    let mut distance = 0.0;

    distance += mid;
    match side.floor() as u32 {
        0 => {
            distance += (position - mid).abs();
        },
        1 => {
            distance += (position - (mid + (1.0 * (square - 1.0)))).abs();
        },
        2 => {
            distance += (position - (mid + (2.0 * (square - 1.0)))).abs();
        },
        3 => {
            distance += (position - (mid + (3.0 * (square - 1.0)))).abs();
        },
        _ => ()
    }

    println!("Distance: {}", distance)
}
