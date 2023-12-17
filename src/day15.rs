pub fn puzzle1(input: &str) {
    let answer = input.split(",").fold(0, |acum, txt| {
        acum + txt
            .chars()
            .fold(0, |acum, c| ((acum + c as i32) * 17) % 256)
    });
    println!("{answer}");
}

pub fn puzzle2(input: &str) {
    let mut boxes: Vec<Vec<(&str, i32)>> = Vec::with_capacity(256);
    boxes.resize(256, Vec::new());
    'instruction: for txt in input.split(",") {
        let compute_hash = |idx| {
            txt[0..idx]
                .chars()
                .fold(0, |acum, c| ((acum + c as usize) * 17) % 256)
        };
        if let Some(idx) = txt.find('=') {
            let hash = compute_hash(idx);
            let lens = txt[idx + 1..].parse().unwrap();

            for i in 0..boxes[hash].len() {
                if *boxes[hash][i].0 == txt[0..idx] {
                    boxes[hash][i].1 = lens;
                    continue 'instruction;
                }
            }
            boxes[hash].push((&txt[0..idx], lens));
        } else if let Some(idx) = txt.find('-') {
            let hash = compute_hash(idx);
            let mut i = 0;
            loop {
                if i == boxes[hash].len() {
                    break;
                } else if boxes[hash][i].0 == &txt[0..idx] {
                    boxes[hash].remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }

    let answer = boxes.iter().enumerate().fold(0, |acum, (box_id, bx)| {
        acum + bx.iter().enumerate().fold(0, |acum, (slot, (_, focal))| {
            acum + (box_id + 1) * (slot + 1) * *focal as usize
        })
    });

    println!("{answer}");
}
