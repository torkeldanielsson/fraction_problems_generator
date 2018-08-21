extern crate rand;

use std::fs::File;
use std::io::prelude::*;
use rand::distributions::IndependentSample;
use rand::Rng;

fn main() {


    let decimal_add_sub = 4;
    let power = 3;
    let multiplication = 3;
    let square_root = 2;
    let integer_equation = 3;
    let fraction_add_sub = 2;
    let fraction_multiplication = 2;
    let division = 3;
    let fraction_equation = 2;
    let unit_conversion = 2;
    let percentage1 = 0;
    let percentage2 = 0;
    let percentage3 = 2;
    let percentage4 = 2;
    let parenthesis_1 = 2;
    let parenthesis_2 = 0;
    let trig_1 = 7;


/*
    let decimal_add_sub = 0;
    let power = 0;
    let multiplication = 0;
    let square_root = 0;
    let integer_equation = 0;
    let fraction_add_sub = 0;
    let fraction_multiplication = 0;
    let fraction_equation = 0;
    let division = 0;
    let unit_conversion = 0;
    let percentage1 = 0;
    let percentage2 = 0;
    let percentage3 = 0;
    let percentage4 = 0;
    let parenthesis_1 = 0;
    let parenthesis_2 = 0;
    let trig_1 = 10;
*/

    let mut rng = rand::thread_rng();

    let mut file = File::create("gen.tex").unwrap();
    file.write_all(
b"\\documentclass{article}
\\usepackage[utf8]{inputenc}
\\usepackage[a4paper]{geometry}
\\usepackage{lmodern}
\\usepackage[T1]{fontenc}
\\usepackage{graphicx}
\\usepackage{tikz}
\\usepackage{textcomp}
\\usepackage{amsmath}
\\usetikzlibrary{angles,quotes}
\\pagenumbering{gobble}
\\begin{document}").expect("1");


    file.write_all(b"\n\\huge\n\n").expect("5");

    let mut problems = Vec::new();

    for _i in 0..decimal_add_sub {
        let a_ints = rand::distributions::Range::new(1, 4).ind_sample(&mut rng);
        let a_decs = rand::distributions::Range::new(1, 4).ind_sample(&mut rng);
        let b_ints = rand::distributions::Range::new(1, 4).ind_sample(&mut rng);
        let b_decs = rand::distributions::Range::new(1, 4).ind_sample(&mut rng);

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
            d = rand::distributions::Range::new(2, 20).ind_sample(&mut rng);
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

        let chars = vec!["x", "y", "z", "\\alpha", "\\beta", "\\gamma", "\\Omega", "\\theta", "\\varphi"];
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

    for _i in 0..fraction_equation {
        let mut digits: Vec<i32> = (2..10).collect();
        let digit_slice: &mut [i32] = digits.as_mut_slice();
        rng.shuffle(digit_slice);
        
        let mut chosen_digits: Vec<String> = Vec::new();
        for j in 0..3 {
            chosen_digits.push(format!("{}", digit_slice[j]));
        }

        let chars = vec!["x", "y", "z", "\\alpha", "\\beta", "\\gamma", "\\Omega", "\\theta", "\\varphi"];
        let c = rng.choose(&chars).expect("");
        chosen_digits.push(format!("{}", c));

        let chosen_digits_slice: &mut [String] = chosen_digits.as_mut_slice();
        rng.shuffle(chosen_digits_slice);

        problems.push(format!("\\(\\dfrac{{{}}}{{{}}} = \\dfrac{{{}}}{{{}}}  \\quad \\rightarrow \\quad {} =\\)\\\\\\\n\n", 
            chosen_digits_slice[0],
            chosen_digits_slice[1],
            chosen_digits_slice[2],
            chosen_digits_slice[3],
            c));
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

    for _i in 0..unit_conversion {
        let num_a = rand::distributions::Range::new(0, 5).ind_sample(&mut rng);
        let num_b = rand::distributions::Range::new(0, 4).ind_sample(&mut rng);

        let mut a = format!("{}", rand::distributions::Range::new(1, 10).ind_sample(&mut rng));

        for _j in 0..num_a {
            a = format!("{}{}", a, rand::distributions::Range::new(0, 10).ind_sample(&mut rng));
        }

        a = format!("{}.", a);

        for _j in 0..num_b {
            a = format!("{}{}", a, rand::distributions::Range::new(0, 10).ind_sample(&mut rng));
        }

        a = format!("{}{}", a, rand::distributions::Range::new(1, 10).ind_sample(&mut rng));

        let mut units = vec![/*"\\mu m", */"mm", "cm", "dm", "m", "km"];

        if 0 == rand::distributions::Range::new(0, 2).ind_sample(&mut rng) {
            units = vec![/*"\\mu m", */"mg", "g", "hg", "kg", "ton"];
        }

        let i = rand::distributions::Range::new(0, units.len()).ind_sample(&mut rng);
        let mut j = i;
        while i == j {
            let min_val = std::cmp::max(i as i32 - 2, 0) as usize;
            let max_val = std::cmp::min(i + 2, units.len());
            j = rand::distributions::Range::new(min_val, max_val).ind_sample(&mut rng);
        }

        let problem = format!("\\( {}\\ {} = \\quad \\quad \\quad \\quad  \\quad \\quad {} \\)\\\\\\\n\n", a, units[i], units[j]);
        problems.push(problem);
    }

    for _i in 0..percentage1 {
        let a = rand::distributions::Range::new(1, 100).ind_sample(&mut rng);
        
        let problem = format!("\\( {}\\% \\textrm{{ av }} 100 =\\)\\\\\\\n\n", a);
        problems.push(problem);
    }

    for _i in 0..percentage2 {
        let a = rand::distributions::Range::new(1, 10).ind_sample(&mut rng);
        
        let problem = format!("\\( {}\\% \\textrm{{ av }} 10 =\\)\\\\\\\n\n", 10*a);
        problems.push(problem);
    }

    for _i in 0..percentage3 {
        let a = rand::distributions::Range::new(1, 11).ind_sample(&mut rng);
        let b = rand::distributions::Range::new(1, 11).ind_sample(&mut rng);
        
        let problem = format!("\\( {}\\% \\textrm{{ av }} {} =\\)\\\\\\\n\n", 10*a, 10*b);
        problems.push(problem);
    }

    for _i in 0..percentage4 {
        let mut a: i32 = rand::distributions::Range::new(1, 101).ind_sample(&mut rng);

        let mut b: i32 = 11;
        while 100 % b != 0 {
            b = rand::distributions::Range::new(2, 30).ind_sample(&mut rng);
        }

        a = a/b;
        if a == 0 {
            a = 1;
        }
        
        let problem = format!("\\( {}\\% \\textrm{{ av }} {} =\\)\\\\\\\n\n", b*a, 100/b);
        problems.push(problem);
    }

    for _i in 0..parenthesis_1 {

        let chars = vec!["a", "b", "c", "x", "y", "z", "i", "j", "k", "\\alpha", "\\beta", "\\gamma", "\\Omega", "\\theta", "\\varphi"];

        let selected_char = rng.choose(&chars).expect("");

        let mut first_term = format!("{}", selected_char);
        if 0 == rand::distributions::Range::new(0, 2).ind_sample(&mut rng) {
            first_term = format!("{}", rand::distributions::Range::new(2, 13).ind_sample(&mut rng));
        }

        let mut sign = "+";
        if 0 == rand::distributions::Range::new(0, 2).ind_sample(&mut rng) {
            sign = "-";
        }

        let mut second_term = format!("({} {} {})", selected_char, sign, rand::distributions::Range::new(2, 13).ind_sample(&mut rng));
        if 0 == rand::distributions::Range::new(0, 2).ind_sample(&mut rng) {
            second_term = format!("({} {} {})", rand::distributions::Range::new(2, 13).ind_sample(&mut rng), sign, selected_char);
        }

        let mut pre_sign = "";
        if 0 == rand::distributions::Range::new(0, 2).ind_sample(&mut rng) {
            pre_sign = "-";
        }
        
        let mut problem = format!("\\( {}{}{} =\\)\\\\\\\n\n", pre_sign, first_term, second_term);
        if 0 == rand::distributions::Range::new(0, 2).ind_sample(&mut rng) {
            problem = format!("\\( {}{}{} =\\)\\\\\\\n\n", pre_sign, second_term, first_term);
        }

        problems.push(problem);
    }

    for _i in 0..parenthesis_2 {

        let chars = vec!["a", "b", "c", "x", "y", "z", "\\alpha", "\\beta", "\\gamma", "\\Omega", "\\theta", "\\varphi"];

        let selected_chars = vec![rng.choose(&chars).expect(""), rng.choose(&chars).expect(""), rng.choose(&chars).expect("")];

        let mut inner_terms = vec!["inner_term".to_owned(), "inner_term".to_owned(), "inner_term".to_owned(), "inner_term".to_owned()];

        for i in 0..inner_terms.len() {
            if 0 == rand::distributions::Range::new(0, 2).ind_sample(&mut rng) {
                inner_terms[i] = format!("{}", rand::distributions::Range::new(1, 10).ind_sample(&mut rng));
            } else {
                inner_terms[i] = format!("{}", rng.choose(&selected_chars).expect(""));
            }
        }

        let mut outer_terms = vec!["outer_term".to_owned(), "outer_term".to_owned()];
        for i in 0..outer_terms.len() {
            if i != 1 && 0 == rand::distributions::Range::new(0, 2).ind_sample(&mut rng) {
                outer_terms[i] = inner_terms[2 * i].to_owned();
            } else {
                let mut sign = "+";
                if 0 == rand::distributions::Range::new(0, 2).ind_sample(&mut rng) {
                    sign = "-";
                }
                outer_terms[i] = format!("({} {} {})", inner_terms[2 * i], sign, inner_terms[2*i + 1]);
            }
        }
        
        let problem = format!("\\( {}{} =\\)\\\\\\\n\n", outer_terms[0], outer_terms[1]);
        problems.push(problem);
    }

    for _i in 0..trig_1 {
        let scale = 0.1;
        let base: f64 = scale * 50.0;
        let height: f64 = scale * (rand::distributions::Range::new(5, 50).ind_sample(&mut rng) as f64);
        let xpos: f64 = scale * (rand::distributions::Range::new(-15, 65).ind_sample(&mut rng) as f64);

        let angle_a = (180.0/3.14159)*((base*xpos) / (base*((xpos*xpos + height*height).sqrt()))).acos();
        let xpos_2 = xpos - base;
        let angle_c = (180.0/3.14159)*((-base*xpos_2) / (base*((xpos_2*xpos_2 + height*height).sqrt()))).acos();
        let angle_b = 180.0 - angle_a - angle_c;

        let mut a_s: String = format!("{:.0}^{{\\circ}}", angle_a);
        let mut b_s: String = format!("{:.0}^{{\\circ}}", angle_b);
        let mut c_s: String = format!("{:.0}^{{\\circ}}", angle_c);

        let chars = vec!["a", "b", "c", "x", "y", "z", "\\alpha", "\\beta", "\\gamma", "\\Omega", "\\theta", "\\varphi"];
        let selected_char: String = format!("{}", rng.choose(&chars).expect(""));

        let r: i32 = rand::distributions::Range::new(0, 3).ind_sample(&mut rng);
        if r == 0 {
            a_s = selected_char.clone();
        } else if r == 1 {
            b_s = selected_char.clone();
        } else {
            c_s = selected_char.clone();
        }

        let problem = format!(
"
\\(
\\begin{{tikzpicture}}[
my angle/.style={{
  every pic quotes/.append style={{text=black}},
  draw=gray,
  angle radius=0.5cm,
}}]
\\normalsize
\\coordinate [label=left:${}$] (A) at (0, 0);
\\coordinate [label=above:${}$] (B) at ({}, {});
\\coordinate [label=right:${}$] (C) at ({}, 0);
\\draw (C) -- (B) -- (A) -- (C);
\\pic [my angle] {{angle=B--C--A}};
\\pic [my angle] {{angle=A--B--C}};
\\pic [my angle] {{angle=C--A--B}};
\\huge
\\node[] at ({}, {}) {{{} =}};
\\end{{tikzpicture}}
\\)\\\\\\
\\huge
\n\n", 
            a_s,
            b_s, xpos, height,
            c_s, base,
            scale*75.0, 0.5*height, selected_char);

        problems.push(problem);
    }

    let slice: &mut [String] = problems.as_mut_slice();
    rng.shuffle(slice);

    for problem in slice {
        file.write_all(problem.as_bytes()).expect("6");
    }

    file.write_all(b"\\end{document}\n").expect("7");
}
