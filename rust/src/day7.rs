/*
 * Advent of Code 2023 Day 7
 */
use crate::day::Day;
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Day7;

impl Day for Day7 {
    fn part1(&self, input: String) {
        let hand_inputs: Vec<&str> = input.split("\n").collect();
        let mut hands: Vec<Hand> = hand_inputs
            .iter()
            .map(|hand| Hand::new(hand.to_string()))
            .collect();
        hands.sort_by(|a, b| a.compare(b, false));
        let mut sum: i32 = 0;
        for i in 0..hands.len() {
            sum += (hands.len() - i) as i32 * hands[i].bid;
        }
        println!("Part 1: {}", sum);
    }

    fn part2(&self, input: String) {
        let hand_inputs: Vec<&str> = input.split("\n").collect();
        let mut hands: Vec<Hand> = hand_inputs
            .iter()
            .map(|hand| Hand::new(hand.to_string()))
            .collect();
        hands.sort_by(|a, b| a.compare(b, true));
        let mut sum: i32 = 0;
        for i in 0..hands.len() {
            sum += (hands.len() - i) as i32 * hands[i].bid;
        }
        println!("Part 2: {}", sum);
    }
}

pub struct Hand {
    cards: Vec<String>,
    bid: i32,
}

impl Hand {
    fn new(input: String) -> Hand {
        let input_split: Vec<&str> = input.split(" ").collect();
        let mut cards: Vec<String> = input_split[0]
            .split("")
            .skip(1)
            .map(|s| s.to_string())
            .collect();
        cards.remove(cards.len() - 1);
        let bid = input_split[1].parse::<i32>().unwrap();
        Hand { cards, bid }
    }

    fn compare(&self, hand: &Hand, with_jokers: bool) -> Ordering {
        // Card ranks are as follows:
        // 2, 3, 4, 5, 6, 7, 8, 9, T(10), J(11), Q(12), K(13), A(14)
        let mut rank_map: HashMap<String, i32> = HashMap::from([
            ("2".to_string(), 2),
            ("3".to_string(), 3),
            ("4".to_string(), 4),
            ("5".to_string(), 5),
            ("6".to_string(), 6),
            ("7".to_string(), 7),
            ("8".to_string(), 8),
            ("9".to_string(), 9),
            ("T".to_string(), 10),
            ("J".to_string(), 11),
            ("Q".to_string(), 12),
            ("K".to_string(), 13),
            ("A".to_string(), 14),
        ]);
        if with_jokers {
            rank_map.remove("J");
            rank_map.insert("J".to_string(), 1);
        }

        // Compare the types of the hands
        let self_type = self.get_type(with_jokers);
        let hand_type = hand.get_type(with_jokers);
        if self_type > hand_type {
            return Ordering::Less;
        } else if self_type < hand_type {
            return Ordering::Greater;
        } else {
            // Compare card ranks
            for i in 0..self.cards.len() {
                let self_rank = rank_map.get(&self.cards[i]).unwrap();
                let hand_rank = rank_map.get(&hand.cards[i]).unwrap();
                if self_rank > hand_rank {
                    return Ordering::Less;
                } else if self_rank < hand_rank {
                    return Ordering::Greater;
                }
            }
            return Ordering::Equal;
        }
    }

    fn get_type(&self, with_jokers: bool) -> i32 {
        // Hand types are as follows:
        // 1. High Card
        // 2. One Pair
        // 3. Two Pairs
        // 4. Three of a Kind
        // 5. Full House
        // 6. Four of a Kind
        // 7. Five of a Kind

        // Sort the hand before checking
        let mut cards = self.cards.clone();
        cards.sort();

        // Check for jokers
        let mut num_jokers = 0;
        if with_jokers {
            for card in &cards {
                if card == "J" {
                    num_jokers += 1;
                }
            }
        }

        // Check for five of a kind
        if cards[0] == cards[4] {
            return 7;
        }

        // Check for four of a kind
        if cards[0] == cards[3] || cards[1] == cards[4] {
            if num_jokers > 0 {
                return 7;
            }
            return 6;
        }

        // Check for full house
        if (cards[0] == cards[2] && cards[3] == cards[4])
            || (cards[0] == cards[1] && cards[2] == cards[4])
        {
            if num_jokers > 0 {
                return 7;
            }
            return 5;
        }

        // Check for three of a kind
        if cards[0] == cards[2] || cards[1] == cards[3] || cards[2] == cards[4] {
            if num_jokers > 0 {
                return 6;
            }
            return 4;
        }

        // Check for two pairs
        if (cards[0] == cards[1] && cards[2] == cards[3])
            || (cards[0] == cards[1] && cards[3] == cards[4])
            || (cards[1] == cards[2] && cards[3] == cards[4])
        {
            if num_jokers == 1 {
                return 5;
            } else if num_jokers == 2 {
                return 6;
            }
            return 3;
        }

        // Check for one pair
        if cards[0] == cards[1]
            || cards[1] == cards[2]
            || cards[2] == cards[3]
            || cards[3] == cards[4]
        {
            if num_jokers > 0 {
                return 4;
            }
            return 2;
        }

        // Otherwise, it's a high card
        if num_jokers == 1 {
            return 2;
        }
        return 1;
    }
}
