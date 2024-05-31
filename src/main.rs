mod card;
mod combinations;
mod action;
mod test;
mod bet;
mod crypt;

pub use aead;
use action::*;
use card::*;
use bet::*;
use combinations::check_comb;
use std::process;

fn main() {
    let mut money = read_money("money.bin");

    let cards_value = vec![
        "2", "3", "4", "5",
        "6", "7", "8", "9",
        "10", "jack", "queen",
        "king", "ace",
    ];

    let cards_suit = vec![
        "cross", "spades", "hearts", "diamonds",
    ];

    let mut player_cards = Card::take_cards(&cards_suit, &cards_value, 2, None);
    let mut table_cards = Card::take_cards(&cards_suit, &cards_value, 5, Some(vec![&player_cards]));

    let blind = blind(&mut money);

    if blind == 0 {
        process::exit(0);
    }

    println!("Your cards is: {player_cards:?}");

    let (is_folded, first_pot) = action(&mut money, false);
    print!("First two cards on table is [{:?}, ", table_cards[0]);
    println!("{:?}]", table_cards[1]);

    let (is_folded, second_pot) = action(&mut money, is_folded);
    print!("The next two cards on table is [{:?}, ", table_cards[2]);
    println!("{:?}]", table_cards[3]);

    let (_, third_pot) = action(&mut money, is_folded);

    println!("The last one is [{:?}]", table_cards[4]);

    let multiply = check_comb(&mut player_cards, &mut table_cards);
    win(blind, first_pot, second_pot, third_pot, multiply, &money);

    if read_money("money.bin") < 50 {
        println!("You don't have money for next blind (you'll got another 1500 xd)");
        std::fs::remove_file("money.bin").unwrap();
        std::fs::remove_file("nonce.bin").unwrap();
    }
}