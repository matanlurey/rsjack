use std::{fmt::Display, io::stdin};

use rsjack::{Hand, Rank, Suit};

/// Given a hand of cards, provides [`Display`].
pub struct DrawHand<'a> {
    hand: &'a Hand,
    hide: bool,
}

impl<'a> DrawHand<'a> {
    /// Creates a new drawable player hand for the UI.
    pub fn for_player(hand: &'a Hand) -> Self {
        Self { hand, hide: false }
    }

    /// Creates a new drawable dealer hand for the UI.
    ///
    /// If `hide` is true, assumes exactly two cards and hides the second in the UI.
    pub fn for_dealer(hand: &'a Hand, hide: bool) -> Self {
        if hide {
            assert_eq!(hand.cards().len(), 2, "Only hide cards when exactly 2");
        }
        Self { hand, hide }
    }
}

impl Display for DrawHand<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.hand.cards().len();

        let top = vec!["┌──┐"; len].join(" ");
        let bot = vec!["└──┘"; len].join(" ");
        let mut mid = Vec::<String>::with_capacity(len);

        for (count, card) in self.hand.cards().iter().enumerate() {
            let first = count == 0;

            if first || !self.hide {
                let rank: String = match card.rank {
                    Rank::Ten => "0".into(),
                    Rank::Jack => "J".into(),
                    Rank::Queen => "Q".into(),
                    Rank::King => "K".into(),
                    Rank::Ace => "A".into(),
                    number => (*number as usize + 2).to_string(),
                };
                let suit: String = match card.suit {
                    Suit::Clubs => "♣".into(),
                    Suit::Diamonds => "♦".into(),
                    Suit::Hearts => "♥".into(),
                    Suit::Spades => "♠".into(),
                };
                mid.push(format!("│{}{}│", rank, suit));
            } else {
                mid.push("│░░│".into());
            }
        }

        writeln!(f, "{}", top)?;
        writeln!(f, "{}", mid.join(" "))?;
        writeln!(f, "{}", bot)?;

        Ok(())
    }
}

/// Possible return options from the player.
pub enum Menu {
    Quit,
    Draw,
    Stay,
}

/// Reads input from the player.
pub fn menu() -> Menu {
    const Q: &str = "Q";
    const D: &str = "D";
    const S: &str = "S";
    println!("{}: Quit | {}: Draw | {}: Stay", Q, D, S);

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

#[cfg(test)]
mod tests {
    use super::*;
    use rsjack::{Card, Hand};

    #[test]
    fn draw_dealer_hidden() {
        let mut h = Hand::new();
        h.add(Card::new(&Rank::Eight, &Suit::Clubs));
        h.add(Card::new(&Rank::Ten, &Suit::Clubs));

        let d = DrawHand::for_dealer(&h, true);
        assert_eq!(
            format!("{}", d),
            "┌──┐ ┌──┐\n\
             │8♣│ │░░│\n\
             └──┘ └──┘\n"
        );
    }

    #[test]
    fn draw_dealer_shown() {
        let mut h = Hand::new();
        h.add(Card::new(&Rank::Jack, &Suit::Hearts));
        h.add(Card::new(&Rank::Queen, &Suit::Spades));

        let d = DrawHand::for_dealer(&h, false);
        assert_eq!(
            format!("{}", d),
            "┌──┐ ┌──┐\n\
             │J♥│ │Q♠│\n\
             └──┘ └──┘\n"
        );
    }

    #[test]
    fn draw_player() {
        let mut h = Hand::new();
        h.add(Card::new(&Rank::King, &Suit::Diamonds));
        h.add(Card::new(&Rank::Ace, &Suit::Clubs));

        let d = DrawHand::for_player(&h);
        assert_eq!(
            format!("{}", d),
            "┌──┐ ┌──┐\n\
             │K♦│ │A♣│\n\
             └──┘ └──┘\n"
        );
    }
}
