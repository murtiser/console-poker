#[cfg(test)]
mod tests {
    use crate::card::CardNum;
    use crate::combinations::*;

    #[test]
    fn true_pair_test() {
        // all these list are sorted cuz all of combination-check functions imply passing sorted lists to them (expect straight flush and flush royale)
        let cards_value: Vec<u32> = vec![2, 3, 5, 5, 8, 11, 12];
        assert!(pair(&cards_value));
    }
    #[test]
    fn false_pair_test() {
        let cards_value: Vec<u32> = vec![2, 3, 4, 5, 8, 11, 12];
        assert!(!pair(&cards_value));
    }

    #[test]
    fn true_two_pair_test() {
        let cards_value: Vec<u32> = vec![3, 3, 5, 5, 8, 11, 12];
        assert!(two_pair(&cards_value));
    }
    #[test]
    fn false_two_pair_test() {
        let cards_value: Vec<u32> = vec![2, 3, 5, 5, 8, 11, 12];
        assert!(!two_pair(&cards_value));
    }

    #[test]
    fn true_set_test() {
        let cards_value: Vec<u32> = vec![3, 5, 5, 5, 8, 11, 12];
        assert!(set(&cards_value));
    }
    #[test]
    fn false_set_test() {
        let cards_value: Vec<u32> = vec![3, 4, 5, 5, 8, 11, 12];
        assert!(!set(&cards_value));
    }

    #[test]
    fn true_straight_test() {
        let cards_value: Vec<u32> = vec![3, 4, 5, 6, 7, 11, 12];
        assert!(straight(&cards_value));
    }
    #[test]
    fn false_straight_test() {
        let cards_value: Vec<u32> = vec![3, 4, 5, 5, 7, 11, 12];
        assert!(!straight(&cards_value));
    }

    #[test]
    fn true_flush_test() {
        let cards_suit: Vec<u32> = vec![2, 3, 3, 3, 3, 3, 4];
        assert!(flush(&cards_suit));
    }
    #[test]
    fn false_flush_test() {
        let cards_suit: Vec<u32> = vec![2, 3, 3, 3, 3, 4, 4];
        assert!(!flush(&cards_suit));
    }

    #[test]
    fn true_full_house_test() {
        let mut cards_value: Vec<u32> = vec![3, 3, 5, 5, 5, 11, 12];
        assert!(full_house(&mut cards_value));
    }
    #[test]
    fn false_full_house_test() {
        let mut cards_value: Vec<u32> = vec![3, 4, 5, 5, 5, 11, 12];
        assert!(!full_house(&mut cards_value));
    }

    #[test]
    fn true_quad_test() {
        let cards_value: Vec<u32> = vec![3, 5, 5, 5, 5, 11, 12];
        assert!(quad(&cards_value));
    }
    #[test]
    fn false_quad_test() {
        let cards_value: Vec<u32> = vec![3, 3, 5, 5, 5, 11, 12];
        assert!(!quad(&cards_value));
    }

    #[test]
    fn true_straight_flush_test1() {
        let num_cards: Vec<CardNum> = vec![
            CardNum { suit: 1, value: 4 },
            CardNum { suit: 3, value: 11 },
            CardNum { suit: 1, value: 6 },
            CardNum { suit: 1, value: 7 },
            CardNum { suit: 1, value: 8 },
            CardNum { suit: 1, value: 5 },
            CardNum { suit: 4, value: 12 },
        ];
        assert!(straight_flush(&num_cards));
    }

    #[test]
    fn true_straight_flush_test2() {
        let num_cards: Vec<CardNum> = vec![
            CardNum { suit: 1, value: 4 },
            CardNum { suit: 1, value: 11 },
            CardNum { suit: 1, value: 6 },
            CardNum { suit: 1, value: 7 },
            CardNum { suit: 1, value: 8 },
            CardNum { suit: 1, value: 5 },
            CardNum { suit: 1, value: 12 },
        ];
        assert!(straight_flush(&num_cards));
    }

    #[test]
    fn false_straight_flush_test1() {
        let num_cards: Vec<CardNum> = vec![
            CardNum { suit: 1, value: 4 },
            CardNum { suit: 1, value: 11 },
            CardNum { suit: 1, value: 6 },
            CardNum { suit: 1, value: 9 },
            CardNum { suit: 1, value: 8 },
            CardNum { suit: 1, value: 5 },
            CardNum { suit: 1, value: 12 },
        ];
        assert!(!straight_flush(&num_cards));
    }

    #[test]
    fn false_straight_flush_test2() {
        let num_cards: Vec<CardNum> = vec![
            CardNum { suit: 3, value: 14 },
            CardNum { suit: 3, value: 13 },
            CardNum { suit: 4, value: 7 },
            CardNum { suit: 2, value: 13 },
            CardNum { suit: 2, value: 12 },
            CardNum { suit: 2, value: 8 },
            CardNum { suit: 1, value: 14},
        ];
        assert!(!straight_flush(&num_cards));
    }

    #[test]
    fn true_royal_flush_test() {
        let num_cards: Vec<CardNum> = vec![
            CardNum { suit: 1, value: 10 },
            CardNum { suit: 3, value: 11 },
            CardNum { suit: 1, value: 11 },
            CardNum { suit: 1, value: 14 },
            CardNum { suit: 1, value: 12 },
            CardNum { suit: 1, value: 13 },
            CardNum { suit: 4, value: 12 },
        ];
        assert!(royal_flush(&num_cards));
    }
    #[test]
    fn false_royal_flush_test() {
        let num_cards: Vec<CardNum> = vec![
            CardNum { suit: 1, value: 10 },
            CardNum { suit: 3, value: 11 },
            CardNum { suit: 1, value: 11 },
            CardNum { suit: 1, value: 14 },
            CardNum { suit: 1, value: 4 },
            CardNum { suit: 1, value: 13 },
            CardNum { suit: 4, value: 12 },
        ];
        assert!(!royal_flush(&num_cards))
    }
}