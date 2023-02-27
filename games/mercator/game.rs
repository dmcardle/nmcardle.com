use std::fmt::Display;

use crate::cards::{get_all_cards, get_all_nobles, Card, CardRow};
use crate::rand::{get_system_random_stream, RandomStream};
use crate::tokens::{Color, ColorCounts};

#[derive(Debug)]
enum TurnAction {
    TakeThreeTokens(Color, Color, Color),
    TakeTwoTokens(Color),
    Reserve(Card),
    Purchase(Card),
}

impl TurnAction {
    fn apply_to(self, player: &mut Player, game: &mut Game) -> Result<(), String> {
        match self {
            TurnAction::TakeThreeTokens(color1, color2, color3) => {
                let colors = ColorCounts::from(color1)
                    .plus(&ColorCounts::from(color2))
                    .and_then(|c| c.plus(&ColorCounts::from(color3)))?;

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
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Debug)]
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
                    let bank = game.bank;
                    let (bank, color1) = bank.random_choice(rand);
                    let (bank, color2) = bank.random_choice(rand);
                    let (_bank, color3) = bank.random_choice(rand);

                    return match (color1, color2, color3) {
                        (Some(c1), Some(c2), Some(c3)) => TurnAction::TakeThreeTokens(c1, c2, c3),
                        _ => self.select_action(rand, game),
                    };
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

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO implement a more compact display.
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

#[derive(Debug)]
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

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Bank: {}", self.bank)?;
        writeln!(f, "Nobles ({} hidden):", self.noble_row.hidden.len())?;
        for noble in self.noble_row.face_up.iter() {
            writeln!(f, "  {}", noble)?;
        }
        for (i, card_row) in self.card_rows.iter().enumerate() {
            writeln!(f, "L{} cards ({} hidden):", i + 1, card_row.hidden.len())?;
            for card in card_row.face_up.iter() {
                writeln!(f, "  {}", card)?;
            }
        }
        Ok(())
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
            players: strategies.iter().copied().map(Player::new).collect(),
            turn_index: 0,
            winner_index: None,
            rand,
        }
    }

    /// Simulate the next player's turn. Returns the unit value iff the game
    /// should continue.
    pub fn step(&mut self) -> Result<(), String> {
        let num_players = self.players.len();
        let player = &mut self.players[self.turn_index];
        self.turn_index = (self.turn_index + 1) % num_players;

        let action = player.select_action(self.rand.as_mut(), &self.game);
        println!("Player selected {:?}", action);
        action.apply_to(player, &mut self.game)
    }
}

impl Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, player) in self.players.iter().enumerate() {
            writeln!(f, "  Player {}: {}", i, player)?;
        }
        write!(f, "{}", self.game)?;
        if let Some(winner_index) = self.winner_index {
            writeln!(f, "Winner is {}", winner_index)?;
        }
        write!(f, "Player {}'s turn...", self.turn_index)?;
        Ok(())
    }
}
