extern crate rand;

use std::fs::File;
use std::io::prelude::*;
use rand::distributions::IndependentSample;

fn main() {
    let range = rand::distributions::Range::new(2, 11);
    let mut rng = rand::thread_rng();

    let mut file = File::create("gen.tex").unwrap();
    file.write_all(b"\\documentclass{article}\n").expect("1");
    file.write_all(b"\\usepackage[a4paper]{geometry}\n").expect("2");
    file.write_all(b"\\begin{document}\n").expect("2");
    file.write_all(b"\n\\huge\n\n").expect("2");

    for i in 0..13 {
        let mut a = range.ind_sample(&mut rng);
        let mut b = a;
        while b == a {
            b = range.ind_sample(&mut rng);
        }
        let mut c = range.ind_sample(&mut rng);
        let mut d = c;
        while d == c || d == b {
            d = range.ind_sample(&mut rng);
        }
        if i % 2 == 1 {
            a = rand::distributions::Range::new(2, 5).ind_sample(&mut rng);
            b = a * rand::distributions::Range::new(2, 6).ind_sample(&mut rng);
            c = rand::distributions::Range::new(3, 5).ind_sample(&mut rng);
            d = b;
            while d == b {
                d = c * rand::distributions::Range::new(2, 6).ind_sample(&mut rng);
            }
        }
        write!(file, "\\(\\frac{{{}}}{{{}}} + ", a, b).expect("3");
        writeln!(file, "\\frac{{{}}}{{{}}} = \\)\\\\\\\n", c, d).expect("3");
    }

    file.write_all(b"\\end{document}\n").expect("6");
}
