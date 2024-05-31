use crate::card::*;

pub fn check_comb<'a>(player_cards: &mut Vec<Card<'a>>, table_cards: &mut Vec<Card<'a>>) -> f64 {
    let mut all_cards: Vec<Card<'a>> = Vec::new();
    all_cards.append(player_cards);
    all_cards.append(table_cards);

    let mut num_cards: Vec<CardNum> = vec![];

    for mut card in all_cards {
        num_cards.push(card.to_num());
    }

    let mut cards_value = sort_by_value(&num_cards);
    let cards_suit = sort_by_suit(&num_cards);

    if royal_flush(&num_cards) {
        println!("Your combination is `Royal Flush`");
        return 5.0;
    } else if straight_flush(&num_cards) {
        println!("Your combination is `Straight Flush`");
        return 3.0;
    } else if quad(&cards_value) {
        println!("Your combination is `Quad`");
        return 2.5;
    } else if full_house(&mut cards_value) {
        println!("Your combination is `Full House`");
        return 2.0;
    } else if flush(&cards_suit) {
        println!("Your combination is `Flush`");
        return 1.8;
    } else if straight(&cards_value) {
        println!("Your combination is `Straight`");
        return 1.7;
    } else if set(&cards_value) {
        println!("Your combination is `Set`");
        return 1.5;
    } else if two_pair(&cards_value) {
        println!("Your combination is `Two Pairs`");
        return 1.2;
    } else if pair(&cards_value) {
        println!("Your combination is `Pair`");
        return 0.5;
    } else {
        println!("Your combination is `High Card`");
        return 0.0;
    }
}

pub fn royal_flush(num_cards: &Vec<CardNum>) -> bool {
    let mut comb_cards: Vec<CardNum> = vec![];

    for i in 0..num_cards.len() {
        if num_cards[i].value == 11 ||  num_cards[i].value == 12 || num_cards[i].value == 13 || num_cards[i].value == 14 || num_cards[i].value == 10 {
            comb_cards.push(num_cards[i]);
        }
    }

    if comb_cards.len() < 5 {
        return false;
    } 
    
    let sort_comb_suit = sort_by_suit(&comb_cards);
    if flush(&sort_comb_suit) {
        return true;
    } else {
        return false;
    }
}

pub fn straight_flush(num_cards: &Vec<CardNum>) -> bool {
    let mut comb_cards: Vec<CardNum> = vec![];

    for i in 0..num_cards.len() - 1 {
        comb_cards.push(num_cards[i]);
        for j in 1..num_cards.len() {
            if num_cards[i].suit == num_cards[j].suit {
                comb_cards.push(num_cards[j]);
            }
        }
        if comb_cards.len() >= 5 {
            break;
        } else {
            comb_cards.clear();
        }
    }
    
    if comb_cards.len() == 0 {
        return false;
    }

    let sort_comb_values = sort_by_value(&comb_cards);
    if straight(&sort_comb_values) {
        return true;
    } else {
        return false;
    }
}

pub fn quad(cards_value: &Vec<u32>) -> bool {
    for i in 3..cards_value.len() {
        if cards_value[i - 3] == cards_value[i] && cards_value[i - 2] == cards_value[i] && cards_value[i - 1] == cards_value[i] {
            return true;
        }
    }
    return false;
}

pub fn full_house(cards_value: &mut Vec<u32>) -> bool {
    for i in 1..cards_value.len() {
        if cards_value[i - 1] == cards_value[i] {
            cards_value.remove(i);
            cards_value.remove(i - 1);
            break;
        } else {
            return false;
        }
    }
    if set(&cards_value) {
        return true;
    } else {
        return false;
    }
}

pub fn flush(cards_suit: &Vec<u32>) -> bool {
    for i in 4..cards_suit.len() {
        if cards_suit[i - 4] == cards_suit[i] && cards_suit[i - 3] == cards_suit[i] && cards_suit[i - 2] == cards_suit[i] && cards_suit[i - 1] == cards_suit[i] {
            return true;
        }
    }
    return false;
}

pub fn straight(cards_value: &Vec<u32>) -> bool {
    for i in 4..cards_value.len() {
        if cards_value[i - 4] + 4 == cards_value[i] && cards_value[i - 3] + 3 == cards_value[i] && cards_value[i - 2] + 2 == cards_value[i] && cards_value[i - 1] + 1 == cards_value[i] {
            return true;
        }
    }
    return false;
}

pub fn set(cards_value: &Vec<u32>) -> bool {
    for i in 2..cards_value.len() {
        if cards_value[i - 2] == cards_value[i] && cards_value[i - 1] == cards_value[i] {
            return true;
        }
    }
    return false;
}

pub fn two_pair(cards_value: &Vec<u32>) -> bool {
    let mut flag = false;

    for i in 1..cards_value.len() {
        if cards_value[i - 1] == cards_value[i] && flag {
            return true;
        } else if cards_value[i - 1] == cards_value[i] && !flag {
            flag = true;
        }
    }
    false
}

pub fn pair(cards_value: &Vec<u32>) -> bool {
    for i in 1..cards_value.len() {
        if cards_value[i - 1] == cards_value[i] {
            return true;
        }
    }
    return false;
}

pub fn sort_by_value(num_cards: &Vec<CardNum>) -> Vec<u32> {
    let mut cards_value: Vec<u32> = vec![];
    for card_num in num_cards {
        cards_value.push(card_num.value);
    }
    cards_value.sort();
    return cards_value;
}

pub fn sort_by_suit(num_cards: &Vec<CardNum>) -> Vec<u32> {
    let mut cards_suit: Vec<u32> = vec![];
    for card_num in num_cards {
        cards_suit.push(card_num.suit);
    }
    cards_suit.sort();
    return cards_suit;
}