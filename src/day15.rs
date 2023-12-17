pub fn puzzle1(input: &str) {
    let answer = input.split(",").fold(0, |acum, txt| {
        acum + txt
            .chars()
            .fold(0, |acum, c| ((acum + c as i32) * 17) % 256)
    });
    println!("{answer}");
}
