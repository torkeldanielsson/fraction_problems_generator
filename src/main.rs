extern crate rand;

use std::fs::File;
use std::io::prelude::*;
use rand::distributions::IndependentSample;
use rand::Rng;

fn main() {
/*
    let decimal_add_sub = 4;
    let power = 3;
    let multiplication = 3;
    let square_root = 2;
    let integer_equation = 5;
    let fraction_add_sub = 2;
    let fraction_multiplication = 4;
    let division = 3;
*/

    let decimal_add_sub = 10;
    let power = 0;
    let multiplication = 0;
    let square_root = 0;
    let integer_equation = 0;
    let fraction_add_sub = 0;
    let fraction_multiplication = 10;
    let division = 0;


    let mut rng = rand::thread_rng();

    let mut file = File::create("gen.tex").unwrap();
    file.write_all(b"\\documentclass{article}\n").expect("1");
    file.write_all(b"\\usepackage[a4paper]{geometry}\n").expect("2");
    file.write_all(b"\\pagenumbering{gobble}\n").expect("3");
    file.write_all(b"\\begin{document}\n").expect("4");
    file.write_all(b"\n\\huge\n\n").expect("5");

    let mut problems = Vec::new();

    for _i in 0..decimal_add_sub {
        let a_ints = rand::distributions::Range::new(1, 2).ind_sample(&mut rng);
        let a_decs = rand::distributions::Range::new(1, 2).ind_sample(&mut rng);
        let b_ints = rand::distributions::Range::new(1, 2).ind_sample(&mut rng);
        let b_decs = rand::distributions::Range::new(1, 2).ind_sample(&mut rng);

        let mut problem = format!("\\(");

        for j in 0..a_ints {
            let mut rnd_start = 0;
            if a_ints != 1 && j == 0 {
                rnd_start = 1;
            }
            problem = format!("{}{}", problem, rand::distributions::Range::new(rnd_start, 10).ind_sample(&mut rng));
        }
        problem = format!("{}.", problem);
        for _j in 0..a_decs {
            problem = format!("{}{}", problem, rand::distributions::Range::new(0, 10).ind_sample(&mut rng));
        }
        problem = format!("{} + ", problem);

        for j in 0..b_ints {
            let mut rnd_start = 0;
            if b_ints != 1 && j == 0 {
                rnd_start = 1;
            }
            problem = format!("{}{}", problem, rand::distributions::Range::new(rnd_start, 10).ind_sample(&mut rng));
        }
        problem = format!("{}.", problem);
        for _j in 0..b_decs {
            problem = format!("{}{}", problem, rand::distributions::Range::new(0, 10).ind_sample(&mut rng));
        }

        problem = format!("{} =\\)\\\\\\\n\n", problem);
        problems.push(problem);
    }

    for _i in 0..power {
        let mut d : u32 = 12;
        let mut e : u32 = 5;
        while d.pow(e) > 200 {
            d = rand::distributions::Range::new(1, 12).ind_sample(&mut rng);
            e = rand::distributions::Range::new(2, 5).ind_sample(&mut rng);
        }
        let problem = format!("\\({}^{} =\\)\\\\\\\n\n", d, e);
        problems.push(problem);
    }

    for i in 0..multiplication {
        let mut a: i32 = 0;
        while a.abs() < 3 || a.abs() == 10 {
            a = rand::distributions::Range::new(-13, 13).ind_sample(&mut rng);
        }
        let mut b = 0;
        while b == 0 || b == a {
            b = rand::distributions::Range::new(-99, 99).ind_sample(&mut rng);
        }
        if i % 2 == 0 {
            let tmp = a;
            a = b;
            b = tmp;
        }
        let mut problem = format!("\\(");
        if a < 0 {
            problem = format!("{}\\left({}\\right)", problem, a);
        } else {
            problem = format!("{}{}", problem, a);
        }
        if a > 0 && b > 0 {
            problem = format!("{}\\cdot", problem);
        }
        if b < 0 {
            problem = format!("{}\\left({}\\right)", problem, b);
        } else {
            problem = format!("{}{}", problem, b);
        }
        problem = format!("{}=\\)\\\\\\\n\n", problem);
        problems.push(problem);
    }

    for _i in 0..square_root {
        let d = rand::distributions::Range::new(1, 12).ind_sample(&mut rng);
        let problem = format!("\\(\\sqrt{{{}}} = \\)\\\\\\\n\n", d*d);
        problems.push(problem);
    }

    for i in 0..integer_equation {
        let mut a = 0;
        while a == 0 {
            a = rand::distributions::Range::new(-9, 9).ind_sample(&mut rng);
        }

        let mut b = 0;
        while b == 0 || b == a {
            b = rand::distributions::Range::new(-9, 9).ind_sample(&mut rng);
        }

        let chars = vec!["x", "y", "z", "\\alpha", "\\beta", "\\gamma"];
        let c = rng.choose(&chars).expect("");

        let problem;

        let mut sign = "+";
        if rand::distributions::Range::new(0, 2).ind_sample(&mut rng) == 0 {
            sign = "-";
        }

        if i % 2 == 1 {
            problem = format!("\\({} {} {} = {} \\quad \\rightarrow \\quad {} =\\)\\\\\\\n\n", a, sign, c, b, c);

        } else {
            if a < 0 {
                sign = "";
            }
            problem = format!("\\({} {} {} = {} \\quad \\rightarrow \\quad {} =\\)\\\\\\\n\n", c, sign, a, b, c);
        }
        problems.push(problem);
    }

    for i in 0..fraction_add_sub {
        let mut a = rand::distributions::Range::new(2, 13).ind_sample(&mut rng);
        let mut b = a;
        while b == a {
            b = rand::distributions::Range::new(2, 13).ind_sample(&mut rng);
        }
        let mut c = rand::distributions::Range::new(2, 13).ind_sample(&mut rng);
        let mut d = c;
        while d == c || d == b {
            d = rand::distributions::Range::new(2, 13).ind_sample(&mut rng);
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
        let mut problem: String;
        if rand::distributions::Range::new(0, 4).ind_sample(&mut rng) == 0 {
            problem = format!("\\( - \\dfrac{{{}}}{{{}}}", a, b);
        } else {
            problem = format!("\\(\\dfrac{{{}}}{{{}}}", a, b);
        }
        if i % 2 == 0 {
            problem = format!("{} + ", problem);            
        } else {
            problem = format!("{} - ", problem);            
        }
        problem = format!("{}\\dfrac{{{}}}{{{}}} = \\)\\\\\\\n\n", problem, c, d);
        problems.push(problem);
    }

    for _i in 0..fraction_multiplication {
        let a = rand::distributions::Range::new(2, 10).ind_sample(&mut rng);
        let mut b = a;
        while b == a {
            b = rand::distributions::Range::new(2, 10).ind_sample(&mut rng);
        }
        let mut c = a;
        while c == a || c == b {
            c = rand::distributions::Range::new(2, 10).ind_sample(&mut rng);
        }
        let mut d = c;
        while d == a || d == b || d == c {
            d = rand::distributions::Range::new(2, 10).ind_sample(&mut rng);
        }

        problems.push(format!("\\(\\dfrac{{{}}}{{{}}} \\cdot \\dfrac{{{}}}{{{}}} = \\)\\\\\\\n\n", a, b, c, d));
    }

    for i in 0..division {
        let mut d = 10;
        while d == 10 {
            d = rand::distributions::Range::new(3, 13).ind_sample(&mut rng);
        }
        let r = rand::distributions::Range::new(12, 51).ind_sample(&mut rng);
        let n = r * d;
        let problem;
        if i % 2 == 1 {
            problem = format!("\\(\\dfrac{{{}}}{{{}}} = \\)\\\\\\\n\n", n, d);
        } else {
            problem = format!("\\({} / {} = \\)\\\\\\\n\n", n, d);
        }
        problems.push(problem);
    }

    let slice: &mut [String] = problems.as_mut_slice();
    rng.shuffle(slice);

    for problem in slice {
        file.write_all(problem.as_bytes()).expect("6");
    }

    file.write_all(b"\\end{document}\n").expect("7");
}
