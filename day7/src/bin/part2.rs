fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}


#[derive(Debug, PartialEq, PartialOrd)]
struct Hand {
    strength: i32,
    values: Vec<i32>,
    cards: String,
    bet: i32,
    jokers: i32,
}

impl Hand {
    fn new(cards: String, bet: i32) -> Self {
        let values: Vec<i32> = Hand::get_hand_values(&cards);
        let joker_cnt: i32 = Hand::get_joker_count(&cards);
        let strength: i32 = Hand::score_hand(&values, &joker_cnt);
        Self {
            cards: cards,
            values: values,
            strength: strength,
            bet: bet,
            jokers: joker_cnt,
        }
    }

    fn get_joker_count(cards: &String) -> i32 {
        return cards.chars().filter(|c| c == &'J').count() as i32;
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
            "T" => 10, 
            "9" => 9, 
            "8" => 8, 
            "7" => 7,
            "6" => 6, 
            "5" => 5, 
            "4" => 4, 
            "3" => 3, 
            "2" => 2,
            "J" => 1, 
            _ => 0,
        };
        return val;
    }

    fn score_hand(values: &Vec<i32>, joker_cnt: &i32) -> i32 {
        let mut temp = values.clone();
        temp.sort();
        temp.dedup();
        temp.retain(|x| x != &1); // Removes jokers to make ranking easier

        if temp.is_empty() { return 6; } // five of a kind 
        
        let mut rank_vec: Vec<i32> = vec![];
        for i in 0..temp.len() {
            rank_vec.push(values.iter().filter(|&x| *x == temp[i]).count() as i32)
        }
        
        
        rank_vec.sort_by(|a, b| b.cmp(a));

        if rank_vec[0] == 5 || rank_vec[0] + joker_cnt == 5 {
            return 6; // Five of a kind
        } else if rank_vec[0] == 4 || rank_vec[0] + joker_cnt == 4 { 
            return 5; // Four of a kind
        } else if (rank_vec[0] == 3 && rank_vec[1] == 2) || (rank_vec[0] + rank_vec[1] + joker_cnt == 5) {
            return 4; // Full house
        } else if rank_vec[0] + joker_cnt == 3 {
            return 3; // Three of a kind
        } else if (rank_vec[0] == 2 && rank_vec[1] == 2) || (rank_vec[0] + rank_vec[1] + joker_cnt == 4) {
            return 2; // Two pair
        } else if rank_vec[0] + joker_cnt == 2 {
            return 1; // One pair
        } else {
            return 0;
        }
    }
}

fn process(input: &str) -> i32 {
    let mut all_hands: Vec<Hand> = vec![];
    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let hand: Hand = Hand::new(line[0].to_string(), line[1].parse().unwrap());
        //println!("{:?}", hand);
        all_hands.push(hand);
    }
    
    all_hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut total_winnings: i32 = 0;
    for i in 0..all_hands.len() {
        let winnings = all_hands[i].bet * (i as i32 + 1); 
        //println!("Hand {i} ({:?}) is rank {} | Winnings {}", all_hands[i].cards, i+1, winnings);
        total_winnings += winnings;
    }
    
    return total_winnings;
}

#[cfg(test)]
mod tests {
    use crate::process;
    use crate::Hand;

    #[test]
    fn part2_sample() {
        let input = include_str!("sample.txt");
        assert_eq!(5905, process(&input));
    }

    #[test]
    fn test_joker_count() {
        let h1 = Hand::new("KKA3J".to_string(), 69);
        let h2 = Hand::new("QQQJJ".to_string(), 420);
        assert_eq!(h1.jokers, 1);
        assert_eq!(h2.jokers, 2)
    }

    #[test]
    fn test_joker_higher_card() {
        let h1 = Hand::new("QQJ22".to_string(), 69);
        let h2 = Hand::new("QQ222".to_string(), 420);
        //println!("TESTING JOKER HIGH CARD\n---------------");
        //println!("{:?}\n{:?}", h1, h2);
        assert!(h2 > h1)
    }

    #[test]
    fn test_joker_hand_rank() {
        let h1 = Hand::new("QQQJ2".to_string(), 69);
        let h2 = Hand::new("QQQ22".to_string(), 420);
        assert!(h1 > h2)
    }
}