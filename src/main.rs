extern crate rand;

use std::fs::File;
use std::io::prelude::*;
use rand::distributions::IndependentSample;

fn main() {
    let mut rng = rand::thread_rng();

    let mut file = File::create("gen.tex").unwrap();
    file.write_all(b"\\documentclass{article}\n").expect("1");
    file.write_all(b"\\usepackage[a4paper]{geometry}\n").expect("2");
    file.write_all(b"\\pagenumbering{gobble}\n").expect("3");
    file.write_all(b"\\begin{document}\n").expect("4");
    file.write_all(b"\n\\huge\n\n").expect("5");

    for i in 0..8 {
        let range = rand::distributions::Range::new(2, 13);

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
            a = rand::distributions::Range::new(2, 7).ind_sample(&mut rng);
            b = a * rand::distributions::Range::new(2, 9).ind_sample(&mut rng);
            c = rand::distributions::Range::new(3, 7).ind_sample(&mut rng);
            d = b;
            while d == b {
                d = c * rand::distributions::Range::new(2, 9).ind_sample(&mut rng);
            }
        }
        write!(file, "\\(\\frac{{{}}}{{{}}} ", a, b).expect("6");
        if i % 2 == 0 {
            write!(file, "+ ").expect("7a");            
        } else {
            write!(file, "- ").expect("7b"); 
        }
        writeln!(file, "\\frac{{{}}}{{{}}} = \\)\\\\\\\n", c, d).expect("8");
    }

    for _i in 0..5 {
        let mut d = 10;
        while d == 10 {
            d = rand::distributions::Range::new(6, 13).ind_sample(&mut rng);
        }
        let r = rand::distributions::Range::new(30, 9999).ind_sample(&mut rng);
        let n = r*d;
        writeln!(file, "\\(\\frac{{{}}}{{{}}} = \\)\\\\\\\n", n, d).expect("9");
    }

    file.write_all(b"\\end{document}\n").expect("8");
}
