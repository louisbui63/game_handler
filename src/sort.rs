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
    pub fn sort(&mut self, key: fn(&Game, &Game) -> Ordering) {
        if let Some(i) = self.selected {
            self.games.rotate_left(i + 1);
            let upto = self.games.len() - 1;
            let subarray = &mut self.games[0..upto];
            subarray.sort_unstable_by(key);
            let mut j = upto;
            while let Ordering::Less = key(&self.games[j + 1], &self.games[j]) {
                self.games.swap(j, j + 1);
                if j == 0 {
                    break;
                }
                j -= 1;
            }
        } else {
            self.games.sort_unstable_by(key);
        }
    }
}

pub fn sort_none_selected(games: &mut Vec<Game>, key: fn(&Game, &Game) -> Ordering) {
    games.sort_unstable_by(key);
}
