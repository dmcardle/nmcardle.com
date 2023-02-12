use crate::rand::{get_system_random_stream, shuffle, RandomStream};

const NUM_COLORS: usize = 6;

#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
    White,
    Black,
    Yellow,
}

/// This is effectively a multiset of `Element`. It represents quantities of
/// each kind of currency.
#[derive(Clone, Debug, PartialEq)]
struct ColorCounts([usize; NUM_COLORS]);

impl ColorCounts {
    const ZERO: ColorCounts = ColorCounts([0; NUM_COLORS]);

    /// The game starts with seven of every regular token and five wild tokens.
    const GAME_START: ColorCounts = ColorCounts([7, 7, 7, 7, 7, 5]);

    fn get(&self, color: Color) -> usize {
        let index = color as usize;
        self.0[index]
    }

    fn plus(&self, other: &ColorCounts) -> Option<ColorCounts> {
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

impl From<Color> for ColorCounts {
    fn from(color: Color) -> Self {
        let mut counts = ColorCounts::ZERO;
        counts.0[color as usize] += 1;
        counts
    }
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

#[derive(Clone)]
struct CardRow {
    face_up: Vec<Card>,
    hidden: Vec<Card>,
}

impl CardRow {
    fn new() -> Self {
        CardRow {
            face_up: Vec::new(),
            hidden: Vec::new(),
        }
    }

    fn new_shuffled(rand: &mut dyn RandomStream, cards: &[Card], num_face_up: usize) -> CardRow {
        let mut cards = Vec::from(cards);
        shuffle(rand, &mut cards);
        let (cards_up, cards_down) = cards.split_at(num_face_up);
        CardRow {
            face_up: Vec::from(cards_up),
            hidden: Vec::from(cards_down),
        }
    }
}

enum TurnAction {
    TakeThreeTokens(Color, Color, Color),
    TakeTwoTokens(Color),
    Reserve(Card),
    Purchase(Card),
}

impl TurnAction {
    fn apply_to(self, player: &mut Player, game: &mut Game) -> Option<()> {
        match self {
            TurnAction::TakeThreeTokens(color1, color2, color3) => {
                let colors = ColorCounts::from(color1)
                    .plus(&ColorCounts::from(color2))?
                    .plus(&ColorCounts::from(color3))?;
                game.bank = game.bank.minus(&colors)?;
                player.tokens = player.tokens.plus(&colors)?;
            }
            TurnAction::TakeTwoTokens(color) => {
                let colors = ColorCounts::from(color);
                game.bank = game.bank.minus(&colors)?;
                player.tokens = player.tokens.plus(&colors)?;
            }
            TurnAction::Reserve(card) => {
                // TODO: Find card in rows? (Seems inefficient.)
                // TODO: Replace card with one from face-down cards.
                // TODO: Add card to player's hand, but face-down.
                todo!()
            }
            TurnAction::Purchase(_) => todo!(),
        }
        Some(())
    }
}

#[derive(Clone)]
enum PlayerStrategy {
    Random,
}

#[derive(Clone)]
struct Player {
    hand: CardRow,
    tokens: ColorCounts,
    strategy: PlayerStrategy,
}

impl Player {
    fn new() -> Self {
        Player {
            hand: CardRow::new(),
            tokens: ColorCounts::ZERO,
            strategy: PlayerStrategy::Random,
        }
    }

    fn play(&mut self, game: &Game) -> TurnAction {
        todo!()
    }
}

struct Players {
    /// All players in the game.
    players: Vec<Player>,
    /// The index of an element in `players`. Represents whose turn it is.
    turn: usize,
}

struct Game {
    /// Quantities of tokens that are available.
    bank: ColorCounts,
    /// The prices of cards in the noble row are interpreted as quantities of cards.
    noble_row: CardRow,
    /// The prices of regular cards are interpreted as quantities of tokens.
    card_rows: [CardRow; 3],
}

impl Game {
    const NUM_CARDS_FACE_UP: usize = 4;
    const ALL_NOBLES: [Card; 1] = [Card {
        points: 4,
        price: ColorCounts([4, 4, 0, 0, 0, 0]),
        value: ColorCounts::ZERO,
    }];
    const ALL_CARDS: [&[Card]; 3] = [&[], &[], &[]];

    fn new_random_game(rand: &dyn RandomStream) -> Game {
        let mut rand = get_system_random_stream().expect("Should get system random stream");
        Game {
            bank: ColorCounts::GAME_START,
            noble_row: CardRow::new_shuffled(
                rand.as_mut(),
                &Game::ALL_NOBLES,
                Game::NUM_CARDS_FACE_UP,
            ),
            card_rows: Game::ALL_CARDS
                .map(|cards| CardRow::new_shuffled(rand.as_mut(), cards, Game::NUM_CARDS_FACE_UP)),
        }
    }
}

impl Players {
    fn new(num_players: usize) -> Self {
        Players {
            players: vec![Player::new(); num_players],
            turn: 0,
        }
    }

    fn step(&mut self, game: &mut Game) {
        let num_players = self.players.len();
        let player = &mut self.players[self.turn];
        self.turn = (self.turn + 1) % num_players;

        player
            .play(&game)
            .apply_to(player, game)
            .expect("Player's selected action should be valid");
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

        let money = ColorCounts([1, 2, 3, 4, 5, 6]);
        assert_eq!(money.plus(&money), Some(ColorCounts([2, 4, 6, 8, 10, 12])));

        let other_money = ColorCounts([2, 3, 4, 5, 6, 7]);
        assert_eq!(
            other_money.plus(&money),
            Some(ColorCounts([3, 5, 7, 9, 11, 13]))
        );

        let max_money = ColorCounts([usize::MAX, 0, 0, 0, 0, 0]);
        assert_eq!(money.plus(&max_money), None);
    }

    #[test]
    fn test_color_counts_minus() {
        assert_eq!(
            ColorCounts::ZERO.minus(&ColorCounts::ZERO),
            Some(ColorCounts::ZERO)
        );

        let money = ColorCounts([1, 2, 3, 4, 5, 6]);
        assert_eq!(money.minus(&money), Some(ColorCounts::ZERO));

        let other_money = ColorCounts([2, 3, 4, 5, 6, 7]);
        assert_eq!(
            other_money.minus(&money),
            Some(ColorCounts([1; NUM_COLORS]))
        );

        assert_eq!(money.minus(&other_money), None);
    }
}
