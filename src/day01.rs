pub fn puzzle1(input: &str) {
    let mut acum = 0;

    for line in input.lines() {
        let mut first = None;
        let mut last = None;
        for c in line.chars() {
            if c.is_numeric() {
                let n = c as i32 - '0' as i32;
                if first == None {
                    first = Some(n);
                }
                last = Some(n);
            }
        }
        if let (Some(first), Some(last)) = (first, last) {
            acum = acum + first * 10 + last;
        }
    }
    println!("{}", acum);
}

pub fn search_first_and_last(line: &str, n: &str) -> Option<(usize, usize)> {
    if let Some(first) = line.find(n) {
        match line.rfind(n) {
            Some(last) => Some((first, last)),
            None => None,
        }
    } else {
        None
    }
}
pub fn search_number(line: &str, n: i32) -> Option<(usize, usize)> {
    let txt = match n {
        1 => Some("one"),
        2 => Some("two"),
        3 => Some("three"),
        4 => Some("four"),
        5 => Some("five"),
        6 => Some("six"),
        7 => Some("seven"),
        8 => Some("eight"),
        9 => Some("nine"),
        _ => None,
    };
    let n = match n {
        0 => Some("0"),
        1 => Some("1"),
        2 => Some("2"),
        3 => Some("3"),
        4 => Some("4"),
        5 => Some("5"),
        6 => Some("6"),
        7 => Some("7"),
        8 => Some("8"),
        9 => Some("9"),
        _ => None,
    };
    let idx1 = match n {
        Some(n) => search_first_and_last(line, &n),
        None => None,
    };
    let idx2 = match txt {
        Some(txt) => search_first_and_last(line, &txt),
        None => None,
    };

    match (idx1, idx2) {
        (Some((first1, last1)), Some((first2, last2))) => Some((
            if first1 < first2 { first1 } else { first2 },
            if last1 > last2 { last1 } else { last2 },
        )),
        (Some(idx1), None) => Some(idx1),
        (_, idx2) => idx2,
    }
}

pub fn puzzle2(input: &str) {
    let mut acum = 0;

    for line in input.lines() {
        let mut first = 0;
        let mut last = 0;

        let mut idxs = None;

        for n in 0..10 {
            if let Some((f, l)) = search_number(line, n) {
                if let Some((idxf, idxl)) = idxs {
                    idxs = Some((
                        if f < idxf {
                            first = n;
                            f
                        } else {
                            idxf
                        },
                        if l > idxl {
                            last = n;
                            l
                        } else {
                            idxl
                        },
                    ))
                } else {
                    first = n;
                    last = n;
                    idxs = Some((f, l));
                }
            }
        }

        println!("{line}: {first}{last}");

        acum = acum + first * 10 + last;
    }
    println!("{}", acum);
}
