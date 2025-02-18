use std::cmp::Ordering;

use crate::games::Game;

pub enum Sorts {
    Name,
    ReleaseDate,
}

impl Sorts {
    pub fn get_fn(&self) -> fn(&Game, &Game) -> Ordering {
        match self {
            Self::Name => |g: &Game, o: &Game| g.name.to_uppercase().cmp(&o.name.to_uppercase()),
            Self::ReleaseDate => |g: &Game, o: &Game| {
                g.release_year
                    .unwrap_or(isize::MAX)
                    .cmp(&o.release_year.unwrap_or(isize::MAX))
            },
        }
    }
}

impl crate::MainGUI {
    /// sorts the games according to `key` while moving `selected` accordingly if applicable.
    /// doesn't work at all
    pub fn sort(&mut self, key: fn(&Game, &Game) -> Ordering) {
        if let Some(i) = self.selected {
            let mut indices = (0..self.games.len()).collect::<Vec<_>>();
            indices.sort_unstable_by(|i, j| key(&self.games[*i], &self.games[*j]));
            self.games.sort_unstable_by(key);
            self.selected = Some(indices[i]);
        } else {
            self.games.sort_unstable_by(key);
        }
    }
}

pub fn sort_none_selected(games: &mut Vec<Game>, key: fn(&Game, &Game) -> Ordering) {
    games.sort_unstable_by(key);
}
