use std::fmt::Display;
use std::sync::{Arc};

use notation_proto::prelude::{Chord, Position};

use crate::prelude::{ModelEntry, Tab};
use crate::tab_bar::TabBar;

#[derive(Clone, Debug)]
pub struct TabChord {
    pub chord: Chord,
    pub entries: Vec<Arc<ModelEntry>>,
    pub bars: Vec<Arc<TabBar>>,
}
impl Display for TabChord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<TabChord>({}, E:{}, B:{})",
            self.chord,
            self.entries.len(),
            self.bars.len(),
        )
    }
}

impl TabChord {
    pub fn tab(&self) -> Option<Arc<Tab>> {
        self.entries.first().and_then(|x| x.tab())
    }
    pub fn first_entry(&self) -> Option<Arc<ModelEntry>> {
        self.entries.first().map(|x| x.clone())
    }
    pub fn calc_bars(tab: Option<Arc<Tab>>, chord: Chord) -> Vec<Arc<TabBar>> {
        let mut bars = Vec::new();
        if tab.is_some() {
            for bar in tab.unwrap().bars.iter() {
                if bar.get_chords().contains(&chord) {
                    bars.push(bar.clone());
                }
            }
        }
        bars
    }
    pub fn search_in_bars(
        &self,
        begin_bar_ordinal: usize,
        end_bar_ordinal: usize,
    ) -> Option<Arc<TabBar>> {
        for bar_ordinal in begin_bar_ordinal..=end_bar_ordinal {
            for bar in self.bars.iter() {
                if bar.props.bar_ordinal == bar_ordinal {
                    return Some(bar.clone())
                }
            }
        }
        None
    }
    pub fn search_next(&self, pass_end: bool, position: Option<Position>) -> Option<Arc<TabBar>> {
        if let Some(tab) = self.tab() {
            let last_bar_ordinal = tab.bars.len() + 1;
            match position {
                Some(pos) => {
                    let bar_ordinal = pos.bar.bar_ordinal;
                    if let Some(entry) =
                        self.search_in_bars(bar_ordinal + 1, last_bar_ordinal)
                    {
                        return Some(entry);
                    } else if pass_end {
                        return self.search_in_bars(1, bar_ordinal);
                    }
                }
                None => {
                    return self.search_in_bars(1, last_bar_ordinal);
                }
            }
        }
        None
    }
}