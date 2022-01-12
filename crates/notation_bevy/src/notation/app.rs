use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::window::WindowResized;

use bevy_asset_loader::AssetLoader;

use crate::theme::theme_colors::UiColors;
use crate::prelude::*;
use super::control_panel::ControlPanel;
use super::tab_viewer::TabViewerPlugin;

#[cfg(feature = "midi")]
use notation_midi::prelude::{
    MidiPlugin,
};

use super::state::{NotationState, TabPathes};

pub struct NotationPlugins;
impl PluginGroup for NotationPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(EguiPlugin);
        group.add(EntryPlugin);
        group.add(MelodyPlugin);
        group.add(LyricsPlugin);
        group.add(BarPlugin);
        group.add(StringsPlugin);
        group.add(ShapesPlugin);
        group.add(MiniPlugin);
        group.add(TabPlugin);
        group.add(PlayPlugin);
        group.add(TabViewerPlugin);

        //crates plugins
        #[cfg(feature = "midi")]
        group.add(MidiPlugin);
        //external plugins
        group.add(bevy_prototype_lyon::prelude::ShapePlugin);
        //group.add(bevy_svg::prelude::SvgPlugin);
    }
}

pub struct NotationApp;

impl NotationApp {
    pub const TITLE: &'static str = "Fun Notation";

    pub fn new_app<A: ExtraAssets>(title: &str) -> App {
        let mut app = App::new();
        AssetLoader::new(NotationAssetsStates::Loading)
            .continue_to_state(NotationAssetsStates::Loaded)
            .with_collection::<NotationAssets>()
            .with_collection::<A>()
            .build(&mut app);
        app.add_state(NotationAssetsStates::Loading)
            .add_startup_system(NotationAssets::setup_keys::<A>);
        Self::insert_window_descriptor(&mut app, String::from(title));
        super::events::add_notation_app_events(&mut app);

        app.insert_resource(Msaa { samples: 4 });
        app.add_plugins(DefaultPlugins);
        app.insert_resource(ClearColor(UiColors::default().app_background));
        app.add_plugin(bevy_easings::EasingsPlugin);

        app.add_plugin(UtilsPlugin);

        app.init_resource::<NotationTheme>();
        app.init_resource::<NotationSettings>();
        app.add_plugins(NotationPlugins);

        //#[cfg(target_arch = "wasm32")]
        //app.add_plugin(bevy_webgl2::WebGL2Plugin);

        // When building for WASM, print panics to the browser console
        #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(crate::wasm::bevy_web_fullscreen::FullViewportPlugin);

        app.add_plugin(bevy_egui::EguiPlugin);
        app.add_plugin(NotationUiPlugin);

        #[cfg(feature = "dev")]
        app.add_plugins(crate::dev::NotationDevPlugins);

        #[cfg(feature = "inspector")]
        app.add_plugins(crate::inspector::NotationInspectorPlugins);

        app
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_tab_from_url() -> Result<String, String> {
        web_sys::window()
            .ok_or("No_Window".to_owned())
            .and_then(|x| x.document().ok_or("No_Document".to_owned()))
            .and_then(|x| x.location().ok_or("No_Location".to_owned()))
            .and_then(|x| x.search().map_err(|e| format!("No_Search:{:?}", e)))
            .map(|x| x.trim_start_matches('?').to_owned())
    }

    pub fn run_with_extra<A, F>(tab_pathes: Vec<String>, extra: F)
    where
        A: ExtraAssets,
        F: Fn(&mut App),
    {
        let mut app = NotationApp::new_app::<A>(Self::TITLE);

        app.insert_resource(TabPathes(tab_pathes));
        app.init_resource::<NotationState>();

        app.add_startup_system(Self::setup_camera);

        #[cfg(debug_assertions)]
        app.add_startup_system(Self::setup_hot_reloading);

        app.add_system_set(
            SystemSet::on_enter(NotationAssetsStates::Loaded)
                .with_system(NotationAssets::add_extra_assets::<A>)
                .with_system(crate::egui::egui_fonts::setup_egui_fonts::<A>)
                .with_system(Self::setup_window_size),
        );
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(ControlPanel::control_ui)
                .with_system(TabViewer::on_add_tab)
                .with_system(TabViewer::on_window_resized)
                .with_system(TabViewer::on_added),
        );
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(Self::on_window_resized)
                .with_system(Self::on_tab_asset),
        );
        extra(&mut app);
        app.run();
    }
    pub fn run<A: ExtraAssets>(tab_pathes: Vec<String>) {
        Self::run_with_extra::<A, _>(tab_pathes, |_app|{})
    }
}

impl NotationApp {
    fn insert_window_descriptor(app: &mut App, title: String) {
        app.insert_resource(WindowDescriptor {
            title,
            //width: 1920.,
            //height: 1080.,
            ..WindowDescriptor::default()
        });
    }

    fn setup_camera(mut commands: Commands) {
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    }

    /*
    * The open tab logic is using some trick in the asset server, which can load from absolute path
    * (outside assets folder), but the hot reloading is not working this way.
    * Ideally can use hot-reloading to update tabs automatically, but that means need to patch bevy
    * to bypass the assumption with asset path under assets folder in reloading.
    *
    * Only enabling for debug build, the hot reloading works really nice for help pages.
    *
    * The crash error is:
    *
    * thread 'Compute Task Pool (2)' panicked at 'called `Result::unwrap()` on an `Err` value: StripPrefixError(())', C:\Users\yjpark\scoop\persist\rustup-msvc\.cargo\registry\src\github.com-1ecc6299db9ec823\bevy_asset-0.5.1\src\io\file_asset_io.rs:135:84
    */
    fn setup_hot_reloading(asset_server: Res<AssetServer>) {
        asset_server.watch_for_changes().unwrap();
    }

    fn on_tab_asset(mut evts: EventReader<AssetEvent<TabAsset>>) {
        for evt in evts.iter() {
            println!("AssetEvent<TabAsset> {:?}", evt);
        }
    }

    fn setup_window_size(window: Res<WindowDescriptor>, mut app_state: ResMut<NotationState>) {
        #[cfg(target_arch = "wasm32")]
        let (width, height) = crate::wasm::bevy_web_fullscreen::get_viewport_size();

        #[cfg(not(target_arch = "wasm32"))]
        let (width, height) = (window.width, window.height);

        println!("setup_window_size(): {} {} ", width, height);
        app_state.window_width = width;
        app_state.window_height = height;
    }

    fn on_window_resized(
        mut window: ResMut<WindowDescriptor>,
        mut evts: EventReader<WindowResized>,
        mut app_state: ResMut<NotationState>,
        mut window_resized_evts: EventWriter<WindowResizedEvent>,
    ) {
        if app_state.tab_path.len() > 0 && app_state.tab.is_none() {
            return;
        }
        for evt in evts.iter() {
            if evt.width as usize != window.width as usize
                || evt.height as usize != window.height as usize
            {
                println!(
                    "on_window_resized(): {} {} -> {} {} ",
                    window.width, window.height, evt.width, evt.height
                );
                let resized_evt = WindowResizedEvent::new(&app_state);
                window.width = evt.width;
                window.height = evt.height;
                app_state.window_width = evt.width;
                app_state.window_height = evt.height;
                app_state.scale_factor_override = window.scale_factor_override;
                window_resized_evts.send(resized_evt);
            }
        }
    }
    pub fn load_tab<F: Fn(String) -> Option<ProtoTab>>(
        commands: &mut Commands,
        time: &Time,
        windows: &mut Windows,
        state: &mut NotationState,
        theme: &mut NotationTheme,
        evts: &mut EventWriter<AddTabEvent>,
        entities_query: &Query<Entity, With<GlobalTransform>>,
        viewer_query: &Query<(Entity, &TabViewer), With<TabViewer>>,
        load_tab: F,
    ) {
        if state.window_width > 0.0
            && state.window_height > 0.0
            && state.tab.is_none()
            && state.parse_error.is_none()
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
            if let Some(tab) = load_tab(state.tab_path.clone()) {
                match Tab::try_parse_arc(tab) {
                    Ok(tab) => {
                        state.tab = Some(tab.clone());
                        if let Some(window) = windows.get_primary_mut() {
                            let title = format!("{} - {}", NotationApp::TITLE, state.tab_path);
                            window.set_title(title);
                        }
                        theme._bypass_systems = false;
                        evts.send(AddTabEvent(tab));
                    }
                    Err(err) => {
                        println!("nload_tab(): Parse Tab Failed: {:?}", err);
                        state.parse_error = Some(err);
                    }
                }
            }
        }
    }
    pub fn load_tab_from_assets(
        asset_server: &AssetServer,
        assets: &Assets<TabAsset>,
        tab_path: String,
    ) -> Option<ProtoTab> {
        let tab_asset: Handle<TabAsset> = asset_server.load(tab_path.as_str());
        if let Some(asset) = assets.get(&tab_asset) {
            Some(asset.tab.clone())
        } else {
            None
        }
    }
}
