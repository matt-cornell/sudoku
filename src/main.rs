use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus as RandGen;
use clap::*;

fn calc<R: Rng>(rng: &mut R) -> usize {
    let mut state = [0u8; 81];
    for i in 0..81 {
        let mut valid = [true; 9];
        let col = i % 9;
        for j in (i - col)..i {
            valid[(state[j] - 1) as usize] = false;
        }
        for j in (col..i).step_by(9) {
            valid[(state[j] - 1) as usize] = false;
        }
        let start = i / 27 * 27 + col / 3 * 3;
        'square: for j in 0..3 {
            for k in 0..3 {
                let l = start + j * 9 + k;
                if l >= i { break 'square }
                valid[(state[l] - 1) as usize] = false;
            }
        }
        let mut valid_buf = [0u8; 9];
        let mut valid_len = 0;
        for (n, v) in valid.iter().enumerate() {
            if *v {
                valid_buf[valid_len] = n as u8;
                valid_len += 1;
            }
        }
        let Some(digit) = valid_buf[..valid_len].choose(rng) else {
            return i;
        };
        state[i] = *digit + 1;
    }
    81
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    seed: Option<String>,
    #[arg(short, long)]
    trials: Option<usize>,
    #[arg(short, long)]
    percents: bool,
}

fn main() {
    let Cli { seed, trials, percents } = Cli::parse();
    let seed = seed.map_or(rand::random(), |s| hmac_sha256::Hash::hash(s.as_bytes()));
    println!("seed: {}", hex::encode(seed));
    let mut rng = RandGen::from_seed(seed);
    let mut outcomes = [0; 82];
    let trials = trials.unwrap_or(1000000);
    for n in 0..trials {
        print!("running: {n}/{trials}\r");
        let res = calc(&mut rng);
        outcomes[res] += 1;
    }
    let avg = outcomes.iter().enumerate().map(|(s, &n)| s * n).sum::<usize>() as f64 / trials as f64;
    let w = if percents { 6 } else { outcomes.iter().max().unwrap().to_string().len() };
    println!("average {avg}");
    println!("-------------");
    for row in outcomes.chunks(9) {
        for num in row {
            if percents {
                print!("{:>6.3} ", *num as f64 / trials as f64 * 100.0);
            } else {
                print!("{num:>w$} ");
            }
        }
        println!();
    }
}
