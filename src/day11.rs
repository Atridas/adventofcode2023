use std::collections::HashSet;

pub fn puzzle1(input: &str) {
    let galaxies =
        input
            .lines()
            .enumerate()
            .fold(Vec::new(), |last: Vec<(usize, usize)>, (y, line)| {
                let mut v = line
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(|(x, _)| (x, y))
                    .collect::<Vec<_>>();
                v.extend(last);
                v
            });
    let mut max = (0, 0);
    let mut used_rows = HashSet::new();
    let mut used_columns = HashSet::new();
    for galaxy in &galaxies {
        if galaxy.0 > max.0 {
            max.0 = galaxy.0;
        }
        if galaxy.1 > max.1 {
            max.1 = galaxy.1;
        }
        used_rows.insert(galaxy.0);
        used_columns.insert(galaxy.1);
    }
    let mut row_expansion = Vec::new();
    let mut column_expansion = Vec::new();
    let mut current_row_expansion = 0;
    let mut current_column_expansion = 0;
    for x in 0..=max.0 {
        if !used_rows.contains(&x) {
            current_row_expansion += 999999;
        }
        row_expansion.push(current_row_expansion);
    }
    for y in 0..=max.1 {
        if !used_columns.contains(&y) {
            current_column_expansion += 999999;
        }
        column_expansion.push(current_column_expansion);
    }

    let mut accum = 0;
    for i in 0..galaxies.len() {
        let galaxy1 = &galaxies[i];
        for j in i + 1..galaxies.len() {
            let galaxy2 = &galaxies[j];

            let min = (galaxy1.0.min(galaxy2.0), galaxy1.1.min(galaxy2.1));
            let max = (galaxy1.0.max(galaxy2.0), galaxy1.1.max(galaxy2.1));

            let dx = max.0 - min.0 + row_expansion[max.0] - row_expansion[min.0];
            let dy = max.1 - min.1 + column_expansion[max.1] - column_expansion[min.1];

            accum += dx + dy;
        }
    }

    println!("{accum}");
}
