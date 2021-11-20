use notation_bevy_utils::prelude::LayoutEnv;

use crate::prelude::{NotationAppState, NotationSettings, NotationTheme};

pub struct NotationLayout<'a> {
    pub theme: &'a NotationTheme,
    pub state: &'a NotationAppState,
    pub settings: &'a NotationSettings,
}

impl<'a> LayoutEnv for NotationLayout<'a> {
    /*
    fn query_get<'w, Q: WorldQuery>(&self, world: &'w mut World, entity: Entity
    ) -> Result<<Q::Fetch as Fetch<'w>>::Item, QueryEntityError>
    where <Q as WorldQuery>::Fetch: ReadOnlyFetch {
        let mut query = world.query::<Q>();
        query.get(world, entity)
    }
     */
}

impl<'a> NotationLayout<'a> {
    pub fn new(
        theme: &'a NotationTheme,
        state: &'a NotationAppState,
        settings: &'a NotationSettings,
    ) -> Self {
        Self {
            theme,
            state,
            settings,
        }
    }
}
