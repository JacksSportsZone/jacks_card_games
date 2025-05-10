use rand::seq::SliceRandom;
use rand::rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug,PartialEq)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Create a new shuffled deck
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);

        for suit in &[Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
            for rank in &[
                Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven,
                Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
            ] {
                cards.push(Card { suit: *suit, rank: *rank });
            }
        }

        let mut deck = Deck { cards };
        deck.shuffle(); // Optionally shuffle on creation
        deck
    }

    /// Shuffle the deck
    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    /// Draw a card from the top of the deck
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Get number of remaining cards
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Check if the deck is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Peek at the top card without removing it
    pub fn peek(&self) -> Option<&Card> {
        self.cards.last()
    }

    /// Return all cards (immutable)
    pub fn all_cards(&self) -> &[Card] {
        &self.cards
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn deck_of_cards_length() {
        let deck1 = Deck::new();
        assert_eq!(deck1.len(), 52);
    }
    #[test]
    fn deck_of_cards_unique(){
        let mut deck1 = Deck::new();
        let mut deck:Vec<Card> = Vec::with_capacity(52);
        while let Some(card) = deck1.draw(){
            deck.push(card);
        }
        let unique: HashSet<_> = deck.iter().collect();
        assert_eq!(unique.len(), 52, "Deck contains duplicate cards");
    }
    #[test]
    fn deck_of_cards_draw(){
        let mut deck1 = Deck::new();
        if let Some(card) = deck1.draw(){
            assert_eq!(deck1.len(), 51, "Deck is not empty after draw");
        }

    }
    #[test]
    fn deck_of_cards_draw_all(){
        let mut deck1 = Deck::new();
        let mut deck:Vec<Card> = Vec::with_capacity(52);
        while let Some(card) = deck1.draw(){
            deck.push(card);
        }
        assert_eq!(deck.len(), 52, "Deck is not full after draw");
        assert!(deck1.is_empty(), "Deck is not empty after draw");
    }
    #[test]
    fn deck_of_cards_shuffle(){
        let mut deck1 = Deck::new();
        let mut deck2 = Deck::new();
        deck1.shuffle();
        deck2.shuffle();
        assert_ne!(deck1, deck2, "Deck is not shuffled")
    }

    #[test]
    fn deck_of_cards_peek(){
        let deck1 = Deck::new();
        if let Some(_) = deck1.peek(){
            assert_eq!(deck1.len(), 52, "Deck lost card after peek");
        }
    }
}