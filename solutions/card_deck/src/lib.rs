use rand::Rng;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let random_number = rand::thread_rng().gen_range(0..4);
        let suits = [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade];
        suits[random_number]
    }
// (u8) to a suit (1 -> Heart, 2 -> Diamonds, 3 -> Spade, 4 -> Club).
    pub fn translate(value: u8) -> Suit {
        if value == 1 {
            return Suit::Heart;
        }else if value == 2{
            return Suit::Diamond;
        }else if value == 3{
            return Suit::Spade;
        }else if value == 4{
            return Suit::Club;
        }
        Suit::Heart 
    }
}

impl Rank {
    pub fn random() -> Rank {
        let random_number = rand::thread_rng().gen_range(0..14);
        let ranks = [
            Rank::Ace,
            Rank::Number(2),
            Rank::Number(3),
            Rank::Number(4),
            Rank::Number(5),
            Rank::Number(6),
            Rank::Number(7),
            Rank::Number(8),
            Rank::Number(9),
            Rank::Number(10),
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ];
        ranks[random_number]
    }
    // (u8) to a rank ( 1 -> Ace, 2 -> 2, .., 10 -> 10, 11 -> Jack, 12 -> Queen, 13 -> King)
    pub fn translate(value: u8) -> Rank {
        if value == 1 {
            return Rank::Ace;
        }else if value == 2{
            return Rank::Number(2);
        }else if value == 3{
            return Rank::Number(3);
        }else if value == 4{
            return Rank::Number(4);
        }else if value == 5{
            return Rank::Number(5);
        }else if value == 6{
            return Rank::Number(6);
        }else if value == 7{
            return Rank::Number(7);
        }else if value == 8{
            return Rank::Number(8);
        }else if value == 9{
            return Rank::Number(9);
        }else if value == 10{
            return Rank::Number(10);
        }else if value == 11{
            return Rank::Number(11);
        }else if value == 12{
            return Rank::Jack;
        }else if value == 13{
            return Rank::Queen;
        }
        Rank::Number(2)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}
