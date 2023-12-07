use std::{collections::HashMap, default};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}


#[derive(Debug)]
struct Hand {
    //TODO: update cards to vector of cards, using an enum for rank
    cards: String,
    values: Vec<i32>,
    bet: i32,
    winnings: i32,
}

impl Hand {
    fn new(cards: String, bet: i32) -> Self {
        let values = Hand::get_hand_values(&cards);
        Self {
            cards: cards,
            values: values,
            bet: bet,
            winnings: 0
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
}

fn process(input: &str) -> i32 {
    let mut all_hands: Vec<Hand> = vec![];
    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let hand: Hand = Hand::new(line[0].to_string(), line[1].parse().unwrap());
        all_hands.push(hand);
    }
    println!("{:?}", all_hands);
    return 0;
}

fn score_hand(hand: Hand) -> i32 {
    todo!()
}

fn check_high_card(h1: Hand, h2: Hand) -> bool{
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

#[cfg(test)]
mod tests {
    use crate::{process, Hand, check_high_card};

    #[test]
    fn part1_sample() {
        let input: &str = include_str!("sample.txt");
        assert_eq!(6440, process(&input));
    }

    #[test]
    fn hand1_higher_than_hand2() {
        let h1 = Hand::new("69420".to_string(), 69);
        let h2 = Hand::new("68420".to_string(), 420);
        assert!(check_high_card(h1, h2))
    }
}