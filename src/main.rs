use core::panic;
use std::{io};
use rand::{Rng};

mod utils;
use crate::utils::wait_n_seconds;

const WAIT_TIME: u64 = 2;

struct Hand {
    cards: Vec<Card>,
    values: Vec<i8>
}

impl Hand {
    fn clear_invalid(&mut self){
        let values = self.values.iter().filter(|&x| *x <= 21).copied()
        .collect();
        self.values = values;
    }

    fn format_options(&mut self) -> String {
        let mut formatted_options = String::new();
        match self.values.len() {
            0 => panic!("No values discovered"),
            1.. => {
                formatted_options.push_str(&self.values[0].to_string());

                for i in 1..(self.values.len()) {
                    formatted_options.push_str(" or ");
                    formatted_options.push_str(&self.values[i].to_string());
                };

                formatted_options
            }
        }
    }

    fn busted(&mut self) -> bool {
        if ((self.values.len() == 0) && (self.cards.len() != 0)) || (self.max_value() > 21) {
            return true;
        }
        false
    }

    fn blackjack(&mut self) -> bool {
        (self.max_value() == 21) && (self.cards.len() == 2)
    }

    fn max_value(&mut self) -> i8 {
        self.values.iter().max().copied().unwrap_or(0)
    }

    fn add_card(&mut self, card: Card) {
        if self.values.len() == 0 {
            self.values.push(0);
        }

        if card.rank == "Ace" {
            for i in 0..(self.values.len()) {
                self.values[i] += 1;
                self.values.push(self.values[i] + 10)
            }
        } else {
            for i in 0..(self.values.len()) {
                self.values[i] += card.value;
            }
        }
        self.clear_invalid();
        self.cards.push(card);
    }
}

struct Card {
    rank: String,
    suit: String,
    value: i8
}

fn generate_random_card() -> Card {
    let mut rng = rand::rng();
    let mut random_num = rng.random_range(0..13);
    let rank;
    let value;
    match random_num {
        0 => {
            rank = String::from("Ace");
            value = 11;
        },
        1..=9 => {
            rank = (random_num + 1).to_string();
            value = random_num + 1;
        },
        10 => {
            rank = String::from("Jack");
            value = 10;
    },
        11 => {
            rank = String::from("Queen");
            value = 10;
        },
        12 => {
            rank = String::from("King");
            value = 10;
        },
        ..0 | 13.. => panic!("Invalid card value")
    }
    
    random_num = rng.random_range(0..4);
    let suit;
    match random_num {
        0 => suit = String::from("Hearts"),
        1 => suit = String::from("Diamonds"),
        2 => suit = String::from("Spades"),
        3 => suit = String::from("Clubs"),
        ..0 | 4.. => panic!("Invalid suit")
    }

    Card{rank, suit, value}
}

fn main() {
    let mut user_hand = Hand{cards: Vec::new(), values: Vec::new()};
    let card1: Card = generate_random_card();
    let card2: Card = generate_random_card();
    
    println!("Your two cards are the {} of {} and the {} of {}", card1.rank, card1.suit, card2.rank, card2.suit);
    user_hand.add_card(card1);
    user_hand.add_card(card2);

    let mut next_step: String = String::new();

    loop {
        wait_n_seconds(WAIT_TIME);
        if user_hand.busted() {
            println!("You went over 21, so you lost");
            return
        } else if user_hand.blackjack() {
            println!("Blackjack! You win unless dealer has blackjack.");
            break
        } else if user_hand.max_value() == 21 {
            println!("You hit 21, which is the best possible value.");
            break
        } else {
            println!("You have {}", user_hand.format_options());
        }
        wait_n_seconds(WAIT_TIME);

        println!("Would you like to hit or stay?");
        next_step.clear();
        io::stdin().read_line(&mut next_step)
        .expect("Failed to understand next step");
        if next_step.trim().to_lowercase().contains("hit") {
            let new_card = generate_random_card();
            println!("Your card was {} of {}", new_card.rank, new_card.suit);
            user_hand.add_card(new_card);
            continue;
        }

        println!("Your final value is {}", user_hand.max_value());
        break;
    }

    let mut dealer_hand = Hand{cards: Vec::new(), values: Vec::new()};
    let dealer_card1: Card = generate_random_card();
    let dealer_card2: Card = generate_random_card();

    wait_n_seconds(WAIT_TIME);
    println!("The dealer cards are the {} of {} and the {} of {}",
    dealer_card1.rank, dealer_card1.suit, dealer_card2.rank, dealer_card2.suit);

    dealer_hand.add_card(dealer_card1);
    dealer_hand.add_card(dealer_card2);

    
    if user_hand.blackjack() && dealer_hand.blackjack() {
        wait_n_seconds(WAIT_TIME);
        println!("You and the dealer both had blackjack. It's a tie.");
        return
    } else if user_hand.blackjack() {
        wait_n_seconds(WAIT_TIME);
        println!("You had blackjack and the dealer didn't. You win!");
        return
    } else if dealer_hand.blackjack() {
        wait_n_seconds(WAIT_TIME);
        println!("Dealer had blackjack and you didn't. You lose.");
        return
    }

    loop {
        wait_n_seconds(WAIT_TIME);
        if dealer_hand.busted() {
            println!("You win! The dealer ended with a value of {} (over the limit)", dealer_hand.max_value());
            return;
        }

        println!("Dealer has {}", dealer_hand.format_options());
        wait_n_seconds(WAIT_TIME);
        
        if dealer_hand.max_value() >= 17 {
            if user_hand.max_value() > dealer_hand.max_value() {
                println!("You beat the dealer! {} vs {}", user_hand.max_value(), dealer_hand.max_value());
            } else if user_hand.max_value() < dealer_hand.max_value() {
                println!("You lost. {} vs {}", user_hand.max_value(), dealer_hand.max_value());
            } else {
                println!("You tied. Both of you got {}", user_hand.max_value());
            }
            return;
        }

        let new_card = generate_random_card();
        println!("Dealer will draw a new card");
        wait_n_seconds(WAIT_TIME);
        println!("Dealer card was {} of {}", new_card.rank, new_card.suit);
        dealer_hand.add_card(new_card);
    }
}
