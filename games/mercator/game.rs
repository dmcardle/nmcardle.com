use crate::rand::{get_system_random_stream, shuffle, RandomStream};

const NUM_COLORS: usize = 5;

pub enum Color {
    Red,
    Green,
    Blue,
    White,
    Black,
}

/// This is effectively a multiset of `Element`. It represents quantities of
/// each kind of currency.
#[derive(Clone, Debug, PartialEq)]
struct ColorCounts([usize; NUM_COLORS]);

impl ColorCounts {
    const ZERO: ColorCounts = ColorCounts([0; NUM_COLORS]);

    fn plus(&self, other:&ColorCounts) -> Option<ColorCounts> {
        let mut out = ColorCounts::ZERO;
        for i in 0..NUM_COLORS {
            out.0[i] = self.0[i].checked_add(other.0[i])?;
        }
        Some(out)
    }

    fn minus(&self, other: &ColorCounts) -> Option<ColorCounts> {
        let mut out = ColorCounts::ZERO;
        for i in 0..NUM_COLORS {
            if self.0[i] < other.0[i] {
                return None;
            }
            out.0[i] = self.0[i] - other.0[i];
        }
        Some(out)
    }
}

pub enum Token {
    Elemental(Color),
    Wild,
}

#[derive(Clone)]
struct Card {
    points: usize,
    /// The purchase price of this card.
    price: ColorCounts,
    /// The purchasing power this card gives when held. It could be represented
    /// by `Option<Color>`, but `ColorCounts` is more ergonomic for addition and
    /// subtraction.
    value: ColorCounts,
}

struct TableRow {
    face_up: Vec<Card>,
    hidden: Vec<Card>,
}

impl TableRow {
    fn new_shuffled(rand: &mut dyn RandomStream, cards: &[Card], num_face_up: usize) -> TableRow {
        let mut cards = Vec::from(cards);
        shuffle(rand, &mut cards);
        let (cards_up, cards_down) = cards.split_at(num_face_up);
        TableRow {
            face_up: Vec::from(cards_up),
            hidden: Vec::from(cards_down),
        }
    }
}

#[derive(Clone)]
struct Player {
    hand: Vec<Card>,
    tokens: ColorCounts,
}

impl Player {
    fn new() -> Self {
        Player {
            hand: Vec::new(),
            tokens: ColorCounts::ZERO,
        }
    }

    fn play_turn(&mut self, game: &mut Game) {
        todo!()
    }
}

struct Game {
    // The prices of cards in the noble row are interpreted as quantities of cards.
    nobles: TableRow,
    // The prices of regular cards are interpreted as quantities of tokens.
    rows: [TableRow; 3],

    players: Vec<Player>,
    // The index of an element in `players`.
    // TODO: Can I use a circular iterator or something?
    turn: usize,
}

impl Game {
    const NUM_CARDS_FACE_UP: usize = 4;
    const ALL_NOBLES: [Card; 1] = [Card {
        points: 4,
        price: ColorCounts([4, 4, 0, 0, 0]),
        value: ColorCounts::ZERO,
    }];
    const ALL_CARDS: [&[Card]; 3] = [&[], &[], &[]];

    fn new_random_game(rand: &dyn RandomStream, num_players: usize) -> Game {
        let mut rand = get_system_random_stream().expect("Should get system random stream");

        Game {
            nobles: TableRow::new_shuffled(
                rand.as_mut(),
                &Game::ALL_NOBLES,
                Game::NUM_CARDS_FACE_UP,
            ),
            rows: Game::ALL_CARDS
                .map(|cards| TableRow::new_shuffled(rand.as_mut(), cards, Game::NUM_CARDS_FACE_UP)),
            players: vec![Player::new(); num_players],
            turn: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_color_counts_plus() {
        assert_eq!(
            ColorCounts::ZERO.plus(&ColorCounts::ZERO),
            Some(ColorCounts::ZERO)
        );

        let money = ColorCounts([1, 2, 3, 4, 5]);
        assert_eq!(money.plus(&money), Some(ColorCounts([2, 4, 6, 8, 10])));

        let other_money = ColorCounts([2, 3, 4, 5, 6]);
        assert_eq!(other_money.plus(&money), Some(ColorCounts([3, 5, 7, 9, 11])));

        let max_money = ColorCounts([usize::MAX, 0, 0, 0, 0]);
        assert_eq!(money.plus(&max_money), None);
    }

    #[test]
    fn test_color_counts_minus() {
        assert_eq!(
            ColorCounts::ZERO.minus(&ColorCounts::ZERO),
            Some(ColorCounts::ZERO)
        );

        let money = ColorCounts([1, 2, 3, 4, 5]);
        assert_eq!(money.minus(&money), Some(ColorCounts::ZERO));

        let other_money = ColorCounts([2, 3, 4, 5, 6]);
        assert_eq!(other_money.minus(&money), Some(ColorCounts([1; 5])));

        assert_eq!(money.minus(&other_money), None);
    }
}
