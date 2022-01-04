use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::window::WindowResized;

use bevy_asset_loader::AssetLoader;

use crate::theme::theme_colors::UiColors;
use crate::prelude::*;
use super::control_panel::ControlPanel;
use super::tab_viewer::TabViewerPlugin;

use notation_midi::prelude::{
    MidiPlugin,
};

use super::state::{NotationState, TabPathes};

pub struct NotationPlugins;
impl PluginGroup for NotationPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(FontPlugin);
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
        group.add(MidiPlugin);
        //external plugins
        group.add(bevy_prototype_lyon::prelude::ShapePlugin);
        //group.add(bevy_svg::prelude::SvgPlugin);
    }
}

pub struct NotationApp;

impl NotationApp {
    pub const TITLE: &'static str = "Fun Notation";

    pub fn new_builder<A: ExtraAssets>(title: &str) -> AppBuilder {
        let mut app = App::build();
        AssetLoader::new(NotationAssetsStates::Loading)
            .continue_to_state(NotationAssetsStates::Loaded)
            .with_collection::<NotationAssets>()
            .with_collection::<A>()
            .build(&mut app);
        app.add_state(NotationAssetsStates::Loading);
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

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);

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
        F: Fn(&mut AppBuilder),
    {
        let mut app = NotationApp::new_builder::<A>(Self::TITLE);

        app.insert_resource(TabPathes(tab_pathes));
        app.init_resource::<NotationState>();

        app.add_startup_system(Self::setup_camera.system());

        #[cfg(debug_assertions)]
        app.add_startup_system(Self::setup_hot_reloading.system());

        app.add_system_set(
            SystemSet::on_enter(NotationAssetsStates::Loaded)
                .with_system(NotationAssets::add_extra_assets::<A>.system())
                .with_system(Self::setup_window_size.system()),
        );
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(ControlPanel::control_ui.system())
                .with_system(TabViewer::on_add_tab.system())
                .with_system(TabViewer::on_window_resized.system())
                .with_system(TabViewer::on_added.system()),
        );
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(Self::on_window_resized.system())
                .with_system(Self::on_tab_asset.system()),
        );
        extra(&mut app);
        app.run();
    }
    pub fn run<A: ExtraAssets>(tab_pathes: Vec<String>) {
        Self::run_with_extra::<A, _>(tab_pathes, |_app|{})
    }
}

impl NotationApp {
    fn insert_window_descriptor(app: &mut AppBuilder, title: String) {
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
                window.width = evt.width;
                window.height = evt.height;
                app_state.window_width = evt.width;
                app_state.window_height = evt.height;
                app_state.scale_factor_override = window.scale_factor_override;
                window_resized_evts.send(WindowResizedEvent());
            }
        }
    }
}
