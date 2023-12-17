pub fn puzzle1(input: &str) {
    let input = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|element| element.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut acum = 0;
    let mut acum2 = 0;
    for seq in input {
        let mut extr = Vec::new();
        extr.push(seq);
        loop {
            let mut sub = Vec::new();
            let last = extr.last().unwrap();
            for i in 1..last.len() {
                sub.push(last[i] - last[i - 1]);
            }
            if sub.iter().fold(true, |a, b| a && *b == 0) {
                break;
            }
            extr.push(sub);
        }
        let result = extr.iter().rev().fold(0, |v, sub| sub.last().unwrap() + v);
        let result2 = extr.iter().rev().fold(0, |v, sub| sub.first().unwrap() - v);
        //println!("{:?}", extr.last().unwrap());
        //println!("{result}");
        //println!("{result2}");
        acum += result;
        acum2 += result2;
    }

    println!("{acum}");
    println!("{acum2}");
}
