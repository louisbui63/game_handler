use icu::collator::options::CollatorOptions;
use icu::collator::*;
use icu::locale::{locale, Locale};
use std::cmp::Ordering;
use std::sync::{LazyLock, Mutex};

use crate::games::Game;

// see https://www.unicode.org/reports/tr35/tr35-collation.html#Setting_Options for how you would
// set options for those
static DEFAULT_LOCALE: Locale = locale!("en-u-kn");
static COLLATOR: LazyLock<Mutex<CollatorBorrowed>> = LazyLock::new(|| {
    let preferences = CollatorPreferences::from_locale_strict(&DEFAULT_LOCALE).unwrap();
    Mutex::new(Collator::try_new(preferences, CollatorOptions::default()).unwrap())
});

pub enum Sorts {
    Name,
    UnicodeName,
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
            Sorts::UnicodeName => move |g: &Game, o: &Game| {
                let l = g.sort_name.as_ref().unwrap_or(&g.name);
                let r = o.sort_name.as_ref().unwrap_or(&o.name);
                (*COLLATOR).lock().unwrap().compare(&l[..], &r[..])
            },
        }
    }
}

impl crate::MainGUI {
    /// sorts the games according to `key` while moving `selected` accordingly if applicable.
    /// FIXME: doesn't work at all
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

fn set_collator_to_default() {
    *(COLLATOR.lock().unwrap()) = Collator::try_new(
        CollatorPreferences::from_locale_strict(&DEFAULT_LOCALE).unwrap(),
        CollatorOptions::default(),
    )
    .unwrap();
}
pub fn set_locale(locale: String) {
    let Ok(l) = locale.parse::<Locale>() else {
        log::error!("locale is invalid, using default instead");
        set_collator_to_default();
        return;
    };
    let Ok(cp) = CollatorPreferences::from_locale_strict(&l) else {
        log::error!("locale contains invalid preferences, using default instead");
        set_collator_to_default();
        return;
    };
    let Ok(co) = Collator::try_new(cp, CollatorOptions::default()) else {
        log::error!("locale cannot result in a valid collator, using default instead");
        set_collator_to_default();
        return;
    };
    *(COLLATOR.lock().unwrap()) = co
}
