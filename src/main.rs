use std::process::exit;

use rsjack::{Deck, Hand};

use crate::ui::{DrawHand, Menu};

pub mod lib;
pub mod ui;

fn main() {
    println!("Welcome to BlackJack!");

    let mut random = rand::thread_rng();
    let mut deck = Deck::new(6, &mut random);
    let mut dealer = Hand::new();
    let mut player = Hand::new();

    loop {
        deck.discard(&mut dealer);
        deck.discard(&mut player);

        dealer.add(deck.draw());
        dealer.add(deck.draw());
        player.add(deck.draw());
        player.add(deck.draw());

        println!("DEALER: ??");
        println!("{}", DrawHand::for_dealer(&dealer, true));

        loop {
            println!("PLAYER: {}", player.total());
            println!("{}", DrawHand::for_player(&player));

            if player.is_bust() {
                break;
            }

            match ui::menu() {
                Menu::Quit => {
                    println!("\nBye!");
                    exit(0);
                }

                Menu::Draw => {
                    player.add(deck.draw());
                }

                Menu::Stay => break,
            }

            println!();
        }

        if player.is_bust() {
            println!("** bust **");
            continue;
        }

        println!();

        while dealer.total() < 17 {
            dealer.add(deck.draw());

            println!("DEALER: {}", dealer.total());
            println!("{}", DrawHand::for_dealer(&dealer, false));
        }

        if dealer.is_bust() {
            println!("Dealer busts ... You win!\n")
        } else {
            match player.total().cmp(&dealer.total()) {
                std::cmp::Ordering::Less => {
                    println!("You lose!\n");
                }
                std::cmp::Ordering::Equal => {
                    println!("You tie!\n");
                }
                std::cmp::Ordering::Greater => {
                    println!("You win!\n");
                }
            }
        }
    }
}
