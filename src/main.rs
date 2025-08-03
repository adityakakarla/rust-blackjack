use core::panic;
use std::io;
use rand::{Rng};
use std::process::Command;

const WAIT_TIME: i8 = 2;

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

fn wait_n_seconds(n: i8){
    let mut child = Command::new("sleep").arg(n.to_string()).spawn().unwrap();
    let _result = child.wait().unwrap();
}

fn main() {
    let mut cards : Vec<Card> = Vec::new();
    let card1: Card = generate_random_card();
    let card2: Card = generate_random_card();
    
    println!("Your two cards are the {} of {} and the {} of {}", card1.rank, card1.suit, card2.rank, card2.suit);
    wait_n_seconds(WAIT_TIME);
    println!("Your current total value is {}", card1.value + card2.value);
    
    let mut next_step: String = String::new();
    let mut curr_sum = card1.value + card2.value;
    
    cards.push(card1);
    cards.push(card2);

    loop {
        wait_n_seconds(WAIT_TIME);
        if curr_sum > 21 {
            println!("You went over 21, with a final sum of {}", curr_sum);
            return
        } else if curr_sum == 21 {
            println!("Blackjack! You win.");
            return
        }

        println!("Would you like to hit or stay?");
        next_step.clear();
        io::stdin().read_line(&mut next_step)
        .expect("Failed to understand next step");
        if next_step.trim().to_lowercase().contains("hit") {
            let new_card = generate_random_card();
            curr_sum += new_card.value;
            println!("Your card was {} of {}", new_card.rank, new_card.suit);
            wait_n_seconds(WAIT_TIME);
            println!("New total value is {}", curr_sum);
            cards.push(new_card);
            continue;
        }

        println!("Your final value is {}", curr_sum);
        break;
    }

    let mut dealer_cards : Vec<Card> = Vec::new();
    let dealer_card1: Card = generate_random_card();
    let dealer_card2: Card = generate_random_card();

    wait_n_seconds(WAIT_TIME);
    println!("The dealer cards are the {} of {} and the {} of {}",
    dealer_card1.rank, dealer_card1.suit, dealer_card2.rank, dealer_card2.suit);

    wait_n_seconds(WAIT_TIME);
    println!("Dealer current value is {}", dealer_card1.value + dealer_card2.value);
    let mut curr_dealer_sum = dealer_card1.value + dealer_card2.value;
    
    cards.push(dealer_card1);
    cards.push(dealer_card2);

    loop {
        wait_n_seconds(2);
        if curr_dealer_sum > 21 {
            println!("You win! The dealer ended with a value of {} (over the limit)", curr_dealer_sum);
            return;
        } else if curr_dealer_sum >= 17 {
            if curr_sum > curr_dealer_sum {
                println!("You beat the dealer! {} vs {}", curr_sum, curr_dealer_sum);
            } else if curr_sum < curr_dealer_sum {
                println!("You lost. {} vs {}", curr_sum, curr_dealer_sum);
            } else {
                println!("You tied. Both of you got {}", curr_sum);
            }
            return;
        }

        let new_card = generate_random_card();
        curr_dealer_sum += new_card.value;
        println!("Dealer will draw a new card");
        wait_n_seconds(WAIT_TIME);
        println!("Dealer card was {} of {}", new_card.rank, new_card.suit);
        wait_n_seconds(WAIT_TIME);
        println!("New dealer value is {}", curr_dealer_sum);
        dealer_cards.push(new_card);
    }
}
