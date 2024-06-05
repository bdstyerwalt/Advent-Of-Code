use core::panic;

#[derive(Debug, PartialEq, PartialOrd)]
struct Hand {
    strength: i32,
    values: Vec<i32>,
    cards: String,
    bet: i32,
}

impl Hand {
    fn new(cards: String, bet: i32) -> Self {
        let values: Vec<i32> = Hand::get_hand_values(&cards);
        let strength: i32 = Hand::score_hand(&values);
        Self {
            cards: cards,
            values: values,
            strength: strength,
            bet: bet,
        }
    }

    fn get_hand_values(cards: &String) -> Vec<i32> {
        let mut hand_vals: Vec<i32> = vec![];
        for i in 0..cards.len() {
            hand_vals.push(Hand::get_card(&cards[i..i+1]))
        }
        return hand_vals;
    }

    fn get_card(card: &str) -> i32 {
        let val = match card {
            "A" => 14, 
            "K" => 13, 
            "Q" => 12, 
            "J" => 11, 
            "T" => 10, 
            "9" => 9, 
            "8" => 8, 
            "7" => 7,
            "6" => 6, 
            "5" => 5, 
            "4" => 4, 
            "3" => 3, 
            "2" => 2,
            _ => 0,
        };
        return val;
    }

    fn score_hand(values: &Vec<i32>) -> i32 {
        let mut temp = values.clone();
        temp.sort();
        temp.dedup();

        let mut hand_rank_vec: Vec<i32> = vec![];
        for i in 0..temp.len() {
            hand_rank_vec.push(values.iter().filter(|&x| *x == temp[i]).count() as i32)
        }
        
        hand_rank_vec.sort_by(|a, b| b.cmp(a));
        let rank: i32 = match hand_rank_vec[0] {
            5 => 6, // Five of a kind
            4 => 5, // Four of a kind
            3 => if hand_rank_vec[1] == 2 { 4 } else { 3 }, // Full house or Three of a kind
            2 => if hand_rank_vec[1] == 2 { 2 } else { 1 }, // Two pair or One pair
            1 => 0, // High card only
            _ => panic!(),
        };
        return rank;
    }
}

pub fn process(input: &str) -> i32 {
    let mut all_hands: Vec<Hand> = vec![];
    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let hand: Hand = Hand::new(line[0].to_string(), line[1].parse().unwrap());
        // println!("{:?}", hand);
        all_hands.push(hand);
    }
    
    all_hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut total_winnings: i32 = 0;
    for i in 0..all_hands.len() {
        //println!("Hand {i} is rank {} out of {}", i+1, all_hands.len());
        total_winnings += all_hands[i].bet * (i as i32 + 1);
    }
    
    return total_winnings;
}

/*
fn check_high_card(h1: &Hand, h2: &Hand) -> bool{
    // Comparing the first card in each hand; return winner
    // Case of tie, proceed to next card, etc
    // returns true if hand 1 beats hand 2
    for i in 0..h1.values.len() {
        if h1.values[i] > h2.values[i] {
            return true
        }
    }
    return false;
}
*/

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn part1_sample() {
        let input: &str = include_str!("sample.txt");
        assert_eq!(6440, process(&input));
    }

    /*
    #[test]
    fn hand1_higher_than_hand2() {
        let h1 = Hand::new("6AAQK".to_string(), 69);
        let h2 = Hand::new("6TAQJ".to_string(), 420);
        assert!(check_high_card(&h1, &h2));
        assert_eq!(false, check_high_card(&h2, &h1));
    }
    */

    #[test]
    fn test_card_values() {
        let h1 = Hand::new("K9A3J".to_string(), 69);
        let h2 = Hand::new("Q479T".to_string(), 420);
        assert_eq!(h1.values, vec![13, 9, 14, 3, 11]);
        assert_eq!(h2.values, vec![12, 4,  7, 9, 10])
    }

    #[test]
    fn test_rank_hand() {
        let h1 = Hand::new("KKA3J".to_string(), 69);
        let h2 = Hand::new("QQQ99".to_string(), 420);
        let h3 = Hand::new("99Q99".to_string(), 420);
        assert_eq!(h1.strength, 1);
        assert_eq!(h2.strength, 4);
        assert_eq!(h3.strength, 5)
    }

    
}