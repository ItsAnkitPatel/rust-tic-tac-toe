use colored::Colorize;
use std::io;

struct Winner {
    continue_loop: bool,
}

fn find_winner(vec: &Vec<&str>) -> Winner {
    let mut x_choices: Vec<usize> = vec![];
    let mut o_choices: Vec<usize> = vec![];

    for (index, value) in vec.iter().enumerate() {
        if *value == "X" {
            x_choices.push(index);
        }
        if *value == "O" {
            o_choices.push(index);
        }
    }
    if x_choices.len() <= 2 && o_choices.len() <= 2 {
        return Winner {
            continue_loop: true,
        };
    }
    let mut x_i = 0;
    while x_i < x_choices.len() - 2 {
        let slice = &mut x_choices[x_i..x_i + 3];

        let mut j = 1;
        let first_diff = slice[j] - slice[j - 1];
        j += 1;
        let second_diff = slice[j] - slice[j - 1];

        if first_diff == second_diff {
            print_gridbox(vec);
            println!("{} {}", "X".green(), "won the game!".bright_green());
            return Winner {
                continue_loop: false,
            };
        }
        x_i += 1;
    }

    let mut o_i = 0;
    while o_i < o_choices.len() - 2 {
        let slice = &mut o_choices[o_i..o_i + 3];
        let mut j = 1;
        let first_diff = slice[j] - slice[j - 1];
        j += 1;
        let second_diff = slice[j] - slice[j - 1];

        if first_diff == second_diff {
            print_gridbox(vec);
            println!("{} {}", "O".cyan(), "won the game!".bright_green());
            return Winner {
                continue_loop: false,
            };
        }

        o_i += 1;
    }

    for val in vec {
        if val.is_empty() {
            return Winner {
                continue_loop: true,
            };
        }
    }

    print_gridbox(vec);

    println!("{}", "DRAW. Nobody Won!".bright_blue());
    Winner {
        continue_loop: false,
    }
}

fn print_gridbox(vec: &[&str]) {
    let mut index = 0;
    for row in 0..5 {
        for col in 0..5 {
            // if row is even print the spaces along with value
            if row % 2 == 0 {
                if col == 1 || col == 3 {
                    print!("||")
                } else if col == 0 || col == 2 || col == 4 {
                    if vec[index].is_empty() {
                        print!("   ")
                    } else {
                        print!(" {} ", vec[index]);
                    }
                    index += 1;
                } else {
                    print!("   ");
                }
            }
            // if row is odd then just need to take care of printing grid lines
            else {
                if col == 1 || col == 3 {
                    print!("||")
                } else {
                    print!("===");
                }
            }
        }
        println!()
    }
}
fn main() {
    println!("{}", "===Welcome to the arena fellas!===\n".blue());
    let mut vec: Vec<&str> = vec![""; 9];

    let mut first_user_turn: bool = true; //if false that means it's second user turn

    loop {
        let mut input = String::from("");

        print_gridbox(&vec);
        if first_user_turn {
            println!(
                "{}",
                "It's X user turn. Input number between 1 to 9".magenta()
            );
        } else {
            println!("{}", "It's O user turn. Input number between 1 to 9".cyan())
        }

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<i32>() {
                    Ok(input_val) => {
                        if !(1..=9).contains(&input_val) {
                            println!(
                                "{}{}",
                                "The value need to be under 1 to 9\n".red(),
                                "Try again\n".bright_white()
                            );
                            continue;
                        }

                        if let Some(value) = vec.get_mut((input_val - 1) as usize) {
                            if !value.is_empty() {
                                println!(
                                    "{} {} {}",
                                    "Box".red(),
                                    (input_val - 1).to_string().red(),
                                    "is already occupied. Try a different number from 1 to 9".red(),
                                );
                                continue;
                            }

                            if first_user_turn {
                                *value = "X";
                            } else {
                                *value = "O";
                            }

                            let winner = find_winner(&vec);

                            if !winner.continue_loop {
                                break;
                            }
                        } else {
                            println!("Index out of bounds");
                        }
                    }
                    Err(_) => {
                        println!(
                            "{}",
                            "Invalid input. Please use integer value from 1 to 9 ".red(),
                        )
                    }
                }

                first_user_turn = !first_user_turn;
            }
            Err(error) => println!("Some error happened {error}"),
        }
    }
}
