use std::io;

fn main() {
    //Add Data
    let atomic_number: Vec<&str> = vec![
        "", "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg", "Al", "Si", "P", "S", "Cl", "Ar",
        "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge", "As", "Se", "Br", "Kr",
        "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd", "In", "Sn", "Sb", "Te", "I", "Xe",
        "Cs", "Ba", "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb", "Dy", "Ho", "Er", "Tm", "Yb", "Lu",
        "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl", "Pb", "Bi", "Po", "At", "Rn",
        "Fr", "Ra", "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", "Fm", "Md", "No", "Lr",
        "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn", "Fl", "Lv", "Ts", "Og"
    ];

    let orbitals = vec![
        ("1s", 2),
        ("2s", 2),
        ("2p", 6),
        ("3s", 2),
        ("3p", 6),
        ("4s", 2),
        ("3d", 10),
        ("4p", 6),
        ("5s", 2),
        ("4d", 10),
        ("5p", 6),
        ("6s", 2),
        ("4f", 14),
        ("5d", 10),
        ("6p", 6),
        ("7s", 2),
        ("5f", 14),
        ("6d", 10),
        ("7p", 6),
    ];

    loop {
        //Declare variables
        let mut input: String = String::new();
        let mut num:u8 = 0;
        let mut current:&str = "";
        let mut last:&str = "";
        let mut count:u8 = 0;
        let mut total:String = String::new();

        io::stdin().read_line(&mut input).expect("Error reading input"); //Reads Line

        //Convert input to atomic number from number or symbol
        if input.trim().parse::<f64>().is_ok() {
            num = input.trim().parse().expect("Failed to convert String into u8");
        } else {
            num = atomic_number.iter().position(|&r| r == input.trim()).unwrap_or(0) as u8;
            if num == 0 {
                println!("Invalid number/letter(s)");
                continue;
            }
        }

        if num == 1 {
            println!("1s1");
            continue;
        } else if num == 2 {
            println!("1s2");
            continue;
        } else if num > 118 {
            print!("Invalid: too high");
            continue;
        }

        let mut i:usize = 0;
        let mut remaining:u8 = num;

        //Calculate
        while remaining > 0 {
            let (name, capacity) = orbitals[i];
            if capacity < remaining {
                total.push_str(name);
                total.push_str(capacity.to_string().as_str());
                total.push_str(" ");
                remaining -= capacity;
                i += 1;
            } else {
                total.push_str(name);
                total.push_str(remaining.to_string().as_str());
                total.push_str(" ");
                remaining = 0;
            }

        }

        println!("{}", total);
    }

}
