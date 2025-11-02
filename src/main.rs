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

    let orbital: Vec<&str> = vec![
       "1s", "1s",
       "2s", "2s",
       "2p", "2p", "2p", "2p", "2p", "2p",
       "3s", "3s",
       "3p", "3p", "3p", "3p", "3p", "3p",
       "4s", "4s",
       "3d", "3d", "3d", "3d", "3d", "3d", "3d", "3d", "3d", "3d",
       "4p", "4p", "4p", "4p", "4p", "4p",
       "5s", "5s",
       "4d", "4d", "4d", "4d", "4d", "4d", "4d", "4d", "4d", "4d",
       "5p", "5p", "5p", "5p", "5p", "5p",
       "6s", "6s",
       "4f", "4f", "4f", "4f", "4f", "4f", "4f", "4f", "4f", "4f", "4f", "4f", "4f", "4f",
       "5d", "5d", "5d", "5d", "5d", "5d", "5d", "5d", "5d", "5d",
       "6p", "6p", "6p", "6p", "6p", "6p",
       "7s", "7s",
       "5f", "5f", "5f", "5f", "5f", "5f", "5f", "5f", "5f", "5f", "5f", "5f", "5f", "5f",
       "6d", "6d", "6d", "6d", "6d", "6d", "6d", "6d", "6d", "6d",
       "7p", "7p", "7p", "7p", "7p", "7p"
    ];

    loop {
        //Declare variables
        let mut input: String = String::new();
        let mut num:u8 = 0;
        let mut i:u8 = 0;
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
            println!("1s^1");
            continue;
        } else if num == 2 {
            println!("1s^2");
            continue;
        } else if num > 118 {
            print!("Invalid: too high");
            continue;
        }

        //Calculate
        while i < num {
            current = orbital.get(i as usize).unwrap();

            if last != "" {
                if current == last {
                    count += 1;
                } else {
                    total += current;
                    total += "^";
                    total += count.to_string().as_str();
                    total += " ";

                    count = 0;
                }
            }

            last = current;
            i += 1;
        }

        println!("\r1s^2 {}", total);
    }

}
