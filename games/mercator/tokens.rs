use crate::rand::RandomStream;
use std::fmt::Display;

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
    pub const BANK_START: ColorCounts = ColorCounts([7, 7, 7, 7, 7, 5]);

    /// Return the quantity of the given color.
    pub fn get(&self, color: Color) -> usize {
        let index = color as usize;
        self.0[index]
    }

    /// Add another [ColorCounts] to this one. Returns a value iff the result
    /// does not overflow.
    pub fn plus(&self, other: &ColorCounts) -> Result<ColorCounts, String> {
        let mut out = ColorCounts::ZERO;
        for i in 0..NUM_COLORS {
            out.0[i] = self.0[i]
                .checked_add(other.0[i])
                .ok_or("ColorCounts addition overflowed")?;
        }
        Ok(out)
    }

    /// Subtract another [ColorCounts] from this one. Returns a value iff the
    /// result does not overflow.
    pub fn minus(&self, other: &ColorCounts) -> Result<ColorCounts, String> {
        let mut out = ColorCounts::ZERO;
        for i in 0..NUM_COLORS {
            out.0[i] = self.0[i]
                .checked_sub(other.0[i])
                .ok_or("ColorCounts subtraction overflowed")?;
        }
        Ok(out)
    }

    /// Returns the total number of coins.
    pub fn len(&self) -> usize {
        self.0.iter().sum()
    }

    /// Create an iterator that goes over each coin individually.
    pub fn iter(&self) -> ColorCountsIter {
        ColorCountsIter {
            i: 0,
            counts: *self,
        }
    }

    /// Attempts to select a token at random. Returns a tuple containing the new
    /// [ColorCounts], with the selected token removed, and the token that was
    /// removed (if any).
    pub fn random_choice(&self, rand: &mut dyn RandomStream) -> (ColorCounts, Option<Color>) {
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

impl Display for ColorCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_displays: Vec<String> = self
            .0
            .iter()
            .enumerate()
            .filter_map(|(i, n)| match n {
                0 => None,
                _ => Some(format!("{n} {:?}", Color::from(i))),
            })
            .collect();

        write!(f, "{}", color_displays.join(", "))
    }
}

pub struct ColorCountsIter {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_counts_plus() {
        assert_eq!(
            ColorCounts::ZERO.plus(&ColorCounts::ZERO),
            Ok(ColorCounts::ZERO)
        );

        let money = ColorCounts([1, 2, 3, 4, 5, 6]);
        assert_eq!(money.plus(&money), Ok(ColorCounts([2, 4, 6, 8, 10, 12])));

        let other_money = ColorCounts([2, 3, 4, 5, 6, 7]);
        assert_eq!(
            other_money.plus(&money),
            Ok(ColorCounts([3, 5, 7, 9, 11, 13]))
        );

        // Any number added to `usize::MAX` would overflow.
        let max_money = ColorCounts([usize::MAX, 0, 0, 0, 0, 0]);
        assert!(money.plus(&max_money).is_err());
    }

    #[test]
    fn test_color_counts_minus() {
        assert_eq!(
            ColorCounts::ZERO.minus(&ColorCounts::ZERO),
            Ok(ColorCounts::ZERO)
        );

        let money = ColorCounts([1, 2, 3, 4, 5, 6]);
        assert_eq!(money.minus(&money), Ok(ColorCounts::ZERO));

        let other_money = ColorCounts([2, 3, 4, 5, 6, 7]);
        assert_eq!(other_money.minus(&money), Ok(ColorCounts([1; NUM_COLORS])));

        // Any non-zero number subtracted from zero would overflow.
        assert!(ColorCounts::ZERO.minus(&other_money).is_err());
    }

    #[test]
    fn test_color_counts_get() {
        assert_eq!(ColorCounts::ZERO.get(Color::Red), 0);

        let money = ColorCounts([1, 2, 3, 4, 5, 6]);
        assert_eq!(money.get(Color::Red), 1);
        assert_eq!(money.get(Color::Yellow), 6);
    }
}
