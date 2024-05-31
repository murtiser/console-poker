use std::num::ParseIntError;
use crate::{action::input, crypt::*};
use std::{
    fs::{File, OpenOptions},
    io::{
        Read,
        ErrorKind,
        Write,
    },
};

pub fn read_money(file_name: &str) -> usize {
    match File::open(file_name) {
        Ok(mut money_file) => {
            let mut money = String::new();
            let mut money_encr = Vec::new();
            money_file.read_to_end(&mut money_encr).unwrap();

            let money_bytes = decrypt_money(&money_encr);
            for byte in money_bytes {
                money.push(byte as char);
            }

            let money_usize: usize = money.trim().parse()
                .expect("File `money.bin` has characters that impossible to turn into `usize`");
            
            return money_usize;
        }

        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    let mut money_file = File::create(file_name).unwrap();
                    let money_amount = b"1500";
                    let money_encr = encrypt_money(money_amount);
                    money_file.write(&money_encr).unwrap();

                    return 1500;
                }

                _ => {
                    panic!("Problem with file {error:?}");
                }
            }
        }
    }
}

pub fn bet(money: &mut usize) -> usize {
    let bet_amount: Result<usize, ParseIntError> = input(Some(&format!("Your money is {money}. How much do you bet?")))
        .trim()
        .parse();

    match bet_amount {
        Ok(bet) => {
            if bet > *money {
                println!("You don't have that much money!");
                return 0;
            } else {
                *money -= bet;
                
                bet
            }
        },

        Err(_) => 0
    }
}

pub fn blind(money: &mut usize) -> usize {
    loop {
        let act = input(Some(&format!("Blind is 50, now you have {money} money, wanna play? (y / n)")));

        match act.trim() {
            "y" => {
                *money -= 50;
                return 50;
            },

            "n" => return 0,

            _ => println!("Wrong input, try again!"),
        }
    }
}

pub fn win(blind: usize, first_pot: usize, second_pot: usize, third_pot: usize, multiply: f64, money: &usize) {
    let win_money = (*money as f64 + ((blind as f64 * multiply + first_pot as f64 * multiply) + (second_pot as f64 * (multiply - 0.1) + third_pot as f64 * (multiply - 0.2)))) as usize;
    println!("You won {}!", win_money - *money);

    let win_money_str = &win_money.to_string()[..];
    let win_money_bytes = win_money_str.as_bytes();
    let encr_money = encrypt_money(win_money_bytes);

    let mut money_file = OpenOptions::new().write(true).open("money.bin").unwrap();
    money_file.set_len(0).unwrap();
    money_file.write_all(&encr_money).unwrap();
}