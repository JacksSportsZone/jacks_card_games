mod card_deck;

use card_deck::Deck;
#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use card_deck::Card;
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
