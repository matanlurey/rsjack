use std::{collections::VecDeque, fmt::Display, io::stdin, process::exit};

use rand::seq::SliceRandom;

fn main() {
    println!("Welcome to BlackJack!");

    let random = &mut rand::thread_rng();
    let mut discard = deck(6);
    let mut deck = VecDeque::<Card>::new();

    let half_way = discard.len() / 2;

    loop {
        if discard.len() >= half_way {
            println!("... shuffling ...");

            // Merge both the discard pile and the deck.
            let mut temp = discard.clone();
            temp.extend(deck.iter());

            // Clear both piles.
            deck.clear();
            discard.clear();

            // Recreate the deck.
            temp.shuffle(random);
            deck = temp.into();
        }

        let mut dealer = vec![deck.pop_front().unwrap(), deck.pop_front().unwrap()];
        dealer[0].hide = false;

        let mut player = vec![deck.pop_front().unwrap(), deck.pop_front().unwrap()];
        player[0].hide = false;
        player[1].hide = false;

        println!("DEALER: ??");
        println!("{}", Hand(&dealer));

        loop {
            println!("PLAYER: {}", score(&player));
            println!("{}", Hand(&player));

            if is_bust(&player) {
                break;
            }

            match menu() {
                Menu::Quit => {
                    println!("Bye!");
                    exit(0);
                }

                Menu::Draw => {
                    let mut card = deck.pop_front().unwrap();
                    card.hide = false;
                    player.push(card);
                }

                Menu::Stay => break,
            }

            println!();
        }

        if is_bust(&player) {
            println!("** bust **");
            continue;
        }

        println!();
        dealer[1].hide = false;

        while score(&dealer) < 17 {
            let mut card = deck.pop_front().unwrap();
            card.hide = false;
            dealer.push(card);

            println!("DEALER: {}", score(&dealer));
            println!("{}", Hand(&dealer));
        }

        match score(&player).cmp(&score(&dealer)) {
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

enum Menu {
    Quit,
    Draw,
    Stay,
}

/// Reads input from the player.
fn menu() -> Menu {
    const Q: &str = "Q";
    const D: &str = "D";
    const S: &str = "S";
    println!("[{}: Quit | {}: Draw | {}: Stay]", Q, D, S);

    loop {
        let mut buffer = String::new();
        stdin()
            .read_line(&mut buffer)
            .expect("Could not read from stdin");
        match buffer.to_ascii_uppercase().trim() {
            Q => return Menu::Quit,
            D => return Menu::Draw,
            S => return Menu::Stay,
            u => {
                println!("Unsupported: {u}");
            }
        }
    }
}

/// Creates and returns a "deck" (or deck of decks) of cards.
fn deck(count: usize) -> Vec<Card> {
    assert!(count >= 1, "At least 1 deck is required");
    const RANKS: [Rank; 13] = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];
    const SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
    let mut deck = Vec::<Card>::with_capacity(count * RANKS.len() * SUITS.len());
    for _ in 0..count {
        for rank in RANKS {
            for suit in SUITS {
                deck.push(Card {
                    rank,
                    suit,
                    hide: true,
                })
            }
        }
    }
    deck
}

/// Card rank, from 2 to Ace.
#[derive(Clone, Copy, PartialEq)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

/// Card suit.
#[derive(Clone, Copy)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/// A playing card (both a rank and a suit).
#[derive(Clone, Copy)]
struct Card {
    rank: Rank,
    suit: Suit,
    hide: bool,
}

/// A hand of cards, in practice shown directly on the table.
#[derive(Clone)]
struct Hand<'a>(&'a Vec<Card>);

impl Display for Hand<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.0.len();

        let top = vec!["┌──┐"; len].join(" ");
        let bot = vec!["└──┘"; len].join(" ");
        let mut mid = Vec::<String>::with_capacity(len);

        for card in self.0.iter() {
            if card.hide {
                mid.push("│░░│".into());
            } else {
                let rank: String = match card.rank {
                    Rank::Ten => "0".into(),
                    Rank::Jack => "J".into(),
                    Rank::Queen => "Q".into(),
                    Rank::King => "K".into(),
                    Rank::Ace => "A".into(),
                    number => (number as usize + 2).to_string(),
                };
                let suit: String = match card.suit {
                    Suit::Clubs => "♣".into(),
                    Suit::Diamonds => "♦".into(),
                    Suit::Hearts => "♥".into(),
                    Suit::Spades => "♠".into(),
                };
                mid.push(format!("│{}{}│", rank, suit));
            }
        }

        writeln!(f, "{}", top)?;
        writeln!(f, "{}", mid.join(" "))?;
        writeln!(f, "{}", bot)?;

        Ok(())
    }
}

fn score(cards: &Vec<Card>) -> u8 {
    let mut sum: u8 = 0;
    for card in cards {
        sum += match card.rank {
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ace => 1,
            number => (number as u8) + 2,
        };
    }
    if sum > 11 {
        return sum;
    }
    for card in cards {
        if card.rank == Rank::Ace {
            sum += 10;
        }
        if sum > 11 {
            return sum;
        }
    }
    sum
}

fn is_bust(cards: &Vec<Card>) -> bool {
    score(cards) > 21
}
