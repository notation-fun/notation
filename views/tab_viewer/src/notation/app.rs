use edger_bevy::bevy_prelude::*;
use edger_bevy::bevy::{self, app::PluginGroupBuilder, window::PrimaryWindow, asset::AssetPath};
use edger_bevy::bevy::window::WindowResized;

use bevy_asset_loader::prelude::*;

use crate::theme::theme_colors::UiColors;
use crate::prelude::*;
use super::tab_viewer::TabViewerPlugin;

#[cfg(feature = "with_egui")]
use super::egui_control_panel::EguiControlPanel;

#[cfg(feature = "midi")]
use notation_midi::prelude::{
    MidiPlugin,
};


pub struct NotationPlugins;
impl PluginGroup for NotationPlugins {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>()
            .add(EntryPlugin)
            .add(MelodyPlugin)
            .add(LyricsPlugin)
            .add(BarPlugin)
            .add(MelodyPlugin)
            .add(HarmonyPlugin)
            .add(StringsPlugin)
            .add(ShapesPlugin)
            .add(MiniPlugin)
            .add(TabPlugin)
            .add(PlayPlugin)
            .add(TabViewerPlugin);

        //crates plugins
        #[cfg(feature = "midi")]
        let group = group.add(MidiPlugin);

        group
    }
}

pub struct NotationApp;

impl NotationApp {
    pub const TITLE: &'static str = "Fun Notation";

    fn setup(app: &mut App) {
        app.insert_resource(ClearColor(UiColors::default().app_background));
        app.add_plugins(bevy_easings::EasingsPlugin);

        app.init_resource::<NotationTheme>();
        app.init_resource::<NotationSettings>();
        app.add_plugins(NotationPlugins);

        #[cfg(feature = "dev")]
        app.add_plugins(crate::dev::NotationDevPlugins);
    }

    pub fn run_with_extra<A, F>(args: NotationArgs, extra: F)
    where
        A: PreloadAssets + FromWorld,
        F: Fn(&mut App),
    {
        run_2d_app(|app| {
            app.insert_resource(args.clone());
            Self::setup(app);

            app.init_resource::<NotationState>();

            init_preload_assets::<NotationAssets>(app);
            init_preload_assets::<A>(app);

            #[cfg(feature = "with_egui")]
            add_assets_loaded_systems(app, NotationAssets::setup_egui_context);

            app.add_systems(Update, (
                TabViewer::on_add_tab,
                TabViewer::on_window_resized,
                TabViewer::on_added,
                Self::on_tab_asset,
            ).run_if(in_state(AssetsStates::Loaded)));
            #[cfg(feature = "with_egui")]
            app.add_systems(Update, EguiControlPanel::control_ui
                .run_if(in_state(AssetsStates::Loaded)));

            extra(app);
        });
    }
}

impl NotationApp {
    fn on_tab_asset(mut evts: EventReader<AssetEvent<TabAsset>>) {
        for evt in evts.read() {
            println!("AssetEvent<TabAsset> {:?}", evt);
        }
    }

    pub fn load_tab<F: Fn(&mut Commands, String) -> Option<TabAsset>>(
        commands: &mut Commands,
        time: &Time,
        window_query: &mut Query<&mut Window, With<PrimaryWindow>>,
        app_state: &AppState,
        state: &mut NotationState,
        theme: &mut NotationTheme,
        settings: &NotationSettings,
        evts: &mut EventWriter<AddTabEvent>,
        entities_query: &Query<Entity, With<GlobalTransform>>,
        viewer_query: &Query<(Entity, &TabViewer), With<TabViewer>>,
        load_tab: F,
    ) {
        if app_state.window_width > 0.0
            && app_state.window_height > 0.0
            && state.tab.is_none()
            && state.tab_error.is_none()
        {
            let mut count = 0;
            for _ in entities_query.iter() {
                count += 1;
            }
            //A bit hacky to make sure despawning finished, otherwise might got panic with "Entity not exist"
            if count > 1 {
                if state._despawn_delay_seconds > 0.0 {
                    state._despawn_delay_seconds -= time.delta_seconds();
                    println!(
                        "load_tab(): Waiting to despawn: {} -> {}",
                        count, state._despawn_delay_seconds
                    );
                    return;
                }
                let mut despawn_count = 0;
                for (entity, _viewer) in viewer_query.iter() {
                    commands.entity(entity).despawn_recursive();
                    despawn_count += 1;
                }
                if despawn_count > 0 {
                    println!(
                        "load_tab(): Despawning viewers: {} {}",
                        despawn_count, count
                    );
                } else {
                    println!(
                        "load_tab(): Waiting for entities to be despawned: {}",
                        count
                    );
                }
                return;
            }
            if state._load_tab_delay_seconds > 0.0 {
                state._load_tab_delay_seconds -= time.delta_seconds();
                println!(
                    "load_tab(): Waiting to Load tab: -> {}",
                    state._load_tab_delay_seconds
                );
                return;
            }
            println!("\nload_tab(): Loading: {}", state.tab_path);
            if state.tab_error.is_none() {
                if let Some(tab_asset) = load_tab(commands, state.tab_path.clone()) {
                    match tab_asset.tab {
                        Ok(tab) => {
                            match Tab::try_parse_arc(tab, settings.add_ready_section, state.bars_range) {
                                Ok(tab) => {
                                    state.tab = Some(tab.clone());
                                    if let Ok(mut window) = window_query.get_single_mut() {
                                        let title = format!("{} - {}", NotationApp::TITLE, state.tab_path);
                                        window.title = title;
                                    }
                                    theme._bypass_systems = false;
                                    evts.send(AddTabEvent(tab));
                                }
                                Err(err) => {
                                    println!("nload_tab(): Parse Tab Failed: {:?}", err);
                                    state.tab_error = Some(TabError::ParseFailed(err));
                                }
                            }
                        },
                        Err(err) => {
                            println!("\nload_tab(): Load Failed: {}", err);
                            state.tab_error = Some(err);
                        },
                    }
                } else {
                    println!("\nload_tab(): Not Found: {}", state.tab_path);
                }
            }
        }
    }

    pub fn load_tab_from_assets(
        commands: &mut Commands,
        asset_server: &AssetServer,
        assets: &Assets<TabAsset>,
        tab_path: String,
    ) -> Option<TabAsset> {
        let tab_asset: Handle<TabAsset> = asset_server.load(AssetPath::from(tab_path));
        if let Some(asset) = assets.get(&tab_asset) {
            Some(asset.clone())
        } else {
            commands.insert_resource(TabAssetHandle(tab_asset));
            None
        }
    }
}
