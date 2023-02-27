use std::fmt::Display;

use crate::rand::{shuffle, RandomStream};
use crate::tokens::{Color, ColorCounts};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Card {
    pub points: usize,
    /// The purchase price of this card.
    pub price: ColorCounts,
    /// The purchasing power this card gives when held.
    pub value: Option<Color>,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card: ")?;
        if self.points > 0 {
            write!(f, "+{} points, ", self.points)?;
        }
        if let Some(color) = self.value {
            write!(f, "{:?}, ", color)?;
        }
        write!(f, "[{}].", self.price)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CardRow {
    pub face_up: Vec<Card>,
    pub hidden: Vec<Card>,
}

impl CardRow {
    pub fn new() -> Self {
        CardRow {
            face_up: Vec::new(),
            hidden: Vec::new(),
        }
    }

    pub fn new_shuffled(
        rand: &mut dyn RandomStream,
        cards: &[Card],
        num_face_up: usize,
    ) -> CardRow {
        let mut cards = Vec::from(cards);
        shuffle(rand, &mut cards);
        let (cards_up, cards_down) = cards.split_at(num_face_up);
        CardRow {
            face_up: Vec::from(cards_up),
            hidden: Vec::from(cards_down),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.face_up.is_empty() && self.hidden.is_empty()
    }
}

impl Display for CardRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Face-up: ")?;
        for card in self.face_up.iter() {
            write!(f, "{} ", card)?;
        }
        writeln!(f)?;
        write!(f, "Hidden: ")?;
        for card in self.hidden.iter() {
            write!(f, "{} ", card)?;
        }
        Ok(())
    }
}

// TODO: Replace this bogus list of nobles with the real list.
pub fn get_all_nobles() -> [Card; 7] {
    [
        Card {
            points: 4,
            price: ColorCounts::from(&[(Color::Red, 4), (Color::Black, 3)]),
            value: None,
        },
        Card {
            points: 5,
            price: ColorCounts::from(&[(Color::Red, 7)]),
            value: None,
        },
        Card {
            points: 4,
            price: ColorCounts::from(&[(Color::Red, 5), (Color::Blue, 3), (Color::White, 3)]),
            value: None,
        },
        Card {
            points: 5,
            price: ColorCounts::from(&[(Color::Green, 4), (Color::Blue, 4)]),
            value: None,
        },
        Card {
            points: 5,
            price: ColorCounts::from(&[(Color::Red, 3), (Color::Green, 3), (Color::Blue, 3)]),
            value: None,
        },
        Card {
            points: 5,
            price: ColorCounts::from(&[(Color::Red, 5), (Color::Blue, 5)]),
            value: None,
        },
        Card {
            points: 5,
            price: ColorCounts::from(&[(Color::Red, 5), (Color::Black, 5)]),
            value: None,
        },
    ]
}

/// All cards, grouped by level.
pub fn get_all_cards() -> [Vec<Card>; 3] {
    [
        // Level 1 cards.
        vec![
            Card {
                points: 0,
                price: ColorCounts::from(&[(Color::White, 1), (Color::Black, 1)]),
                value: Some(Color::Black),
            },
            Card {
                points: 1,
                price: ColorCounts::from(&[(Color::Red, 2), (Color::Green, 2)]),
                value: Some(Color::Blue),
            },
            Card {
                points: 0,
                price: ColorCounts::from(&[
                    (Color::Red, 2),
                    (Color::Green, 1),
                    (Color::White, 1),
                    (Color::Black, 1),
                ]),
                value: Some(Color::White),
            },
            Card {
                points: 0,
                price: ColorCounts::from(&[(Color::Blue, 1), (Color::White, 2)]),
                value: Some(Color::Green),
            },
            Card {
                points: 0,
                price: ColorCounts::from(&[(Color::Blue, 1), (Color::Black, 1)]),
                value: Some(Color::Black),
            },
            Card {
                points: 0,
                price: ColorCounts::from(&[(Color::Green, 1), (Color::Black, 1)]),
                value: Some(Color::Black),
            },
        ],
        // Level 2 cards.
        vec![
            Card {
                points: 1,
                price: ColorCounts::from(&[(Color::White, 2), (Color::Black, 2)]),
                value: Some(Color::Black),
            },
            Card {
                points: 2,
                price: ColorCounts::from(&[(Color::Red, 3), (Color::Green, 3)]),
                value: Some(Color::Blue),
            },
            Card {
                points: 1,
                price: ColorCounts::from(&[
                    (Color::Red, 3),
                    (Color::Green, 2),
                    (Color::White, 2),
                    (Color::Black, 2),
                ]),
                value: Some(Color::White),
            },
            Card {
                points: 3,
                price: ColorCounts::from(&[(Color::Blue, 2), (Color::White, 3)]),
                value: Some(Color::Green),
            },
            Card {
                points: 2,
                price: ColorCounts::from(&[(Color::Blue, 2), (Color::Black, 2)]),
                value: Some(Color::Green),
            },
            Card {
                points: 1,
                price: ColorCounts::from(&[(Color::Green, 4)]),
                value: Some(Color::Black),
            },
        ],
        // Level 3 cards.
        vec![
            Card {
                points: 5,
                price: ColorCounts::from(&[(Color::Green, 4), (Color::Black, 4)]),
                value: Some(Color::Black),
            },
            Card {
                points: 6,
                price: ColorCounts::from(&[(Color::Red, 7)]),
                value: Some(Color::Blue),
            },
            Card {
                points: 5,
                price: ColorCounts::from(&[(Color::Red, 5), (Color::Green, 5), (Color::White, 2)]),
                value: Some(Color::White),
            },
            Card {
                points: 7,
                price: ColorCounts::from(&[(Color::Blue, 7)]),
                value: Some(Color::Green),
            },
            Card {
                points: 5,
                price: ColorCounts::from(&[(Color::White, 5), (Color::Black, 1)]),
                value: Some(Color::Green),
            },
            Card {
                points: 6,
                price: ColorCounts::from(&[(Color::Green, 7)]),
                value: Some(Color::Black),
            },
        ],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_fmt() {
        let noble = Card {
            points: 4,
            price: ColorCounts::from(&[(Color::Red, 4), (Color::Black, 3)]),
            value: None,
        };
        assert_eq!(format!("{}", noble), "Card: +4 points, [4 Red, 3 Black].");

        let regular_card = Card {
            points: 5,
            price: ColorCounts::from(&[(Color::Green, 4), (Color::Black, 4)]),
            value: Some(Color::Black),
        };
        assert_eq!(
            format!("{}", regular_card),
            "Card: +5 points, Black, [4 Green, 4 Black]."
        );
    }
}
