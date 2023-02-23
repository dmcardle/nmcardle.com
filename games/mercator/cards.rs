use crate::game::{Card, Color, ColorCounts};

// TODO: Replace this bogus list of nobles with the real list.
pub fn get_all_nobles() -> [Card; 7] {
    [
        Card {
            points: 4,
            price: ColorCounts::from(&[(Color::Red, 4), (Color::Black, 3)]),
            value: ColorCounts::ZERO,
        },
        Card {
            points: 5,
            price: ColorCounts::from(&[(Color::Red, 7)]),
            value: ColorCounts::ZERO,
        },
        Card {
            points: 4,
            price: ColorCounts::from(&[(Color::Red, 5), (Color::Blue, 3), (Color::White, 3)]),
            value: ColorCounts::ZERO,
        },
        Card {
            points: 5,
            price: ColorCounts::from(&[(Color::Green, 4), (Color::Blue, 4)]),
            value: ColorCounts::ZERO,
        },
        Card {
            points: 5,
            price: ColorCounts::from(&[(Color::Red, 3), (Color::Green, 3), (Color::Blue, 3)]),
            value: ColorCounts::ZERO,
        },
        Card {
            points: 5,
            price: ColorCounts::from(&[(Color::Red, 5), (Color::Blue, 5)]),
            value: ColorCounts::ZERO,
        },
        Card {
            points: 5,
            price: ColorCounts::from(&[(Color::Red, 5), (Color::Black, 5)]),
            value: ColorCounts::ZERO,
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
                value: ColorCounts::from(Color::Black),
            },
            Card {
                points: 1,
                price: ColorCounts::from(&[(Color::Red, 2), (Color::Green, 2)]),
                value: ColorCounts::from(Color::Blue),
            },
            Card {
                points: 0,
                price: ColorCounts::from(&[
                    (Color::Red, 2),
                    (Color::Green, 1),
                    (Color::White, 1),
                    (Color::Black, 1),
                ]),
                value: ColorCounts::from(Color::White),
            },
            Card {
                points: 0,
                price: ColorCounts::from(&[(Color::Blue, 1), (Color::White, 2)]),
                value: ColorCounts::from(Color::Green),
            },
            Card {
                points: 0,
                price: ColorCounts::from(&[(Color::Blue, 1), (Color::Black, 1)]),
                value: ColorCounts::from(Color::Black),
            },
            Card {
                points: 0,
                price: ColorCounts::from(&[(Color::Green, 1), (Color::Black, 1)]),
                value: ColorCounts::from(Color::Black),
            },
        ],
        // Level 2 cards.
        vec![
            Card {
                points: 1,
                price: ColorCounts::from(&[(Color::White, 2), (Color::Black, 2)]),
                value: ColorCounts::from(Color::Black),
            },
            Card {
                points: 2,
                price: ColorCounts::from(&[(Color::Red, 3), (Color::Green, 3)]),
                value: ColorCounts::from(Color::Blue),
            },
            Card {
                points: 1,
                price: ColorCounts::from(&[
                    (Color::Red, 3),
                    (Color::Green, 2),
                    (Color::White, 2),
                    (Color::Black, 2),
                ]),
                value: ColorCounts::from(Color::White),
            },
            Card {
                points: 3,
                price: ColorCounts::from(&[(Color::Blue, 2), (Color::White, 3)]),
                value: ColorCounts::from(Color::Green),
            },
            Card {
                points: 2,
                price: ColorCounts::from(&[(Color::Blue, 2), (Color::Black, 2)]),
                value: ColorCounts::from(Color::Green),
            },
            Card {
                points: 1,
                price: ColorCounts::from(&[(Color::Green, 4)]),
                value: ColorCounts::from(Color::Black),
            },
        ],
        // Level 3 cards.
        vec![
            Card {
                points: 5,
                price: ColorCounts::from(&[(Color::Green, 4), (Color::Black, 4)]),
                value: ColorCounts::from(Color::Black),
            },
            Card {
                points: 6,
                price: ColorCounts::from(&[(Color::Red, 7)]),
                value: ColorCounts::from(Color::Blue),
            },
            Card {
                points: 5,
                price: ColorCounts::from(&[(Color::Red, 5), (Color::Green, 5), (Color::White, 2)]),
                value: ColorCounts::from(Color::White),
            },
            Card {
                points: 7,
                price: ColorCounts::from(&[(Color::Blue, 7)]),
                value: ColorCounts::from(Color::Green),
            },
            Card {
                points: 5,
                price: ColorCounts::from(&[(Color::White, 5), (Color::Black, 1)]),
                value: ColorCounts::from(Color::Green),
            },
            Card {
                points: 6,
                price: ColorCounts::from(&[(Color::Green, 7)]),
                value: ColorCounts::from(Color::Black),
            },
        ],
    ]
}
