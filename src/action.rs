use crate::bet::bet;

pub fn action(money: &mut usize, is_folded: bool) -> (bool, usize) {
    if is_folded {
        return (true, 0);
    } else {
        loop {
            let act = input(Some("What would you do?: (c - check / b - bet / f - fold)"));
            match act.trim() {
                "c" => {
                    return (false, 0);
                }
                "b" => {
                    let bet = bet(money);
                    if bet == 0 {
                        println!("Wrong input, try again!");
                        continue;
                    }
                    return (false, bet);
                }
                "f" => {
                return (true, 0);
                }
                _ => println!("Wrong input, try again!"),
            }
        }
    }
}

pub fn input(txt: Option<&str>) -> String {
    let mut input = String::new();
    
    match txt {
        Some(text) => {
            println!("{text}");
        }
        None => (),
    }
    std::io::stdin().read_line(&mut input).expect("Input failed");
    input
}