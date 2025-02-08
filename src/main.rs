mod engine;
mod plotter;
use std::env;

fn print_help() {
    println!("Actions:");
    println!("\tplot\t\t\tPlots luckiness of values in a given range to 'chart.png'");
    println!("\tis_lucky\t\tTells you if the numbers in a given range are lucky");
    println!("\tluckiness\t\tPrints the luckiness of the numbers in a gien range");
    println!("\tconstructions\t\tPrints the constructions for numbers in a given range");
}

fn print_err_non_int() {
    println!("Please enter a valid integer for the range");
}

fn print_err_range() {
    println!("Please enter a valid range, ex: '1 15'");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        println!("Usage: {} action [range]", args[0]);
        print_help();
        return;
    }

    let mut single_value: bool = false;
    let mut lower_bound: i64 = 1;
    let mut upper_bound: i64 = 1;

    
    if args.len() == 3 {
        single_value = true;
        match args[2].parse::<i64>() {
            Ok(x) => upper_bound = x,
            Err(_) => {
                print_err_non_int();
                return;
            }
        }
    }

    if args.len() == 4 {
        match args[2].parse::<i64>() {
            Ok(x) => lower_bound = x,
            Err(_) => {
                print_err_non_int();
                return;
            }
        }
        match args[3].parse::<i64>() {
            Ok(x) => upper_bound = x,
            Err(_) => {
                print_err_non_int();
                return;
            }
        }


    }

    if lower_bound > upper_bound {
        print_err_range();
        return;
    }

    if lower_bound < 1 {
        print_err_range();
        return;
    }
    
    let mut engine = engine::LuckyNumberEngine::new();
    match args[1].as_str() {
        "plot" => {
            engine.generate_range(lower_bound, upper_bound);

            let mut data: Vec<(u32, u32)> = Vec::new();


            for i in lower_bound..=upper_bound {
                data.push((i as u32, engine.luckiness(i) as u32));
            }
            match plotter::quick_plot("Value", "Luckiness", data) {
                Ok(_) => println!("Results saved to chart.png"),
                Err(x) => println!("Error creating chart: {:?}", x)
            }
            return;
        }
        "constructions" => {
            if single_value {
                lower_bound = upper_bound;
            }
            engine.generate_range(lower_bound, upper_bound);

            for i in lower_bound..=upper_bound {
                let constructions = engine.constructions(i);
                if constructions.len() == 0 {
                    println!("{} [Not Lucky]", i);
                } else {
                    print!("{}:", i);
                    for construction in engine.constructions(i) {
                        println!("\t{}", construction);
                    }
                }
            }
            return;
        }
        "luckiness" => {
            if single_value {
                lower_bound = upper_bound;
            }
            engine.generate_range(lower_bound, upper_bound);

            for i in lower_bound..=upper_bound {
                println!("{}: {}", i, engine.luckiness(i));
            }
            return;
        }
        "is_lucky" => {
            if single_value {
                lower_bound = upper_bound;
            }
            engine.generate_range(lower_bound, upper_bound);

            for i in lower_bound..=upper_bound {
                println!("{}: {}", i, engine.is_lucky(i));
            }
            return;
        }
        "help" => {print_help(); return;}
        _ => {println!("Unknown action: {}", args[1]); print_help(); return;}
    }
}