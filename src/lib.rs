use std::collections::VecDeque;

use rand::{seq::SliceRandom, RngCore};

/// Encompasses both the deck to draw cards and a discard pile.
pub struct Deck<'a> {
    /// Cards yet to be drawn.
    draw: VecDeque<Card>,

    /// Discard pile of cards from previous rounds.
    discard: Vec<Card>,

    /// Used to shuffle the deck of cards.
    random: &'a mut dyn RngCore,
}

impl<'a> Deck<'a> {
    /// Creates, shuffles, and returns a new deck of cards.
    ///
    /// For BlackJack, typically a few decks are used (~6) to reduce the ability to card count.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsjack::Deck;
    /// use rand;
    ///
    /// let deck = Deck::new(6, &mut rand::thread_rng());
    /// ```
    ///
    /// # Panics
    ///
    /// If `decks` is not at least 1.
    pub fn new(decks: usize, random: &'a mut impl RngCore) -> Self {
        assert!(decks >= 1, "At least 1 deck is required");

        // All ranks of cards.
        const RANKS: [&Rank; 13] = [
            &Rank::Two,
            &Rank::Three,
            &Rank::Four,
            &Rank::Five,
            &Rank::Six,
            &Rank::Seven,
            &Rank::Eight,
            &Rank::Nine,
            &Rank::Ten,
            &Rank::Jack,
            &Rank::Queen,
            &Rank::King,
            &Rank::Ace,
        ];

        // All suits of cards.
        const SUITS: [&Suit; 4] = [&Suit::Clubs, &Suit::Diamonds, &Suit::Hearts, &Suit::Spades];

        let mut cards = Vec::<Card>::with_capacity(decks * RANKS.len() * SUITS.len());
        for _ in 0..decks {
            for rank in RANKS {
                for suit in SUITS {
                    cards.push(Card::new(rank, suit));
                }
            }
        }

        Self {
            draw: VecDeque::new(),
            discard: cards,
            random,
        }
    }

    /// Draws a card.
    ///
    /// If the discard pile is sufficiently large, shuffles it into the deck.
    pub fn draw(&mut self) -> Card {
        if self.discard.len() > self.draw.len() {
            // Move all of the draw into the discard pile.
            self.discard.extend(self.draw.iter());
            self.draw.clear();

            // Shuffle the discard pile.
            self.discard.shuffle(&mut self.random);

            // Move all of the discard pile into the draw.
            self.draw.extend(self.discard.iter());
            self.discard.clear();
        }
        self.draw.pop_front().unwrap_or_else(|| {
            panic!(
                "Should have had cards remaining (discard = {})",
                self.discard.len()
            )
        })
    }

    /// Discards a hand of cards.
    pub fn discard(&mut self, hand: &mut Hand) {
        self.discard.extend(hand.cards());
        hand.cards.clear();
    }
}

/// A playing card within a hand or on the table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    pub rank: &'static Rank,
    pub suit: &'static Suit,
}

impl Card {
    /// Creates a new card.
    pub fn new(rank: &'static Rank, suit: &'static Suit) -> Self {
        Self { rank, suit }
    }

    /// Whether this card is an ace.
    pub fn is_ace(&self) -> bool {
        self.rank == &Rank::Ace
    }

    /// Returns the sub-total score for this card, counting an ace as 1.
    pub fn score_ace_as_1(&self) -> u8 {
        match self.rank {
            Rank::Ace => 1,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            other => *other as u8 + 2,
        }
    }
}

/// Card rank, from [`Rank::Two`] to [`Rank::Ace`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rank {
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

/// Card suit. Used for visuals only.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/// A hand of cards, used for display and computational purposes.
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    /// Creates a new empty hand.
    pub fn new() -> Self {
        Self { cards: vec![] }
    }

    /// Returns the highest possible total, i.e. not busting by convering an ace.
    pub fn total(&self) -> u8 {
        let mut aces = 0;
        let mut sum = 0;
        for card in self.cards.iter() {
            sum += card.score_ace_as_1();
            if card.is_ace() {
                aces += 1;
            }
        }
        while sum <= 11 && aces > 0 {
            sum += 10;
            aces -= 1;
        }

        sum
    }

    /// Returns whether the hand wins immediately.
    pub fn is_black_jack(&self) -> bool {
        self.cards.len() == 2 && self.total() == 21
    }

    /// Returns whether this hand is a bust.
    pub fn is_bust(&self) -> bool {
        self.total() > 21
    }

    /// Returns the cards in the hand.
    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    /// Adds a card to a hand.
    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, iter};

    use super::*;
    use rand::{Error, RngCore};

    /// A simple RNG that intentionally does not generate very random output (i.e. for testing).
    struct CountingRng(u64);

    impl RngCore for CountingRng {
        fn next_u32(&mut self) -> u32 {
            self.next_u64() as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.0 += 1;
            self.0
        }

        fn fill_bytes(&mut self, _dest: &mut [u8]) {
            panic!("Not implemented")
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    #[test]
    fn new_deck_single_has_52_cards() {
        let mut rand = CountingRng(0);
        let mut deck = Deck::new(1, &mut rand);

        let full = iter::repeat_with(|| deck.draw()).take(52);
        let uniq = HashSet::<Card>::from_iter(full);
        assert_eq!(uniq.len(), 52);
    }

    #[test]
    fn new_deck_double_has_104_cards() {
        let mut rand = CountingRng(0);
        let mut deck = Deck::new(2, &mut rand);

        let full = iter::repeat_with(|| deck.draw()).take(104);
        let uniq = HashSet::<Card>::from_iter(full);
        assert_eq!(uniq.len(), 52);
    }

    #[test]
    fn deck_discard() {
        let mut rand = CountingRng(0);
        let mut deck = Deck::new(1, &mut rand);
        let mut hands = Hand::new();

        for _ in 0..4 {
            hands.add(deck.draw());
        }
        assert_eq!(hands.cards().len(), 4);

        deck.discard(&mut hands);
        assert_eq!(hands.cards().len(), 0);

        let full = iter::repeat_with(|| deck.draw()).take(52);
        let uniq = HashSet::<Card>::from_iter(full);
        assert_eq!(uniq.len(), 52);
    }

    #[test]
    fn card_new() {
        let a = Card::new(&Rank::Ten, &Suit::Clubs);
        let b = Card::new(&Rank::Ten, &Suit::Clubs);
        assert_eq!(a, b);
    }

    #[test]
    fn card_is_ace() {
        let c = Card::new(&Rank::Ace, &Suit::Clubs);
        assert!(c.is_ace());
    }

    #[test]
    fn card_total_2_10() {
        const RANKS: [&Rank; 9] = [
            &Rank::Two,
            &Rank::Three,
            &Rank::Four,
            &Rank::Five,
            &Rank::Six,
            &Rank::Seven,
            &Rank::Eight,
            &Rank::Nine,
            &Rank::Ten,
        ];

        for rank in RANKS {
            let c = Card::new(rank, &Suit::Clubs);
            assert_eq!(c.score_ace_as_1(), *rank as u8 + 2);
        }
    }

    #[test]
    fn card_total_jack_king() {
        const RANKS: [&Rank; 3] = [&Rank::Jack, &Rank::Queen, &Rank::King];

        for rank in RANKS {
            let c = Card::new(rank, &Suit::Clubs);
            assert_eq!(c.score_ace_as_1(), 10);
        }
    }

    #[test]
    fn card_total_ace() {
        let c = Card::new(&Rank::Ace, &Suit::Clubs);
        assert_eq!(c.score_ace_as_1(), 1);
    }

    #[test]
    fn hand_total_no_ace() {
        let mut h = Hand::new();
        h.add(Card::new(&Rank::Ten, &Suit::Clubs));
        h.add(Card::new(&Rank::Seven, &Suit::Clubs));

        assert_eq!(h.total(), 17);
    }

    #[test]
    fn hand_total_ace_as_11() {
        let mut h = Hand::new();
        h.add(Card::new(&Rank::Nine, &Suit::Clubs));
        h.add(Card::new(&Rank::Ace, &Suit::Clubs));

        assert_eq!(h.total(), 20);
    }

    #[test]
    fn hand_total_ace_as_1() {
        let mut h = Hand::new();
        h.add(Card::new(&Rank::Six, &Suit::Clubs));
        h.add(Card::new(&Rank::Nine, &Suit::Clubs));
        h.add(Card::new(&Rank::Ace, &Suit::Clubs));

        assert_eq!(h.total(), 16);
    }

    #[test]
    fn hand_total_ace_as_1_and_11() {
        let mut h = Hand::new();
        h.add(Card::new(&Rank::Six, &Suit::Clubs));
        h.add(Card::new(&Rank::Ace, &Suit::Clubs));
        h.add(Card::new(&Rank::Ace, &Suit::Clubs));

        assert_eq!(h.total(), 18);
    }

    #[test]
    fn hand_is_black_jack() {
        let mut h = Hand::new();
        h.add(Card::new(&Rank::Ace, &Suit::Clubs));
        h.add(Card::new(&Rank::Jack, &Suit::Clubs));

        assert!(h.is_black_jack());
    }

    #[test]
    fn hand_is_not_black_jack_after_2_cards() {
        let mut h = Hand::new();
        h.add(Card::new(&Rank::Ace, &Suit::Clubs));
        h.add(Card::new(&Rank::Five, &Suit::Clubs));
        h.add(Card::new(&Rank::Five, &Suit::Clubs));

        assert!(!h.is_black_jack());
    }
}
