use std::fmt::Display;

use crate::cards::{get_all_cards, get_all_nobles, Card, CardRow};
use crate::rand::{get_system_random_stream, RandomStream};
use crate::tokens::{Color, ColorCounts};

#[derive(Debug)]
enum TurnAction {
    /// Take up to three distinct tokens and add them to the player's hand.
    /// Players are allowed to temporarily exceed the max of ten tokens, so long
    /// as they discard down to ten before the end of their turn.
    TakeDistinctTokensAndDiscard(ColorCounts, ColorCounts),
    /// Take two tokens of the same color.
    TakeTwoTokens(Color),
    Reserve(Card),
    Purchase(Card),
}

impl TurnAction {
    fn apply_to(self, player: &mut Player, game: &mut Game) -> Result<(), String> {
        match self {
            TurnAction::TakeDistinctTokensAndDiscard(take, discard) => {
                if take.iter().any(|(_color, n)| n > 1) {
                    return Err(format!("Expected distinct colors, but got {}", take));
                } else if take.get(Color::Yellow) > 0 {
                    return Err("Players cannot take yellow tokens directly".to_string());
                } else if take.len() > 3 {
                    return Err(format!("Cannot take more than three tokens: {}", take));
                }

                let new_bank = game.bank.minus(&take)?;
                let new_player_tokens = player.tokens.plus(&take)?.minus(&discard)?;

                if new_player_tokens.len() > 10 {
                    return Err(format!(
                        "Player would exceed ten tokens if they took {} and discarded {}",
                        take, discard
                    ));
                }
                game.bank = new_bank;
                player.tokens = new_player_tokens;
                Ok(())
            }
            TurnAction::TakeTwoTokens(color) => {
                let colors: ColorCounts = [color, color].into();
                let new_bank = game.bank.minus(&colors)?;
                let new_player_tokens = player.tokens.plus(&colors)?;

                if new_player_tokens.len() > 10 {
                    return Err(format!(
                        "Player would exceed ten tokens if they took {}",
                        colors
                    ));
                }
                game.bank = new_bank;
                player.tokens = new_player_tokens;
                Ok(())
            }
            TurnAction::Reserve(card) => {
                game.take_card(card);
                player.hand.hidden.push(card);
                player.tokens = player
                    .tokens
                    .plus(&ColorCounts::from(Color::Yellow))
                    .expect("ColorCounts should not overflow");

                // TODO Remember to discard tokens if they now exceed 10. Does
                // player get to choose which tokens? If so, that slightly
                // complicates the definition of `TurnAction`.
                Ok(())
            }
            TurnAction::Purchase(card) => {
                // TODO return an error when the player cannot afford this card.

                game.take_card(card);
                player.hand.face_up.push(card);
                Ok(())
            }
        }
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

    fn select_action(
        &mut self,
        rand: &mut dyn RandomStream,
        game: &Game,
    ) -> Result<TurnAction, String> {
        // TODO Return an error if no moves remain.
        // TODO Enumerate all possible moves?

        let no_coins_remain = game.bank.len() == 0;
        let no_cards_remain = game.card_rows.iter().all(|row| row.is_empty());
        let no_reserved_cards = self.hand.hidden.is_empty();

        if no_coins_remain && no_cards_remain && no_reserved_cards {
            return Err("No moves".to_string());
        }

        match self.strategy {
            PlayerStrategy::Random => match rand.read_u8() % 4 {
                0 => {
                    if game.bank.len() == 0 {
                        println!("Cannot take any tokens. Trying again.");
                        return self.select_action(rand, game);
                    }

                    let bank: ColorCounts = game.bank;
                    let (bank, _color1) = bank.random_choice(rand);
                    let (bank, _color2) = bank.random_choice(rand);
                    let (bank, _color3) = bank.random_choice(rand);

                    let take: ColorCounts = game.bank.minus(&bank)?;
                    let new_player_tokens = self.tokens.plus(&take)?;
                    let discard = if new_player_tokens.len() <= 10 {
                        ColorCounts::ZERO
                    } else {
                        let mut discard = ColorCounts::ZERO;
                        let mut new_player_tokens = new_player_tokens;
                        while new_player_tokens.len() > 10 {
                            let (tokens, color) = new_player_tokens.random_choice(rand);
                            new_player_tokens = tokens;
                            discard = discard.plus(&ColorCounts::from(color.unwrap()))?;
                        }
                        discard
                    };

                    Ok(TurnAction::TakeDistinctTokensAndDiscard(take, discard))
                }
                1 => match game.bank.random_choice(rand) {
                    (_, Some(color)) => Ok(TurnAction::TakeTwoTokens(color)),
                    _ => {
                        println!("Zero tokens remain. Trying again.");
                        self.select_action(rand, game)
                    }
                },
                2 => match game.random_card(rand) {
                    Some(card) => Ok(TurnAction::Reserve(card)),
                    None => self.select_action(rand, game),
                },
                3 => match game.random_card(rand) {
                    Some(card) => Ok(TurnAction::Purchase(card)),
                    None => self.select_action(rand, game),
                },
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

    fn random_card(&self, rand: &mut dyn RandomStream) -> Option<Card> {
        let num_visible_cards: usize = self.card_rows.iter().map(|row| row.face_up.len()).sum();
        if num_visible_cards == 0 {
            return None;
        }
        let want_index = rand.read_usize() % num_visible_cards;
        let mut i = 0;
        for row in self.card_rows.iter() {
            for card in row.face_up.iter() {
                if i == want_index {
                    return Some(*card);
                }
                i += 1;
            }
        }
        None
    }

    /// Remove [card] from the table. If possible, replace it with a hidden card
    /// from the appropriate deck.
    fn take_card(&mut self, card: Card) {
        for row in self.card_rows.iter_mut() {
            let mut delete_index = None;

            for (i, table_card) in row.face_up.iter_mut().enumerate() {
                if *table_card == card {
                    match row.hidden.pop() {
                        Some(new_card) => {
                            // Replace the matching table card with a card drawn
                            // from the deck.
                            *table_card = new_card;
                            return;
                        }
                        None => {
                            // Delete the matching table card. We have to defer
                            // this deletion until `row.face_up` is not being
                            // mutably borrowed.
                            delete_index = Some(i);
                        }
                    }
                }
            }

            if let Some(i) = delete_index {
                // Remove the card from its row and shift subsequent cards
                // leftward, since there are no hidden cards to replace it.
                // Although `Vec::remove()` runs in O(n) time, we know n <= 4
                // (`Game::NUM_CARDS_FACE_UP`), so this is effectively O(1).
                row.face_up.remove(i);
                return;
            }
        }
        unreachable!("Cannot take a card that is not on the table")
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

        let action = player.select_action(self.rand.as_mut(), &self.game)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rand::RandomStreamForTest;

    use Color::*;

    #[test]
    fn test_turnaction_take_distinct() {
        let mut player = Player::new(PlayerStrategy::Random);
        let mut game = Game::new_random_game(&mut RandomStreamForTest::new());
        let action =
            TurnAction::TakeDistinctTokensAndDiscard([Red, Green, Blue].into(), ColorCounts::ZERO);
        assert_eq!(action.apply_to(&mut player, &mut game), Ok(()));
    }

    #[test]
    fn test_turnaction_take_distinct_repeated_colors() {
        let mut player = Player::new(PlayerStrategy::Random);
        let mut game = Game::new_random_game(&mut RandomStreamForTest::new());

        let action =
            TurnAction::TakeDistinctTokensAndDiscard([Red, Red, Blue].into(), ColorCounts::ZERO);
        assert!(action.apply_to(&mut player, &mut game).is_err());

        let action =
            TurnAction::TakeDistinctTokensAndDiscard([Red, Blue, Blue].into(), ColorCounts::ZERO);
        assert!(action.apply_to(&mut player, &mut game).is_err());

        let action =
            TurnAction::TakeDistinctTokensAndDiscard([Blue, Blue, Blue].into(), ColorCounts::ZERO);
        assert!(action.apply_to(&mut player, &mut game).is_err());
    }

    #[test]
    fn test_turnaction_take_distinct_wild() {
        let mut player = Player::new(PlayerStrategy::Random);
        let mut game = Game::new_random_game(&mut RandomStreamForTest::new());

        let action =
            TurnAction::TakeDistinctTokensAndDiscard([Yellow, Red, Blue].into(), ColorCounts::ZERO);
        assert!(action.apply_to(&mut player, &mut game).is_err());

        let action = TurnAction::TakeDistinctTokensAndDiscard(
            [Yellow, Yellow, Blue].into(),
            ColorCounts::ZERO,
        );
        assert!(action.apply_to(&mut player, &mut game).is_err());

        let action = TurnAction::TakeDistinctTokensAndDiscard(
            [Yellow, Yellow, Yellow].into(),
            ColorCounts::ZERO,
        );
        assert!(action.apply_to(&mut player, &mut game).is_err());
    }

    /// Test behavior when the player requests more tokens of a particular color
    /// than they are allowed.
    #[test]
    fn test_turnaction_take_distinct_until_insufficient() {
        let mut player = Player::new(PlayerStrategy::Random);
        let mut game = Game::new_random_game(&mut RandomStreamForTest::new());

        // Keep taking until zero red tokens remain.
        while game.bank.get(Red) > 0 {
            assert_eq!(
                TurnAction::TakeDistinctTokensAndDiscard(Red.into(), ColorCounts::ZERO)
                    .apply_to(&mut player, &mut game),
                Ok(())
            );
        }

        assert!(
            TurnAction::TakeDistinctTokensAndDiscard(Red.into(), ColorCounts::ZERO)
                .apply_to(&mut player, &mut game)
                .is_err()
        );
    }

    /// When the player requests more than three tokens, [TurnAction::apply_to]
    /// should fail.
    #[test]
    fn test_turnaction_take_distinct_request_too_many() {
        let mut player = Player::new(PlayerStrategy::Random);
        let mut game = Game::new_random_game(&mut RandomStreamForTest::new());

        let action = TurnAction::TakeDistinctTokensAndDiscard(
            [Red, Green, Blue, White].into(),
            ColorCounts::ZERO,
        );
        assert!(action.apply_to(&mut player, &mut game).is_err());
    }
}
