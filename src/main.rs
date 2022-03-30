static GYONGY: [char; 3] = ['F', 'P', 'Z']; // F: fehér, P: piros, Z: zöld gyöngyök

fn permut(n: usize, gyongyok: &mut Vec<String>) {

    if n < 13 {

        for i in 0..gyongyok.len() {

            for _ in 0..2 {
                gyongyok.insert(i*3, gyongyok[i*3].clone());
            }
            for j in 0..3 {
                gyongyok[i * 3 + j].push(GYONGY[j]);
            }

        }

        permut(n + 1, gyongyok);

    }

}

fn main() {

    let mut gyongyok: Vec<String> = GYONGY.iter().map(|x| x.to_string()).collect();

    permut(1, &mut gyongyok);

    // kidobáljuk azokat a gyöngysorokat ahol nem jó számban vannak a gyöngyök
    let gyongyok: usize = gyongyok.into_iter()
        .filter(|g| {
            g.chars().filter(|&c| c == 'F').count() == 6 && g.chars().filter(|&c| c == 'P').count() == 5
        })
        // kidobáljuk azokat a gyöngysorokat ahol piros van zöld mellett
        .filter(|g| {
            let mut g: Vec<char> = g.chars().collect();

            g.insert(0, 'X');
            g.push('X');

            let mut f: bool = true;

            for i in 1..g.len() - 1 {

                if g[i] == 'Z' {
                    if g[i - 1] == 'P' || g[i + 1] == 'P' {
                        f = false;
                        break;
                    }

                }

            }
            f
        })
        .count();

        println!("{}", gyongyok);

}
