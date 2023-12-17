pub fn puzzle1() {
    let mut input = Vec::new();
    //
    // input.push((7.0, 9.0));
    // input.push((15.0, 40.0));
    // input.push((30.0, 200.0));
    //
    // input.push((42.0, 284.0));
    // input.push((68.0, 1005.0));
    // input.push((69.0, 1122.0));
    // input.push((85.0, 1314.0));
    //
    //input.push((71530.0, 940200.0));
    //
    input.push((42686985.0, 284100511221314.0));
    //
    let input = input;
    let mut result = 1;
    for (time, distance) in input {
        let det: f64 = time * time - 4.0 * distance;
        let solutions = if det > 0.0 {
            let sq = det.sqrt();
            let min: f64 = (time - sq) / 2.0;
            let max: f64 = (time + sq) / 2.0;
            let min = (if min == min.ceil() {
                min.ceil() + 1.0
            } else {
                min.ceil()
            })
            .clamp(0.0, time) as i32;
            let max = (if max == max.floor() {
                max.floor() - 1.0
            } else {
                max.floor()
            })
            .clamp(0.0, time) as i32;

            max - min + 1
        } else {
            0
        };
        result *= solutions;
    }

    println!("{result}");
}
