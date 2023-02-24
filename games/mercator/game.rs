use crate::cards::{get_all_cards, get_all_nobles};
use crate::rand::{get_system_random_stream, shuffle, RandomStream};

const NUM_COLORS: usize = 6;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Blue,
    White,
    Black,
    Yellow,
}

impl From<usize> for Color {
    fn from(n: usize) -> Color {
        match n {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            3 => Color::White,
            4 => Color::Black,
            5 => Color::Yellow,
            _ => panic!("No Color for {n}"),
        }
    }
}

/// This is effectively a multiset of `Color`. It represents quantities of each
/// kind of currency.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorCounts([usize; NUM_COLORS]);

impl ColorCounts {
    /// This [ColorCounts] value contains zero of every color.
    pub const ZERO: ColorCounts = ColorCounts([0; NUM_COLORS]);

    /// The game begins with seven of every regular token and five wild tokens.
    const BANK_START: ColorCounts = ColorCounts([7, 7, 7, 7, 7, 5]);

    /// Return the quantity of the given color.
    fn get(&self, color: Color) -> usize {
        let index = color as usize;
        self.0[index]
    }

    /// Add another [ColorCounts] to this one. Returns a value iff the result
    /// does not overflow.
    fn plus(&self, other: &ColorCounts) -> Option<ColorCounts> {
        let mut out = ColorCounts::ZERO;
        for i in 0..NUM_COLORS {
            out.0[i] = self.0[i].checked_add(other.0[i])?;
        }
        Some(out)
    }

    /// Subtract another [ColorCounts] from this one. Returns a value iff the
    /// result does not overflow.
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

    /// Returns the total number of coins.
    fn len(&self) -> usize {
        self.0.iter().sum()
    }

    /// Create an iterator that goes over each coin individually.
    fn iter(&self) -> ColorCountsIter {
        ColorCountsIter {
            i: 0,
            counts: *self,
        }
    }

    /// Attempts to select a token at random. Returns a tuple containing the new
    /// [ColorCounts], with the selected token removed, and the token that was
    /// removed (if any).
    fn random_choice(&self, rand: &mut dyn RandomStream) -> (ColorCounts, Option<Color>) {
        if self.len() == 0 {
            return (*self, None);
        }
        let rand_index = rand.read_usize() % self.len();
        let color = self
            .iter()
            .flat_map(|(color, n)| std::iter::repeat(color).take(n))
            .skip(rand_index)
            .next()
            .unwrap();
        let new_counts = self.clone().minus(&ColorCounts::from(color)).unwrap();
        (new_counts, Some(color))
    }
}

impl From<Color> for ColorCounts {
    fn from(color: Color) -> Self {
        let mut counts = ColorCounts::ZERO;
        counts.0[color as usize] += 1;
        counts
    }
}

impl<const N: usize> From<&[(Color, usize); N]> for ColorCounts {
    fn from(colors: &[(Color, usize); N]) -> Self {
        let mut counts = ColorCounts::ZERO;
        for (color, count) in colors {
            counts.0[*color as usize] += count;
        }
        counts
    }
}

struct ColorCountsIter {
    i: usize,
    counts: ColorCounts,
}

impl Iterator for ColorCountsIter {
    type Item = (Color, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.counts.0.len() {
            let i = self.i;
            self.i += 1;
            Some((Color::from(i), self.counts.0[i]))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Card {
    pub points: usize,
    /// The purchase price of this card.
    pub price: ColorCounts,
    /// The purchasing power this card gives when held. It could be represented
    /// by `Option<Color>`, but `ColorCounts` is more ergonomic for addition and
    /// subtraction.
    pub value: ColorCounts,
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
                game.take_card(card);
                player.hand.hidden.push(card);
                player.tokens = player
                    .tokens
                    .plus(&ColorCounts::from(Color::Yellow))
                    .expect("ColorCounts should not overflow");
            }
            TurnAction::Purchase(card) => {
                game.take_card(card);
                player.hand.face_up.push(card);
            }
        }
        Some(())
    }
}

#[derive(Clone, Copy)]
pub enum PlayerStrategy {
    /// Make choices based on a RNG. I'm not sure whether it makes sense to
    /// uniformly sample the space of actions; this would be heavily weighted
    /// towards purchasing cards. Perhaps this enum variant needs some
    /// parameters.
    Random,
    /// Purchase the highest-value card that the player can afford. Otherwise,
    /// randomly select three tokens.
    GreedyPurchase,
    /// When a high-value card with a single-color cost is placed on the table,
    /// e.g. 7 red tokens, reserve it. Work towards the purchase by taking two
    /// tokens at a time.
    SelectiveReservation,
}

#[derive(Clone)]
struct Player {
    hand: CardRow,
    tokens: ColorCounts,
    strategy: PlayerStrategy,
}

impl Player {
    fn new(strategy: PlayerStrategy) -> Self {
        Player {
            hand: CardRow::new(),
            tokens: ColorCounts::ZERO,
            strategy,
        }
    }

    fn select_action(&mut self, rand: &mut dyn RandomStream, game: &Game) -> TurnAction {
        match self.strategy {
            PlayerStrategy::Random => match rand.read_u8() % 4 {
                0 => {
                    if game.bank.len() < 3 {
                        println!("Cannot take three tokens. Trying again.");
                        return self.select_action(rand, game);
                    }
                    loop {
                        let bank = game.bank;
                        let (bank, color1) = bank.random_choice(rand);
                        let (bank, color2) = bank.random_choice(rand);
                        let (bank, color3) = bank.random_choice(rand);

                        match (color1, color2, color3) {
                            (Some(c1), Some(c2), Some(c3)) => {
                                return TurnAction::TakeThreeTokens(c1, c2, c3);
                            }
                            _ => {}
                        }
                    }
                }
                1 => match game.bank.random_choice(rand) {
                    (_, Some(color)) => TurnAction::TakeTwoTokens(color),
                    _ => {
                        println!("Zero tokens remain. Trying again.");
                        self.select_action(rand, game)
                    }
                },
                2 => TurnAction::Reserve(game.random_card(rand)),
                3 => TurnAction::Purchase(game.random_card(rand)),
                _ => panic!("Unreachable"),
            },
            PlayerStrategy::GreedyPurchase => todo!(),
            PlayerStrategy::SelectiveReservation => todo!(),
        }
    }
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
    fn new_random_game(rand: &mut dyn RandomStream) -> Game {
        Game {
            bank: ColorCounts::BANK_START,
            noble_row: CardRow::new_shuffled(rand, &get_all_nobles(), Game::NUM_CARDS_FACE_UP),
            card_rows: get_all_cards()
                .map(|cards| CardRow::new_shuffled(rand, &cards, Game::NUM_CARDS_FACE_UP)),
        }
    }

    fn random_card(&self, rand: &mut dyn RandomStream) -> Card {
        let row = rand.read_usize() % self.card_rows.len();
        let col = rand.read_usize() % self.card_rows[row].face_up.len();
        self.card_rows[row].face_up[col]
    }

    fn take_card(&mut self, card: Card) {
        for row in self.card_rows.iter_mut() {
            for table_card in row.face_up.iter_mut() {
                if *table_card == card {
                    match row.hidden.pop() {
                        Some(new_card) => {
                            *table_card = new_card;
                        }
                        None => {}
                    }
                    return;
                }
            }
        }
        panic!("Cannot take a card that is not on the table")
    }
}

pub struct Simulation {
    game: Game,
    players: Vec<Player>,
    turn_index: usize,
    winner_index: Option<usize>,
    rand: Box<dyn RandomStream>,
}

impl Simulation {
    pub fn new(strategies: &[PlayerStrategy]) -> Self {
        let mut rand = get_system_random_stream().expect("Should get system random stream");
        Simulation {
            game: Game::new_random_game(rand.as_mut()),
            players: strategies.iter().map(|&s| Player::new(s)).collect(),
            turn_index: 0,
            winner_index: None,
            rand,
        }
    }

    /// Simulate the next player's turn. Returns the unit value iff the game
    /// should continue.
    pub fn step(&mut self) -> Option<()> {
        let num_players = self.players.len();
        let player = &mut self.players[self.turn_index];
        self.turn_index = (self.turn_index + 1) % num_players;

        player
            .select_action(self.rand.as_mut(), &self.game)
            .apply_to(player, &mut self.game)
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

        // Any number added to `usize::MAX` would overflow.
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

        // Any non-zero number subtracted from zero would overflow.
        assert_eq!(ColorCounts::ZERO.minus(&other_money), None);
    }

    #[test]
    fn test_color_counts_get() {
        assert_eq!(ColorCounts::ZERO.get(Color::Red), 0);

        let money = ColorCounts([1, 2, 3, 4, 5, 6]);
        assert_eq!(money.get(Color::Red), 1);
        assert_eq!(money.get(Color::Yellow), 6);
    }
}
