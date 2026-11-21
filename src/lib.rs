//! Legacy4J for Horizon.
//!
//! The GUI is ported first. This first slice owns the title screen's drawing,
//! using Legacy4J's original resources and measurements instead of approximated
//! replacement art.

use mod_api::client::hud_layout::PoseOp;
use mod_api::{
    ClientHost, ClientTickPreEvent, DimensionTransition, DimensionTransitionReason,
    DimensionTransitionScreen, DimensionTransitionScreenDrawEvent, DrawContext, Flow, GuiDraw,
    Hooks, Identifier, PackType, RegisterDimensionTransitionScreenEvent, RegistrationContext,
    ScreenKeyPressedPreEvent, ScreenMouseButtonPressedPreEvent, ScreenMouseScrolledPreEvent,
    ScreenRenderPostEvent, ScreenRenderPreEvent, SoundPlay, Text,
};
use serde_json::Value;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

mod audio;
mod client_options;
mod controller;
mod credits;
#[cfg(test)]
mod flight_math;
mod interface;
mod mobcaps;
mod pause_menu;
mod play_menu;
#[cfg(test)]
#[allow(dead_code)]
mod potion_math;

static QUIT_REQUESTED: AtomicBool = AtomicBool::new(false);

pub use client_options::take as take_client_option_change;

/// Consumed by the native event-loop mixin; follows Horizon's normal shutdown path.
pub fn take_quit_request() -> bool {
    QUIT_REQUESTED.swap(false, Ordering::Relaxed)
}

pub use controller::{
    input_active as legacy_controller_input_active, local_server as legacy_controller_local_server,
    look as legacy_controller_look, report_inverted_look as legacy_controller_report_inverted_look,
    set_input as legacy_controller_set_input,
    take_invert_request as legacy_controller_take_invert_request,
    vector as legacy_controller_vector,
};
pub use controller::{
    menu_key as legacy_title_controller_key, title_active as legacy_title_controller_active,
};

fn now_millis() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_millis() as usize)
}

const TITLE_SCREEN: &str = "title";
const LOADING_ENTER_NETHER: &str = "legacy:loading/enter_nether";
const LOADING_ENTER_END: &str = "legacy:loading/enter_end";
const LOADING_LEAVE_NETHER: &str = "legacy:loading/leave_nether";
const LOADING_LEAVE_END: &str = "legacy:loading/leave_end";
const LOADING_RESPAWN: &str = "legacy:loading/respawn";
const PANORAMA_DAY: &str = "legacy:gui/title/panorama_day";
const PANORAMA_WIDTH: f32 = 1230.0;
const PANORAMA_HEIGHT: f32 = 216.0;
const BUTTON_WIDTH: f32 = 225.0;
const BUTTON_HEIGHT: f32 = 20.0;
const BUTTON_GAP: f32 = 5.0;
const BUTTONS: [&str; 5] = [
    "Play Game",
    "Leaderboards",
    "Help & Options",
    "Minecraft Store",
    "Exit Game",
];
const BUTTON_KEYS: [&str; 5] = [
    "legacy.menu.play_game",
    "legacy.menu.leaderboards",
    "",
    "legacy.menu.store",
    "",
];
const HELP_AND_OPTIONS_BUTTONS: [&str; 5] = [
    "Change Skin",
    "How To Play",
    "Controls",
    "Settings",
    "Credits",
];
const HELP_AND_OPTIONS_KEYS: [&str; 5] = [
    "legacy.menu.change_skin",
    "legacy.menu.how_to_play",
    "controls.title",
    "legacy.menu.settings",
    "credits_and_attribution.button.credits",
];
const CONTROLS_BUTTONS: [&str; 3] = ["Mouse Settings", "Key Binds", "Controller"];
const CONTROLS_KEYS: [&str; 3] = [
    "options.mouse_settings.title",
    "controls.keybinds.title",
    "legacy.options.selectedController",
];
const SETTINGS_BUTTONS: [&str; 5] = [
    "Game Options",
    "Audio",
    "Graphics",
    "User Interface",
    "Reset to Defaults",
];
const SETTINGS_KEYS: [&str; 5] = [
    "legacy.menu.game_options",
    "legacy.menu.audio",
    "legacy.menu.graphics",
    "legacy.menu.user_interface",
    "legacy.menu.reset_defaults",
];
const PLAY_GAME_TABS: [&str; 3] = ["Load", "Create", "Join"];
const PLAY_GAME_KEYS: [&str; 3] = ["legacy.menu.load", "legacy.menu.create", "legacy.menu.join"];
const STORE_BUTTONS: [&str; 7] = [
    "STARTERPACKS",
    "Skin Packs",
    "Community Skin Packs",
    "Mash-up Packs",
    "Bundle Packs",
    "Soundtracks",
    "Legacy4J",
];
const CHANGE_SKIN_BUTTONS: [&str; 5] = [
    "TU3 Change Skin Screen: OFF",
    "Smooth Preview Scroll: ON",
    "Hide Armor: OFF",
    "Skin Animations: ON",
    "Custom Pack Options Tooltips: ON",
];
const CREDITS_BUTTONS: [&str; 3] = ["Credits", "Attribution", "Licenses"];
const CREDITS_KEYS: [&str; 3] = [
    "credits_and_attribution.button.credits",
    "credits_and_attribution.button.attribution",
    "credits_and_attribution.button.licenses",
];
const EXIT_BUTTONS: [&str; 2] = ["Cancel", "Quit Game"];
const MOUSE_OPTIONS: [&str; 10] = [
    "Sensitivity: 100%",
    "Scroll Sensitivity: 100%",
    "Invert Mouse X: OFF",
    "Invert Mouse Y: OFF",
    "System Cursor: OFF",
    "Allow Cursor Changes: ON",
    "Cursor At First Inventory Slot: ON",
    "Raw Input: ON",
    "Discrete Scrolling: OFF",
    "Touchscreen Mode: OFF",
];
const KEY_BIND_OPTIONS: [&str; 12] = [
    "Attack/Destroy: Left Button",
    "Pick Block: Middle Button",
    "Use Item/Place Block: Right Button",
    "Move Forward: W",
    "Move Left: A",
    "Move Backward: S",
    "Move Right: D",
    "Jump/Fly Up: Space",
    "Sneak/Fly Down: Left Shift",
    "Sprint: Left Control",
    "Open Inventory: E",
    "Drop Item: Q",
];
const CONTROLLER_OPTIONS: [&str; 9] = [
    "Selected Controller: PS4 Controller",
    "Controller Cursor: ON",
    "Interface Sensitivity: 100%",
    "Invert Look: OFF",
    "Auto Aim: ON",
    "Left Stick Dead Zone: 15%",
    "Right Stick Dead Zone: 34%",
    "Reset Controller Binds",
    "Smooth Movement: ON",
];
const GAME_OPTIONS: [&str; 10] = [
    "Auto Jump: ON",
    "View Bobbing: ON",
    "Flying View Rolling: ON",
    "Hints: ON",
    "Death Messages: ON",
    "Language",
    "Autosave Interval: 15 minutes",
    "Look Sensitivity: 100%",
    "Difficulty: Normal",
    "Announce Advancements: OFF",
];
const AUDIO_OPTIONS: [&str; 4] = [
    "Music: 100%",
    "Sound: 100%",
    "Cave Sounds: ON",
    "Minecart Sounds: ON",
];
const GRAPHICS_OPTIONS: [&str; 4] = [
    "Render Clouds: ON",
    "Skin Animations: ON",
    "Legacy Gamma: 50%",
    "Graphics Preset: Default",
];
const USER_INTERFACE_OPTIONS: [&str; 12] = [
    "Display HUD: ON",
    "Display Hand: ON",
    "Display Game Messages: ON",
    "Autosave Indicator: ON",
    "HUD Opacity: 100%",
    "In-Game Tooltips: ON",
    "Animated Character: ON",
    "Interface Sensitivity: 100%",
    "Online IDs: ON",
    "Classic Crafting: ON",
    "HUD Size: Default",
    "Advanced Options",
];
const RESET_SETTINGS_BUTTONS: [&str; 2] = ["Cancel", "Reset to Defaults"];
const HOW_TO_PLAY_SECTIONS: [&str; 43] = [
    "What's New",
    "Basics",
    "Creative Mode",
    "Inventory",
    "Chests",
    "Crafting",
    "Furnace",
    "Dispenser",
    "Brewing",
    "Enchantment",
    "Anvil",
    "Farming Animals",
    "Breeding Animals",
    "Trading",
    "Horses",
    "Beacons",
    "Fireworks",
    "Banners",
    "Hoppers",
    "Droppers",
    "Nether Portal",
    "The End",
    "End Cities",
    "Elytra",
    "Oceans",
    "Ocean Mobs",
    "Conduit",
    "Scaffolding",
    "Raids",
    "Bees",
    "Piglins",
    "Netherite",
    "Sculk Sensor",
    "The Deep Dark",
    "Archaeology",
    "Armor Trims",
    "Trial Chambers",
    "Crafter",
    "Bundles",
    "The Pale Gardens",
    "Happy Ghast",
    "Copper Golem",
    "Host and Player Options",
];
const HOW_TO_PLAY_IDS: [&str; 43] = [
    "whats_new",
    "basics",
    "creative",
    "inventory",
    "chest",
    "crafting",
    "furnace",
    "dispenser",
    "brewing",
    "enchantment",
    "anvil",
    "farming-animals",
    "breeding-animals",
    "trading",
    "horses",
    "beacon",
    "firework",
    "banner",
    "hopper",
    "dropper",
    "nether-portal",
    "the-end",
    "end-cities",
    "elytra",
    "oceans",
    "ocean-mobs",
    "conduit",
    "scaffolding",
    "raids",
    "bees",
    "piglins",
    "netherite",
    "sculk-sensor",
    "deep-dark",
    "archaeology",
    "armor-trims",
    "trial-chamber",
    "autocrafter",
    "bundle",
    "pale-garden",
    "happy-ghast",
    "copper-golem",
    "host-and-player-options",
];
const HOW_TO_PLAY_KEYS: [&str; 43] = [
    "legacy.menu.how_to_play.whats_new",
    "legacy.menu.how_to_play.basics",
    "legacy.menu.how_to_play.creative",
    "container.inventory",
    "legacy.menu.how_to_play.chests",
    "legacy.menu.how_to_play.crafting",
    "legacy.menu.how_to_play.furnace",
    "legacy.menu.how_to_play.dispenser",
    "legacy.menu.how_to_play.brewing",
    "legacy.menu.how_to_play.enchantment",
    "legacy.menu.how_to_play.anvil",
    "legacy.menu.how_to_play.farming-animals",
    "legacy.menu.how_to_play.breeding-animals",
    "legacy.menu.how_to_play.trading",
    "legacy.menu.how_to_play.horses",
    "legacy.menu.how_to_play.beacons",
    "legacy.menu.how_to_play.fireworks",
    "legacy.menu.how_to_play.banners",
    "legacy.menu.how_to_play.hoppers",
    "legacy.menu.how_to_play.droppers",
    "legacy.menu.how_to_play.nether-portal",
    "legacy.menu.how_to_play.the-end",
    "legacy.menu.how_to_play.end-cities",
    "legacy.menu.how_to_play.elytra",
    "legacy.menu.how_to_play.oceans",
    "legacy.menu.how_to_play.ocean-mobs",
    "legacy.menu.how_to_play.conduit",
    "legacy.menu.how_to_play.scaffolding",
    "legacy.menu.how_to_play.raids",
    "legacy.menu.how_to_play.bees",
    "legacy.menu.how_to_play.piglins",
    "legacy.menu.how_to_play.netherite",
    "legacy.menu.how_to_play.sculk-sensor",
    "legacy.menu.how_to_play.deep-dark",
    "legacy.menu.how_to_play.archaeology",
    "legacy.menu.how_to_play.armor-trims",
    "legacy.menu.how_to_play.trial-chamber",
    "legacy.menu.how_to_play.crafter",
    "legacy.menu.how_to_play.bundle",
    "legacy.menu.how_to_play.pale-garden",
    "legacy.menu.how_to_play.happy-ghast",
    "legacy.menu.how_to_play.copper-golem",
    "legacy.menu.how_to_play.host-and-player-options",
];
const MAX_VISIBLE_LIST_ROWS: usize = 7;

const PAGE_TITLE: usize = 0;
const PAGE_HELP_AND_OPTIONS: usize = 1;
const PAGE_CONTROLS: usize = 2;
const PAGE_SETTINGS: usize = 3;
const PAGE_PLAY_GAME: usize = 4;
const PAGE_LEADERBOARDS: usize = 5;
const PAGE_STORE: usize = 6;
const PAGE_CHANGE_SKIN: usize = 7;
const PAGE_HOW_TO_PLAY: usize = 8;
const PAGE_HOW_TO_PLAY_ARTICLE: usize = 9;
const PAGE_CREDITS: usize = 10;
const PAGE_EXIT: usize = 11;
const PAGE_MOUSE_SETTINGS: usize = 12;
const PAGE_KEY_BINDS: usize = 13;
const PAGE_CONTROLLER: usize = 14;
const PAGE_GAME_OPTIONS: usize = 15;
const PAGE_AUDIO: usize = 16;
const PAGE_GRAPHICS: usize = 17;
const PAGE_USER_INTERFACE: usize = 18;
const PAGE_RESET_SETTINGS: usize = 19;
const PAGE_STORE_CATEGORY: usize = 20;
const PAGE_CREDITS_ROLL: usize = 21;

#[derive(Default)]
pub struct LegacyHorizon {
    page: AtomicUsize,
    keyboard_focus: AtomicUsize,
    how_to_play_section: AtomicUsize,
    article_scroll: AtomicUsize,
    store_category: AtomicUsize,
    crafting_focus: AtomicUsize,
    crafting_scroll: AtomicUsize,
    crafting_request: AtomicUsize,
    crafting_status: AtomicUsize,
    crafting_category: AtomicUsize,
    play_tab: AtomicUsize,
    credits_started: AtomicUsize,
    credits_clock: std::sync::Mutex<credits::RollClock>,
    audio: audio::Options,
    interface: interface::Options,
    mobcaps: mobcaps::Policy,
    pause: pause_menu::PauseMenu,
}

fn menu_button_rects<const N: usize>(size: [f32; 2]) -> [[f32; 4]; N] {
    let x = (size[0] / 2.0 - BUTTON_WIDTH / 2.0).floor();
    let y = (size[1] / 3.0 + 5.0).floor();
    std::array::from_fn(|index| {
        [
            x,
            y + index as f32 * (BUTTON_HEIGHT + BUTTON_GAP),
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
        ]
    })
}

fn submenu_button_rect(size: [f32; 2], index: usize) -> [f32; 4] {
    [
        (size[0] / 2.0).floor() - 112.0,
        (size[1] / 3.0 + 5.0).floor() + index as f32 * (BUTTON_HEIGHT + BUTTON_GAP),
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
    ]
}

fn how_to_play_panel_rect(size: [f32; 2], visible_count: usize) -> [f32; 4] {
    let height = visible_count as f32 * (BUTTON_HEIGHT + BUTTON_GAP) + 24.0;
    [
        ((size[0] - 240.0) / 2.0).floor(),
        ((size[1] - height) / 2.0).floor() + 20.0,
        240.0,
        height,
    ]
}

fn how_to_play_button_rect(size: [f32; 2], index: usize, visible_count: usize) -> [f32; 4] {
    let panel = how_to_play_panel_rect(size, visible_count);
    [
        panel[0] + 8.0,
        panel[1] + 8.0 + index as f32 * (BUTTON_HEIGHT + BUTTON_GAP),
        panel[2] - 16.0,
        BUTTON_HEIGHT,
    ]
}

fn contains(rect: [f32; 4], point: [f32; 2]) -> bool {
    point[0] >= rect[0]
        && point[1] >= rect[1]
        && point[0] < rect[0] + rect[2]
        && point[1] < rect[1] + rect[3]
}

fn play_button_sound(host: &dyn ClientHost) {
    let _ = host.play_local_sound(&SoundPlay::ui("minecraft:ui.button.click"), 0);
}

fn translated_format(host: &dyn ClientHost, key: &str, fallback: &str, value: &str) -> String {
    translated_label(host, key, fallback).replace("%s", value)
}

fn transition_header(
    host: &dyn ClientHost,
    screen: &str,
    _reason: DimensionTransitionReason,
) -> String {
    if screen == LOADING_RESPAWN {
        return translated_label(host, "legacy.menu.respawning", "Respawning");
    }
    match screen {
        LOADING_ENTER_NETHER => translated_format(
            host,
            "legacy.menu.entering",
            "Entering %s",
            &translated_label(host, "dimension.minecraft.the_nether", "The Nether"),
        ),
        LOADING_ENTER_END => translated_format(
            host,
            "legacy.menu.entering",
            "Entering %s",
            &translated_label(host, "dimension.minecraft.the_end", "The End"),
        ),
        LOADING_LEAVE_NETHER => translated_format(
            host,
            "legacy.menu.leaving",
            "Leaving %s",
            &translated_label(host, "dimension.minecraft.the_nether", "The Nether"),
        ),
        LOADING_LEAVE_END => translated_format(
            host,
            "legacy.menu.leaving",
            "Leaving %s",
            &translated_label(host, "dimension.minecraft.the_end", "The End"),
        ),
        _ => String::new(),
    }
}

fn draw_generic_loading(draw: &mut dyn DrawContext, x: f32, y: f32, millis: f32) {
    const BLOCK_SIZE: f32 = 21.0;
    const BLOCK_DISTANCE: f32 = 6.0;
    let block_d = BLOCK_SIZE + BLOCK_DISTANCE;
    let phase = (millis / 4.0).rem_euclid(1000.0);
    for index in 0..8 {
        let v = (index + 1) as f32 * 100.0;
        let n = (index + 3) as f32 * 100.0;
        let alpha = if phase >= v - 100.0 {
            if phase <= v {
                phase / v
            } else {
                (n - phase) / 200.0
            }
        } else {
            0.0
        }
        .clamp(0.0, 1.0);
        if alpha <= 0.0 {
            continue;
        }
        let column = if index <= 2 {
            index
        } else if index >= 4 {
            if index == 7 { 0 } else { 6 - index }
        } else {
            2
        } as f32;
        let row = if index <= 2 {
            0
        } else if index == 3 || index == 7 {
            1
        } else {
            2
        } as f32;
        let rect = [
            x + column * block_d,
            y + row * block_d,
            BLOCK_SIZE,
            BLOCK_SIZE,
        ];
        let alpha_byte = (alpha * 255.0).round() as u32;
        if !draw.sprite(
            "legacy:widget/loading_block",
            [0.0, 0.0, 21.0, 21.0],
            rect,
            Some((alpha_byte << 24) | 0x00FF_FFFF),
        ) {
            draw.fill(rect, (alpha_byte << 24) | 0x00E0_E0E0);
        }
    }
}

fn draw_dimension_loading(
    host: &dyn ClientHost,
    event: &mut DimensionTransitionScreenDrawEvent<'_>,
) {
    let header = transition_header(host, event.screen, event.reason);
    if header.is_empty() {
        return;
    }
    let [width, height] = event.draw.size();
    event
        .draw
        .fill_gradient([0.0, 0.0, width, height], 0xFF18_1818, 0xFF08_0808);

    let header_width = event.draw.text_width(&header) * 2.0;
    let _ = event.draw.gui_paint(&[
        GuiDraw::Push(vec![
            PoseOp::Translate((width - header_width) / 2.0, height / 2.0 - 23.0),
            PoseOp::Scale(2.0, 2.0),
        ]),
        GuiDraw::Text {
            text: Text::literal(header),
            x: 0,
            y: 0,
            color: 0xFFFF_FFFF,
            shadow: true,
        },
        GuiDraw::Pop,
    ]);

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0.0, |duration| duration.as_millis() as f32);
    draw_generic_loading(
        event.draw,
        ((width - 75.0) / 2.0).floor(),
        (height / 2.0).floor(),
        millis,
    );
}

fn draw_title_background(event: &mut ScreenRenderPostEvent<'_>) {
    let size = event.draw.size();

    // LogoRenderer: Minecraft's top 44 of 64 rows. Legacy4J also draws the
    // Java Edition banner here, but Horizon replaces gui/title/edition with
    // its own Rust Edition art. Drawing that fallback would be visibly wrong;
    // use Legacy4J's namespaced banner once Horizon exposes native-mod GUI
    // textures in the title atlas (see HORIZON_BLOCKER.md).
    let logo_x = (size[0] / 2.0).floor() - 128.0;
    let _ = event.draw.sprite(
        "gui/title/minecraft",
        [0.0, 0.0, 1024.0, 176.0],
        [logo_x, 30.0, 256.0, 44.0],
        None,
    );
    let _ = event.draw.sprite(
        "gui/title/edition",
        [0.0, 0.0, 512.0, 56.0],
        [(size[0] / 2.0).floor() - 64.0, 67.0, 128.0, 14.0],
        None,
    );
}

fn translated_label(host: &dyn ClientHost, key: &str, fallback: &str) -> String {
    if key.is_empty() {
        fallback.to_owned()
    } else {
        host.translate(key).unwrap_or_else(|| fallback.to_owned())
    }
}

fn draw_menu(
    host: &dyn ClientHost,
    event: &mut ScreenRenderPostEvent<'_>,
    labels: &[&str],
    keys: &[&str],
    keyboard_focus: usize,
    title_screen: bool,
    title: Option<(&str, &str)>,
) {
    let size = event.draw.size();
    draw_legacy_panorama(event, size);

    if let Some((title_key, title_fallback)) = title {
        let button_x = submenu_button_rect(size, 0)[0];
        let button_y = (size[1] / 3.0 + 5.0).floor();
        let panel = [
            button_x - 8.0,
            button_y - 24.0,
            BUTTON_WIDTH + 16.0,
            labels.len() as f32 * (BUTTON_HEIGHT + BUTTON_GAP) + 24.0,
        ];
        let title = translated_label(host, title_key, title_fallback);
        let title_width = event.draw.text_width(&title);
        event.draw.text(
            &title,
            [(size[0] - title_width) / 2.0, panel[1] + 8.0],
            0xFFFF_FFFF,
            true,
        );
    } else if title_screen {
        draw_title_background(event);
    }

    for (index, label) in labels.iter().enumerate() {
        let rect = if title_screen {
            menu_button_rects::<7>(size)[index]
        } else {
            submenu_button_rect(size, index)
        };
        let highlighted = contains(rect, event.screen.mouse) || index == keyboard_focus;
        let label = translated_label(host, keys.get(index).copied().unwrap_or_default(), label);
        let _ = event
            .draw
            .widget_button(rect, &Text::literal(label), true, highlighted);
    }
}

fn draw_how_to_play_index(
    host: &dyn ClientHost,
    event: &mut ScreenRenderPostEvent<'_>,
    keyboard_focus: usize,
) {
    let size = event.draw.size();
    draw_legacy_panorama(event, size);
    let (start, count) = list_window(keyboard_focus, HOW_TO_PLAY_SECTIONS.len());
    let panel = how_to_play_panel_rect(size, count);
    let title = translated_label(host, "legacy.menu.how_to_play", "How To Play");
    let title_width = event.draw.text_width(&title);
    event.draw.text(
        &title,
        [(size[0] - title_width) / 2.0, panel[1] - 14.0],
        0xFFFF_FFFF,
        true,
    );
    for visible_index in 0..count {
        let index = start + visible_index;
        let rect = how_to_play_button_rect(size, visible_index, count);
        let highlighted = contains(rect, event.screen.mouse) || index == keyboard_focus;
        let label = translated_label(host, HOW_TO_PLAY_KEYS[index], HOW_TO_PLAY_SECTIONS[index]);
        let _ = event
            .draw
            .widget_button(rect, &Text::literal(label), true, highlighted);
    }
}

fn option_page(page: usize) -> Option<(&'static [&'static str], &'static str)> {
    match page {
        PAGE_MOUSE_SETTINGS => Some((&MOUSE_OPTIONS, "Mouse Settings")),
        PAGE_KEY_BINDS => Some((&KEY_BIND_OPTIONS, "Key Binds")),
        PAGE_CONTROLLER => Some((&CONTROLLER_OPTIONS, "Controller")),
        PAGE_GAME_OPTIONS => Some((&GAME_OPTIONS, "Game Options")),
        PAGE_AUDIO => Some((&AUDIO_OPTIONS, "Audio")),
        PAGE_GRAPHICS => Some((&GRAPHICS_OPTIONS, "Graphics")),
        PAGE_USER_INTERFACE => Some((&USER_INTERFACE_OPTIONS, "User Interface")),
        _ => None,
    }
}

fn option_panel_rect(size: [f32; 2]) -> [f32; 4] {
    [
        ((size[0] - 250.0) / 2.0).floor(),
        ((size[1] - 215.0) / 2.0).floor() + 20.0,
        250.0,
        215.0,
    ]
}

fn option_button_rect(size: [f32; 2], visible_index: usize) -> [f32; 4] {
    let panel = option_panel_rect(size);
    [
        panel[0] + 15.0,
        panel[1] + 18.0 + visible_index as f32 * 25.0,
        panel[2] - 30.0,
        BUTTON_HEIGHT,
    ]
}

fn draw_option_page(
    host: &dyn ClientHost,
    event: &mut ScreenRenderPostEvent<'_>,
    labels: &[&str],
    title: &str,
    keyboard_focus: usize,
    page: usize,
    audio: &audio::Options,
    interface: &interface::Options,
) {
    let size = event.draw.size();
    draw_legacy_panorama(event, size);
    let panel = option_panel_rect(size);
    let title_width = event.draw.text_width(title);
    event.draw.text(
        title,
        [(size[0] - title_width) / 2.0, panel[1] - 14.0],
        0xFFFF_FFFF,
        true,
    );
    let (start, count) = list_window(keyboard_focus, labels.len());
    for visible_index in 0..count {
        let index = start + visible_index;
        let rect = option_button_rect(size, visible_index);
        let highlighted = contains(rect, event.screen.mouse) || index == keyboard_focus;
        let label = option_label(host, page, index, labels[index], audio, interface);
        let enabled = (page == PAGE_AUDIO && index >= 2)
            || (page == PAGE_CONTROLLER && index == 3)
            || vanilla_bool_setting(host, page, index).is_some()
            || interface_setting(page, index).is_some();
        let _ = event
            .draw
            .widget_button(rect, &Text::literal(label), enabled, highlighted);
    }
    if start > 0 {
        event.draw.text(
            "▲",
            [panel[0] + panel[2] - 16.0, panel[1] + 4.0],
            0xFFFF_FFFF,
            true,
        );
    }
    if start + count < labels.len() {
        event.draw.text(
            "▼",
            [panel[0] + panel[2] - 16.0, panel[1] + panel[3] - 13.0],
            0xFFFF_FFFF,
            true,
        );
    }
}

fn on_off(on: bool) -> &'static str {
    if on { "ON" } else { "OFF" }
}

fn vanilla_bool_setting(
    host: &dyn ClientHost,
    page: usize,
    index: usize,
) -> Option<(&'static str, bool)> {
    let input = host.input_options();
    Some(match (page, index) {
        (PAGE_MOUSE_SETTINGS, 2) => ("invertXMouse", input.invert_x),
        (PAGE_MOUSE_SETTINGS, 3) => ("invertYMouse", input.invert_y),
        (PAGE_MOUSE_SETTINGS, 8) => ("discrete_mouse_scroll", input.discrete_scroll),
        (PAGE_GAME_OPTIONS, 0) => ("autoJump", input.auto_jump),
        (PAGE_GAME_OPTIONS, 1) => ("bobView", host.graphics_options().view_bobbing),
        _ => return None,
    })
}

fn interface_setting(page: usize, index: usize) -> Option<interface::Setting> {
    use interface::Setting::*;
    match (page, index) {
        (PAGE_USER_INTERFACE, 0) => Some(Hud),
        (PAGE_USER_INTERFACE, 1) => Some(Hand),
        (PAGE_USER_INTERFACE, 2) => Some(GameMessages),
        (PAGE_GAME_OPTIONS, 4) => Some(DeathMessages),
        (PAGE_GAME_OPTIONS, 9) => Some(AdvancementMessages),
        (PAGE_CONTROLLER, 8) => Some(SmoothControllerMovement),
        _ => None,
    }
}

fn option_label(
    host: &dyn ClientHost,
    page: usize,
    index: usize,
    fallback: &str,
    audio: &audio::Options,
    interface: &interface::Options,
) -> String {
    let input = host.input_options();
    if let Some(setting) = interface_setting(page, index) {
        return format!(
            "{}: {}",
            fallback.split(':').next().unwrap_or(fallback),
            on_off(interface.enabled(setting))
        );
    }
    let value = match (page, index) {
        (PAGE_CONTROLLER, 3) => Some(on_off(controller::inverted_look()).into()),
        (PAGE_CONTROLLER, 5) => Some("25%".into()),
        (PAGE_CONTROLLER, 6) => Some("34%".into()),
        (PAGE_MOUSE_SETTINGS, 0) | (PAGE_GAME_OPTIONS, 7) => {
            Some(format!("{}%", (input.sensitivity * 200.0).round()))
        }
        (PAGE_MOUSE_SETTINGS, 1) => Some(format!(
            "{}%",
            (input.mouse_wheel_sensitivity * 100.0).round()
        )),
        (PAGE_MOUSE_SETTINGS, 2) => Some(on_off(input.invert_x).into()),
        (PAGE_MOUSE_SETTINGS, 3) => Some(on_off(input.invert_y).into()),
        (PAGE_MOUSE_SETTINGS, 8) => Some(on_off(input.discrete_scroll).into()),
        (PAGE_GAME_OPTIONS, 0) => Some(on_off(input.auto_jump).into()),
        (PAGE_GAME_OPTIONS, 1) => Some(on_off(host.graphics_options().view_bobbing).into()),
        (PAGE_AUDIO, 1) => Some(format!(
            "{}%",
            (host.audio_options().master_volume * 100.0).round()
        )),
        (PAGE_AUDIO, 2..=3) => Some(on_off(audio.enabled(index - 2)).into()),
        _ => None,
    };
    if page == PAGE_KEY_BINDS {
        const KEYS: [&str; 12] = [
            "key.attack",
            "key.pickItem",
            "key.use",
            "key.forward",
            "key.left",
            "key.back",
            "key.right",
            "key.jump",
            "key.sneak",
            "key.sprint",
            "key.inventory",
            "key.drop",
        ];
        if let Ok(Some(mapping)) = host.mapping(KEYS[index]) {
            if let Ok(label) = host.text_string(&mapping.label) {
                return format!(
                    "{}: {}",
                    fallback.split(':').next().unwrap_or(fallback),
                    label
                );
            }
        }
    }
    let name = fallback.split(':').next().unwrap_or(fallback);
    match value {
        Some(value) => format!("{name}: {value}"),
        None => format!("{name}: Unavailable"),
    }
}

fn crafting_recipe_rect(panel: [f32; 4], visible_index: usize) -> [f32; 4] {
    let column = visible_index % 5;
    let row = visible_index / 5;
    [
        panel[0] + 9.0 + column as f32 * 28.0,
        panel[1] + 40.0 + row as f32 * 28.0,
        27.0,
        27.0,
    ]
}

fn crafting_category_matches(recipe: &mod_api::Recipe, category: usize) -> bool {
    if recipe.results.is_empty() {
        return false;
    }
    let category_name = recipe
        .json
        .get("category")
        .and_then(|value| value.as_str())
        .unwrap_or("misc");
    category_name == ["building", "redstone", "equipment", "misc"][category.min(3)]
}

fn crafting_browser_fits(stock_left: f32) -> bool {
    stock_left >= 164.0
}

fn crafting_tab_rect(panel: [f32; 4], index: usize) -> [f32; 4] {
    [
        panel[0] + 13.0 + index as f32 * 34.0,
        panel[1] + 22.0,
        30.0,
        17.0,
    ]
}

fn draw_crafting_menu(
    host: &dyn ClientHost,
    event: &mut ScreenRenderPostEvent<'_>,
    focus: usize,
    scroll: usize,
    status: usize,
    category: usize,
) {
    let recipes = host.recipes_of_type("minecraft:crafting");
    let visible: Vec<_> = recipes
        .iter()
        .filter(|recipe| crafting_category_matches(recipe, category))
        .collect();
    let [width, height] = event.draw.size();
    let stock_left = event.screen.rect[0];
    if !crafting_browser_fits(stock_left) {
        event.draw.text(
            "Recipe browser needs a smaller GUI scale",
            [8.0, height - 18.0],
            0xFFFF_FFFF,
            true,
        );
        return;
    }
    let panel_x = if stock_left >= 164.0 {
        stock_left - 160.0
    } else {
        4.0
    };
    let panel = [panel_x, ((height - 215.0) / 2.0).floor(), 154.0, 215.0];
    let _ = host.apply_screen_blur();
    play_menu::bevel(event.draw, panel, false);
    event
        .draw
        .fill([panel[0], panel[1], panel[2], 1.0], 0xFF70_7070);
    event.draw.fill(
        [panel[0], panel[1] + panel[3] - 1.0, panel[2], 1.0],
        0xFF30_3030,
    );
    let title = ["Building", "Redstone", "Equipment", "Miscellaneous"][category.min(3)];
    let title_width = event.draw.text_width(title);
    event.draw.text(
        title,
        [panel[0] + (panel[2] - title_width) / 2.0, panel[1] + 7.0],
        0xFF48_4848,
        false,
    );

    for (index, id) in [
        "minecraft:bricks",
        "minecraft:redstone",
        "minecraft:golden_sword",
        "minecraft:lava_bucket",
    ]
    .iter()
    .enumerate()
    {
        let tab = crafting_tab_rect(panel, index);
        event.draw.fill(
            tab,
            if index == category {
                0xFF9A_AC96
            } else {
                0xFFAA_AAAA
            },
        );
        if let Ok(identifier) = Identifier::parse(id) {
            if let Some(item) = host.item_key(&identifier) {
                let _ = event.draw.item(item, [tab[0] + 7.0, tab[1]], 16.0);
            }
        }
    }

    if visible.is_empty() {
        event.draw.text(
            "No recipes in this category",
            [panel[0] + 5.0, panel[1] + 50.0],
            0xFF48_4848,
            false,
        );
        return;
    }
    let start = scroll.min(visible.len().saturating_sub(1));
    let count = (visible.len() - start).min(10);
    for visible_index in 0..count {
        let recipe_index = start + visible_index;
        let rect = crafting_recipe_rect(panel, visible_index);
        let highlighted = contains(rect, event.screen.mouse) || recipe_index == focus;
        event.draw.fill(
            rect,
            if highlighted {
                0xFF8A_8A8A
            } else {
                0xFF45_4545
            },
        );
        event
            .draw
            .fill([rect[0], rect[1], rect[2], 1.0], 0xFFB0_B0B0);
        let _ = event.draw.item_stack(
            &visible[recipe_index].results[0],
            [rect[0] + 5.5, rect[1] + 5.5],
            16.0,
        );
    }

    let selected = focus.min(visible.len() - 1);
    let recipe = visible[selected];
    let result = &recipe.results[0];
    let name = host.hover_name(result);
    let name_width = event.draw.text_width(&name);
    event.draw.text(
        &name,
        [
            panel[0] + ((panel[2] - name_width) / 2.0).max(4.0),
            panel[1] + 102.0,
        ],
        0xFF48_4848,
        false,
    );
    event.draw.fill(
        [panel[0] + 8.0, panel[1] + 115.0, panel[2] - 16.0, 91.0],
        0xFFB0_B0B0,
    );
    for (index, ingredient) in recipe.ingredients.iter().take(9).enumerate() {
        let x = panel[0] + 17.0 + (index % 3) as f32 * 24.0;
        let y = panel[1] + 124.0 + (index / 3) as f32 * 24.0;
        event.draw.fill([x, y, 20.0, 20.0], 0xFF45_4545);
        if let Some(id) = ingredient.order.first() {
            if let Ok(identifier) = Identifier::parse(id) {
                if let Some(item) = host.item_key(&identifier) {
                    let _ = event.draw.item(item, [x + 2.0, y + 2.0], 16.0);
                }
            }
        }
    }
    let footer = match status {
        2 => "Recipe placed",
        3 => "Recipe could not be placed",
        4 => "Recipe transfer requested",
        _ => "Select recipe",
    };
    event.draw.text(
        footer,
        [panel[0] + 8.0, panel[1] + panel[3] - 9.0],
        0xFF48_4848,
        false,
    );
    let _ = width;
}

fn list_window(selected: usize, count: usize) -> (usize, usize) {
    let selected = selected.min(count.saturating_sub(1));
    let start = selected
        .saturating_sub(MAX_VISIBLE_LIST_ROWS - 1)
        .min(count.saturating_sub(MAX_VISIBLE_LIST_ROWS));
    (start, (count - start).min(MAX_VISIBLE_LIST_ROWS))
}

fn scroll_list_focus(selected: usize, count: usize, scroll_y: f64) -> usize {
    if scroll_y < 0.0 {
        (selected + 1).min(count.saturating_sub(1))
    } else if scroll_y > 0.0 {
        selected.saturating_sub(1)
    } else {
        selected
    }
}

fn draw_panel_message(event: &mut ScreenRenderPostEvent<'_>, title: &str, message: &str) {
    let [width, height] = event.draw.size();
    draw_legacy_panorama(event, [width, height]);
    let rect = [28.0, 92.0, width - 56.0, height - 122.0];
    play_menu::bevel(event.draw, rect, false);
    let title_width = event.draw.text_width(title);
    event.draw.text(
        title,
        [(width - title_width) / 2.0, 78.0],
        0xFFFF_FFFF,
        true,
    );
    event.draw.text(
        message,
        [rect[0] + 12.0, rect[1] + 12.0],
        0xFF48_4848,
        false,
    );
}

fn confirmation_button_rect(size: [f32; 2], index: usize) -> [f32; 4] {
    let x = ((size[0] - 230.0) / 2.0).floor();
    let y = ((size[1] - 145.0) / 2.0).floor();
    [
        x + 15.0,
        y + 145.0 - 52.0 + index as f32 * 22.0,
        200.0,
        20.0,
    ]
}

fn draw_confirmation(
    host: &dyn ClientHost,
    event: &mut ScreenRenderPostEvent<'_>,
    reset: bool,
    focus: usize,
) {
    let size = event.draw.size();
    draw_legacy_panorama(event, size);
    let panel = [
        ((size[0] - 230.0) / 2.0).floor(),
        ((size[1] - 145.0) / 2.0).floor(),
        230.0,
        145.0,
    ];
    play_menu::bevel(event.draw, panel, false);
    let title = translated_label(
        host,
        if reset {
            "legacy.menu.reset_settings"
        } else {
            "menu.quit"
        },
        if reset { "Reset Settings" } else { "Exit Game" },
    );
    event.draw.text(
        &title,
        [panel[0] + 15.0, panel[1] + 15.0],
        0xFF48_4848,
        false,
    );
    let message = translated_label(
        host,
        if reset {
            "legacy.menu.reset_message"
        } else {
            "legacy.menu.gameExitMessage"
        },
        if reset {
            "Are you sure you would like to reset your settings to their default values?"
        } else {
            "Are you sure you want to exit the game?"
        },
    );
    for (index, line) in wrap_text(event, &message, 200.0).iter().enumerate() {
        event.draw.text(
            line,
            [panel[0] + 15.0, panel[1] + 35.0 + index as f32 * 12.0],
            0xFF48_4848,
            false,
        );
    }
    for (index, label) in [
        "Cancel",
        if reset {
            "Reset to Defaults"
        } else {
            "Quit Game"
        },
    ]
    .iter()
    .enumerate()
    {
        let rect = confirmation_button_rect(size, index);
        let _ = event.draw.widget_button(
            rect,
            &Text::literal(*label),
            index == 0 || !reset,
            focus == index || contains(rect, event.screen.mouse),
        );
    }
}

fn collect_message_keys(value: &Value, keys: &mut Vec<String>) {
    match value {
        Value::Object(object) => {
            if let Some(key) = object.get("translate").and_then(Value::as_str)
                && key.contains("_message")
                && !keys.iter().any(|existing| existing == key)
            {
                keys.push(key.to_owned());
            }
            for child in object.values() {
                collect_message_keys(child, keys);
            }
        }
        Value::Array(values) => {
            for child in values {
                collect_message_keys(child, keys);
            }
        }
        _ => {}
    }
}

fn strip_legacy_formatting(text: &str) -> String {
    let mut plain = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '§' {
            let _ = chars.next();
        } else {
            plain.push(c);
        }
    }
    plain.replace("%s", "□")
}

fn how_to_play_text(host: &dyn ClientHost, section: usize) -> String {
    let section = section.min(HOW_TO_PLAY_IDS.len() - 1);
    let resource = format!(
        "legacy:ui_definitions/how_to_play/{}.json",
        HOW_TO_PLAY_IDS[section]
    );
    let Ok(id) = Identifier::parse(&resource) else {
        return String::new();
    };
    let Some(bytes) = host.resource(PackType::ClientResources, &id) else {
        return String::new();
    };
    let Ok(value) = serde_json::from_slice::<Value>(&bytes) else {
        return String::new();
    };
    let mut keys = Vec::new();
    collect_message_keys(&value, &mut keys);
    keys.into_iter()
        .filter_map(|key| host.translate(&key))
        .map(|text| strip_legacy_formatting(&text))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn wrap_text(event: &ScreenRenderPostEvent<'_>, text: &str, width: f32) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.lines() {
        if paragraph.is_empty() {
            lines.push(String::new());
            continue;
        }
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            let candidate = if line.is_empty() {
                word.to_owned()
            } else {
                format!("{line} {word}")
            };
            if !line.is_empty() && event.draw.text_width(&candidate) > width {
                lines.push(std::mem::take(&mut line));
                line.push_str(word);
            } else {
                line = candidate;
            }
        }
        if !line.is_empty() {
            lines.push(line);
        }
    }
    lines
}

fn draw_how_to_play_article(
    host: &dyn ClientHost,
    event: &mut ScreenRenderPostEvent<'_>,
    section: usize,
    scroll: usize,
) {
    let [width, height] = event.draw.size();
    draw_legacy_panorama(event, [width, height]);
    let section = section.min(HOW_TO_PLAY_SECTIONS.len() - 1);
    let rect = [28.0, 92.0, width - 56.0, height - 122.0];
    let title = translated_label(
        host,
        HOW_TO_PLAY_KEYS[section],
        HOW_TO_PLAY_SECTIONS[section],
    );
    let title_width = event.draw.text_width(&title);
    event.draw.text(
        &title,
        [(width - title_width) / 2.0, 78.0],
        0xFF80_FF80,
        true,
    );
    let text = how_to_play_text(host, section);
    let fallback = "This Legacy4J help page has no readable text in the active resource packs.";
    let lines = wrap_text(
        event,
        if text.is_empty() { fallback } else { &text },
        rect[2] - 24.0,
    );
    let max_lines = ((rect[3] - 24.0) / 10.0).max(1.0) as usize;
    for (row, line) in lines.iter().skip(scroll).take(max_lines).enumerate() {
        event.draw.text(
            line,
            [rect[0] + 12.0, rect[1] + 10.0 + row as f32 * 10.0],
            0xFFFF_FFFF,
            true,
        );
    }
    let arrow_y = rect[1] + rect[3] + 3.0;
    if scroll > 0
        && !event.draw.sprite(
            "legacy:widget/scroll_up",
            [0.0, 0.0, 13.0, 7.0],
            [rect[0] + rect[2] - 29.0, arrow_y, 13.0, 7.0],
            None,
        )
    {
        event.draw.quad(
            [
                [rect[0] + rect[2] - 22.5, arrow_y],
                [rect[0] + rect[2] - 29.0, arrow_y + 7.0],
                [rect[0] + rect[2] - 16.0, arrow_y + 7.0],
                [rect[0] + rect[2] - 22.5, arrow_y],
            ],
            0xFFFF_FFFF,
        );
    }
    if scroll + max_lines < lines.len()
        && !event.draw.sprite(
            "legacy:widget/scroll_down",
            [0.0, 0.0, 13.0, 7.0],
            [rect[0] + rect[2] - 13.0, arrow_y, 13.0, 7.0],
            None,
        )
    {
        event.draw.quad(
            [
                [rect[0] + rect[2] - 13.0, arrow_y],
                [rect[0] + rect[2], arrow_y],
                [rect[0] + rect[2] - 6.5, arrow_y + 7.0],
                [rect[0] + rect[2] - 13.0, arrow_y],
            ],
            0xFFFF_FFFF,
        );
    }
}

fn draw_page(
    host: &dyn ClientHost,
    event: &mut ScreenRenderPostEvent<'_>,
    page: usize,
    keyboard_focus: usize,
    how_to_play_section: usize,
    article_scroll: usize,
    store_category: usize,
    play_tab: usize,
    audio: &audio::Options,
    interface: &interface::Options,
) {
    match page {
        PAGE_HELP_AND_OPTIONS => draw_menu(
            host,
            event,
            &HELP_AND_OPTIONS_BUTTONS,
            &HELP_AND_OPTIONS_KEYS,
            keyboard_focus,
            false,
            None,
        ),
        PAGE_CONTROLS => draw_menu(
            host,
            event,
            &CONTROLS_BUTTONS,
            &CONTROLS_KEYS,
            keyboard_focus,
            false,
            None,
        ),
        PAGE_SETTINGS => draw_menu(
            host,
            event,
            &SETTINGS_BUTTONS,
            &SETTINGS_KEYS,
            keyboard_focus,
            false,
            None,
        ),
        PAGE_PLAY_GAME => play_menu::draw(host, event, play_tab),
        PAGE_LEADERBOARDS => draw_panel_message(
            event,
            "Leaderboards",
            "Overall                         My Score",
        ),
        PAGE_STORE => draw_menu(
            host,
            event,
            &STORE_BUTTONS,
            &[],
            keyboard_focus,
            false,
            Some(("legacy.menu.store", "Minecraft Store")),
        ),
        PAGE_CHANGE_SKIN => draw_menu(
            host,
            event,
            &CHANGE_SKIN_BUTTONS,
            &[],
            keyboard_focus,
            false,
            Some(("legacy.menu.change_skin", "Change Skin")),
        ),
        PAGE_HOW_TO_PLAY => draw_how_to_play_index(host, event, keyboard_focus),
        PAGE_HOW_TO_PLAY_ARTICLE => {
            draw_how_to_play_article(host, event, how_to_play_section, article_scroll)
        }
        PAGE_CREDITS => draw_menu(
            host,
            event,
            &CREDITS_BUTTONS,
            &CREDITS_KEYS,
            keyboard_focus,
            false,
            None,
        ),
        PAGE_EXIT | PAGE_RESET_SETTINGS => {
            draw_confirmation(host, event, page == PAGE_RESET_SETTINGS, keyboard_focus)
        }
        PAGE_STORE_CATEGORY => {
            let category = STORE_BUTTONS[store_category.min(STORE_BUTTONS.len() - 1)];
            draw_panel_message(
                event,
                category,
                "No installed Legacy4J store content was found in this category.",
            );
        }
        page if option_page(page).is_some() => {
            let (labels, title) = option_page(page).expect("checked above");
            draw_option_page(
                host,
                event,
                labels,
                title,
                keyboard_focus,
                page,
                audio,
                interface,
            );
        }
        _ => draw_menu(
            host,
            event,
            &BUTTONS,
            &BUTTON_KEYS,
            keyboard_focus,
            true,
            None,
        ),
    }
}

fn page_button_count(page: usize) -> usize {
    match page {
        PAGE_HELP_AND_OPTIONS => HELP_AND_OPTIONS_BUTTONS.len(),
        PAGE_CONTROLS => CONTROLS_BUTTONS.len(),
        PAGE_SETTINGS => SETTINGS_BUTTONS.len(),
        PAGE_PLAY_GAME => PLAY_GAME_TABS.len(),
        PAGE_STORE => STORE_BUTTONS.len(),
        PAGE_CHANGE_SKIN => CHANGE_SKIN_BUTTONS.len(),
        PAGE_HOW_TO_PLAY => HOW_TO_PLAY_SECTIONS.len(),
        PAGE_HOW_TO_PLAY_ARTICLE | PAGE_LEADERBOARDS | PAGE_CREDITS_ROLL => 0,
        PAGE_CREDITS => CREDITS_BUTTONS.len(),
        PAGE_EXIT => EXIT_BUTTONS.len(),
        PAGE_RESET_SETTINGS => RESET_SETTINGS_BUTTONS.len(),
        PAGE_STORE_CATEGORY => 0,
        page if option_page(page).is_some() => {
            option_page(page).map_or(0, |(labels, _)| labels.len())
        }
        _ => BUTTONS.len(),
    }
}

fn parent_page(page: usize) -> Option<(usize, usize)> {
    match page {
        PAGE_HELP_AND_OPTIONS => Some((PAGE_TITLE, 2)),
        PAGE_CONTROLS => Some((PAGE_HELP_AND_OPTIONS, 2)),
        PAGE_SETTINGS => Some((PAGE_HELP_AND_OPTIONS, 3)),
        PAGE_PLAY_GAME => Some((PAGE_TITLE, 0)),
        PAGE_LEADERBOARDS => Some((PAGE_TITLE, 1)),
        PAGE_STORE => Some((PAGE_TITLE, 3)),
        PAGE_CHANGE_SKIN => Some((PAGE_HELP_AND_OPTIONS, 0)),
        PAGE_HOW_TO_PLAY => Some((PAGE_HELP_AND_OPTIONS, 1)),
        PAGE_HOW_TO_PLAY_ARTICLE => Some((PAGE_HOW_TO_PLAY, 0)),
        PAGE_CREDITS => Some((PAGE_HELP_AND_OPTIONS, 4)),
        PAGE_CREDITS_ROLL => Some((PAGE_CREDITS, 0)),
        PAGE_EXIT => Some((PAGE_TITLE, 4)),
        PAGE_MOUSE_SETTINGS => Some((PAGE_CONTROLS, 0)),
        PAGE_KEY_BINDS => Some((PAGE_CONTROLS, 1)),
        PAGE_CONTROLLER => Some((PAGE_CONTROLS, 2)),
        PAGE_GAME_OPTIONS => Some((PAGE_SETTINGS, 0)),
        PAGE_AUDIO => Some((PAGE_SETTINGS, 1)),
        PAGE_GRAPHICS => Some((PAGE_SETTINGS, 2)),
        PAGE_USER_INTERFACE => Some((PAGE_SETTINGS, 3)),
        PAGE_RESET_SETTINGS => Some((PAGE_SETTINGS, 4)),
        PAGE_STORE_CATEGORY => Some((PAGE_STORE, 0)),
        _ => None,
    }
}

fn child_page(page: usize, index: usize) -> Option<usize> {
    match (page, index) {
        (PAGE_TITLE, 0) => Some(PAGE_PLAY_GAME),
        (PAGE_TITLE, 1) => Some(PAGE_LEADERBOARDS),
        (PAGE_TITLE, 2) => Some(PAGE_HELP_AND_OPTIONS),
        (PAGE_TITLE, 3) => Some(PAGE_STORE),
        (PAGE_TITLE, 4) => Some(PAGE_EXIT),
        (PAGE_HELP_AND_OPTIONS, 0) => Some(PAGE_CHANGE_SKIN),
        (PAGE_HELP_AND_OPTIONS, 1) => Some(PAGE_HOW_TO_PLAY),
        (PAGE_HELP_AND_OPTIONS, 2) => Some(PAGE_CONTROLS),
        (PAGE_HELP_AND_OPTIONS, 3) => Some(PAGE_SETTINGS),
        (PAGE_HELP_AND_OPTIONS, 4) => Some(PAGE_CREDITS),
        (PAGE_CREDITS, 0) => Some(PAGE_CREDITS_ROLL),
        (PAGE_CONTROLS, 0) => Some(PAGE_MOUSE_SETTINGS),
        (PAGE_CONTROLS, 1) => Some(PAGE_KEY_BINDS),
        (PAGE_CONTROLS, 2) => Some(PAGE_CONTROLLER),
        (PAGE_SETTINGS, 0) => Some(PAGE_GAME_OPTIONS),
        (PAGE_SETTINGS, 1) => Some(PAGE_AUDIO),
        (PAGE_SETTINGS, 2) => Some(PAGE_GRAPHICS),
        (PAGE_SETTINGS, 3) => Some(PAGE_USER_INTERFACE),
        (PAGE_SETTINGS, 4) => Some(PAGE_RESET_SETTINGS),
        (PAGE_STORE, _) => Some(PAGE_STORE_CATEGORY),
        (PAGE_HOW_TO_PLAY, _) => Some(PAGE_HOW_TO_PLAY_ARTICLE),
        (PAGE_RESET_SETTINGS, 0 | 1) => Some(PAGE_SETTINGS),
        (PAGE_EXIT, 0) => Some(PAGE_TITLE),
        _ => None,
    }
}

fn visible_button_window(page: usize, focus: usize) -> (usize, usize) {
    if page == PAGE_HOW_TO_PLAY {
        list_window(focus, HOW_TO_PLAY_SECTIONS.len())
    } else if let Some((labels, _)) = option_page(page) {
        list_window(focus, labels.len())
    } else {
        (0, page_button_count(page))
    }
}

fn draw_legacy_panorama(event: &mut ScreenRenderPostEvent<'_>, size: [f32; 2]) {
    // Legacy4J treats the 1230x216 strip as an 820:144 scrolling panorama.
    // Convert its virtual source rectangle back into the texture's real pixels.
    let virtual_width = size[1] * 820.0 / 144.0;
    let source_width = (PANORAMA_WIDTH * size[0] / virtual_width).min(PANORAMA_WIDTH);
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0.0, |duration| duration.as_millis() as f32);
    let source_x =
        (millis * PANORAMA_WIDTH * 144.0 / (360.0 * 66.32 * 820.0)).rem_euclid(PANORAMA_WIDTH);
    let first_width = source_width.min(PANORAMA_WIDTH - source_x);
    let first_dst = size[0] * first_width / source_width;
    let _ = event.draw.sprite(
        PANORAMA_DAY,
        [source_x, 0.0, first_width, PANORAMA_HEIGHT],
        [0.0, 0.0, first_dst, size[1]],
        None,
    );
    let remaining = source_width - first_width;
    if remaining > 0.0 {
        let _ = event.draw.sprite(
            PANORAMA_DAY,
            [0.0, 0.0, remaining, PANORAMA_HEIGHT],
            [first_dst, 0.0, size[0] - first_dst, size[1]],
            None,
        );
    }
}

#[mod_api::horizon_mod]
impl Hooks for LegacyHorizon {
    fn register_commands(
        &self,
        _: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::RegisterCommandsEvent<'_>,
    ) {
        event.commands.push(mod_api::Command::new("legacy_mobcaps"));
    }

    fn run_mod_command(
        &self,
        host: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::RunModCommandEvent<'_>,
    ) -> Flow {
        if event.command.literals.first().map(String::as_str) != Some("legacy_mobcaps") {
            return Flow::Continue;
        }
        if let Some(dimension) = event.command.dimension.id(host) {
            let message = self.mobcaps.status(host, &dimension);
            let _ = host.commands().feedback(Text::literal(message));
        }
        Flow::Cancel
    }

    fn register(&self, registrar: &mut dyn mod_api::Registrar, _: &RegistrationContext) {
        self.mobcaps.register(registrar);
    }

    fn potential_spawns(
        &self,
        host: &dyn mod_api::Host,
        event: &mut mod_api::PotentialSpawnsEvent<'_>,
    ) -> Flow {
        self.mobcaps.observe(host, event);
        Flow::Continue
    }

    fn mob_spawn_position_check(
        &self,
        host: &dyn mod_api::Host,
        event: &mut mod_api::MobSpawnPositionCheckEvent<'_>,
    ) {
        self.mobcaps.check(host, event);
    }

    fn entity_join_level(
        &self,
        _: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::EntityJoinLevelEvent<'_>,
    ) -> Flow {
        self.mobcaps.invalidate(event.dimension);
        Flow::Continue
    }

    fn entity_leave_level(
        &self,
        _: &mut dyn mod_api::ServerHost,
        event: &mut mod_api::EntityLeaveLevelEvent<'_>,
    ) {
        self.mobcaps.invalidate(event.dimension);
    }

    fn server_tick(
        &self,
        host: &mut dyn mod_api::ServerHost,
        _: &mut mod_api::ServerTickEvent<'_>,
    ) {
        self.mobcaps.next_tick();
        self.mobcaps.sync_creature_limit(host);
        controller::set_server(true);
    }

    fn server_stopped(&self, _: &mut dyn mod_api::ServerHost, _: &mut mod_api::ServerStoppedEvent) {
        self.mobcaps.clear();
        controller::set_server(false);
    }
    fn render_gui_pre(&self, _: &dyn ClientHost, _: &mut mod_api::RenderGuiPreEvent) -> Flow {
        if self.interface.enabled(interface::Setting::Hud) {
            Flow::Continue
        } else {
            Flow::Cancel
        }
    }

    fn block_highlight(&self, _: &dyn ClientHost, _: &mut mod_api::BlockHighlightEvent) -> Flow {
        if self.interface.enabled(interface::Setting::Hud) {
            Flow::Continue
        } else {
            Flow::Cancel
        }
    }

    fn client_chat_received(
        &self,
        _: &mut dyn ClientHost,
        event: &mut mod_api::ClientChatReceivedEvent<'_>,
    ) -> Flow {
        if self.interface.suppress_message(event.system, event.message) {
            Flow::Cancel
        } else {
            Flow::Continue
        }
    }
    fn render_hand(&self, _: &dyn ClientHost, _: &mut mod_api::RenderHandEvent<'_, '_>) -> Flow {
        if self.interface.display_hand() {
            Flow::Continue
        } else {
            Flow::Cancel
        }
    }
    fn play_sound(&self, _: &dyn mod_api::LoopContext, event: &mut mod_api::PlaySoundEvent<'_>) {
        self.audio.filter(event.sound);
    }

    fn client_tick_pre(&self, host: &mut dyn ClientHost, _: &mut ClientTickPreEvent) {
        self.pause.tick(host);
        match host.observed_screen_id() {
            None => {
                self.pause.reset();
                controller::set_title_active(false);
            }
            Some(screen) if screen.id != pause_menu::SCREEN => self.pause.reset(),
            Some(_) => {}
        }
        self.audio.sync(host);
        self.interface.sync(host);
        let request = self.crafting_request.swap(0, Ordering::Relaxed);
        if request == 0 {
            return;
        }
        let index = request - 1;
        let recipes = host.recipes_of_type("minecraft:crafting");
        let Some(recipe) = recipes
            .iter()
            .filter(|recipe| {
                crafting_category_matches(recipe, self.crafting_category.load(Ordering::Relaxed))
            })
            .nth(index)
        else {
            return;
        };
        let book_ids = host.recipe_book_ids(&recipe.id);
        if let Some(book_id) = book_ids.first() {
            let status = match host.place_recipe(*book_id, false) {
                Ok(()) => 2,
                Err(error) => {
                    eprintln!(
                        "Legacy Horizon: place_recipe refused {}: {error}",
                        recipe.id
                    );
                    3
                }
            };
            self.crafting_status.store(status, Ordering::Relaxed);
        } else {
            host.transfer_recipe_id(&recipe.id, false);
            self.crafting_status.store(4, Ordering::Relaxed);
        }
    }

    fn register_dimension_transition_screen(
        &self,
        _: &RegistrationContext,
        event: &mut RegisterDimensionTransitionScreenEvent<'_>,
    ) {
        let mut conditional = |screen: &str, to: &str, from: &str| {
            event.screens.push(DimensionTransitionScreen {
                screen: screen.to_owned(),
                transition: DimensionTransition::Conditional {
                    to: to.to_owned(),
                    from: from.to_owned(),
                },
            });
        };

        conditional(
            LOADING_ENTER_NETHER,
            "minecraft:the_nether",
            "minecraft:overworld",
        );
        conditional(
            LOADING_ENTER_NETHER,
            "minecraft:the_nether",
            "minecraft:the_end",
        );
        conditional(
            LOADING_ENTER_END,
            "minecraft:the_end",
            "minecraft:overworld",
        );
        conditional(
            LOADING_ENTER_END,
            "minecraft:the_end",
            "minecraft:the_nether",
        );
        conditional(
            LOADING_LEAVE_NETHER,
            "minecraft:overworld",
            "minecraft:the_nether",
        );
        conditional(
            LOADING_LEAVE_END,
            "minecraft:overworld",
            "minecraft:the_end",
        );
        for dimension in [
            "minecraft:overworld",
            "minecraft:the_nether",
            "minecraft:the_end",
        ] {
            conditional(LOADING_RESPAWN, dimension, dimension);
        }
    }

    fn dimension_transition_screen_draw(
        &self,
        host: &dyn ClientHost,
        event: &mut DimensionTransitionScreenDrawEvent<'_>,
    ) {
        draw_dimension_loading(host, event);
    }

    fn screen_render_pre(&self, _: &dyn ClientHost, event: &mut ScreenRenderPreEvent<'_>) -> Flow {
        if event.screen.id == TITLE_SCREEN || event.screen.id == pause_menu::SCREEN {
            // The title's and the pause menu's normal drawing is replaced, then
            // screen_render_post paints Legacy4J's. Toasts remain above it as intended.
            Flow::Cancel
        } else {
            Flow::Continue
        }
    }

    fn screen_render_post(&self, host: &dyn ClientHost, event: &mut ScreenRenderPostEvent<'_>) {
        controller::set_title_active(
            event.screen.id == TITLE_SCREEN || event.screen.id == pause_menu::SCREEN,
        );
        if event.screen.id == TITLE_SCREEN || event.screen.id == pause_menu::SCREEN {
            while let Some(key) = controller::take_menu_key() {
                let mut input = ScreenKeyPressedPreEvent {
                    screen: event.screen,
                    key,
                    repeat: false,
                    modifiers: 0,
                    cancelled: false,
                };
                self.screen_key_pressed_pre(host, &mut input);
            }
        }
        self.audio.sync(host);
        self.interface.sync(host);
        if event.screen.id == TITLE_SCREEN {
            if self.page.load(Ordering::Relaxed) == PAGE_CREDITS_ROLL {
                credits::draw(
                    host,
                    event,
                    self.credits_started.load(Ordering::Relaxed),
                    &self.credits_clock,
                );
            } else {
                draw_page(
                    host,
                    event,
                    self.page.load(Ordering::Relaxed),
                    self.keyboard_focus.load(Ordering::Relaxed),
                    self.how_to_play_section.load(Ordering::Relaxed),
                    self.article_scroll.load(Ordering::Relaxed),
                    self.store_category.load(Ordering::Relaxed),
                    self.play_tab.load(Ordering::Relaxed),
                    &self.audio,
                    &self.interface,
                );
            }
            if self.page.load(Ordering::Relaxed) != PAGE_TITLE {
                let size = event.draw.size();
                event
                    .draw
                    .text("< Back", [16.0, size[1] - 18.0], 0xFFFF_FFFF, true);
            }
        } else if event.screen.id == pause_menu::SCREEN {
            self.pause.draw(host, event);
        } else if event.screen.id == "inventory" && event.screen.menu == Some("minecraft:crafting")
        {
            draw_crafting_menu(
                host,
                event,
                self.crafting_focus.load(Ordering::Relaxed),
                self.crafting_scroll.load(Ordering::Relaxed),
                self.crafting_status.load(Ordering::Relaxed),
                self.crafting_category.load(Ordering::Relaxed),
            );
        }
    }

    fn screen_mouse_button_pressed_pre(
        &self,
        host: &dyn ClientHost,
        event: &mut ScreenMouseButtonPressedPreEvent<'_>,
    ) -> Flow {
        if event.screen.id == pause_menu::SCREEN {
            return self.pause.mouse_pressed(host, event);
        }
        if event.button != 0 {
            return Flow::Continue;
        }
        if event.screen.id == "inventory" && event.screen.menu == Some("minecraft:crafting") {
            if !crafting_browser_fits(event.screen.rect[0]) {
                return Flow::Continue;
            }
            let recipes = host.recipes_of_type("minecraft:crafting");
            let count = recipes
                .iter()
                .filter(|recipe| {
                    crafting_category_matches(
                        recipe,
                        self.crafting_category.load(Ordering::Relaxed),
                    )
                })
                .count();
            let panel_x = if event.screen.rect[0] >= 164.0 {
                event.screen.rect[0] - 160.0
            } else {
                4.0
            };
            let panel = [
                panel_x,
                event.screen.rect[1] + ((event.screen.rect[3] - 215.0) / 2.0).floor(),
                154.0,
                215.0,
            ];
            let start = self
                .crafting_scroll
                .load(Ordering::Relaxed)
                .min(count.saturating_sub(1));
            let visible_count = (count - start).min(10);
            for category in 0..4 {
                if contains(crafting_tab_rect(panel, category), event.screen.mouse) {
                    self.crafting_category.store(category, Ordering::Relaxed);
                    self.crafting_focus.store(0, Ordering::Relaxed);
                    self.crafting_scroll.store(0, Ordering::Relaxed);
                    self.crafting_status.store(0, Ordering::Relaxed);
                    play_button_sound(host);
                    return Flow::Cancel;
                }
            }
            if let Some(visible_index) = (0..visible_count)
                .position(|index| contains(crafting_recipe_rect(panel, index), event.screen.mouse))
            {
                self.crafting_focus
                    .store(start + visible_index, Ordering::Relaxed);
                self.crafting_request
                    .store(start + visible_index + 1, Ordering::Relaxed);
                play_button_sound(host);
                return Flow::Cancel;
            }
            return Flow::Continue;
        }
        if event.screen.id != TITLE_SCREEN {
            return Flow::Continue;
        }
        let size = [event.screen.rect[2], event.screen.rect[3]];
        let page = self.page.load(Ordering::Relaxed);
        let focus = self.keyboard_focus.load(Ordering::Relaxed);
        if page != PAGE_TITLE && contains([12.0, size[1] - 27.0, 86.0, 24.0], event.screen.mouse) {
            let (parent, focus) = parent_page(page).unwrap_or((PAGE_TITLE, 0));
            self.page.store(parent, Ordering::Relaxed);
            self.keyboard_focus.store(focus, Ordering::Relaxed);
            play_button_sound(host);
            return Flow::Cancel;
        }
        if page == PAGE_PLAY_GAME {
            if contains([12.0, size[1] - 27.0, 86.0, 24.0], event.screen.mouse) {
                self.page.store(PAGE_TITLE, Ordering::Relaxed);
                self.keyboard_focus.store(0, Ordering::Relaxed);
                play_button_sound(host);
                return Flow::Cancel;
            }
            for tab in 0..3 {
                if contains(play_menu::tab_rect(size, tab), event.screen.mouse) {
                    self.play_tab.store(tab, Ordering::Relaxed);
                    play_button_sound(host);
                    return Flow::Cancel;
                }
            }
            return Flow::Cancel;
        }
        let (start, visible_count) = visible_button_window(page, focus);
        let clicked = (0..visible_count).position(|visible_index| {
            let rect = if page == PAGE_TITLE {
                menu_button_rects::<7>(size)[visible_index]
            } else if page == PAGE_HOW_TO_PLAY {
                how_to_play_button_rect(size, visible_index, visible_count)
            } else if option_page(page).is_some() {
                option_button_rect(size, visible_index)
            } else if page == PAGE_EXIT || page == PAGE_RESET_SETTINGS {
                confirmation_button_rect(size, visible_index)
            } else {
                submenu_button_rect(size, visible_index)
            };
            contains(rect, event.screen.mouse)
        });
        if let Some(visible_index) = clicked {
            let index = start + visible_index;
            if page == PAGE_AUDIO && index >= 2 {
                self.audio.toggle(index - 2);
            }
            if let Some(setting) = interface_setting(page, index) {
                self.interface.toggle(setting);
            }
            if page == PAGE_CONTROLLER && index == 3 {
                controller::request_invert_look();
            }
            if let Some((key, value)) = vanilla_bool_setting(host, page, index) {
                client_options::toggle(key, value);
            }
            if page == PAGE_EXIT && index == 1 {
                QUIT_REQUESTED.store(true, Ordering::Relaxed);
                return Flow::Cancel;
            }
            if page == PAGE_RESET_SETTINGS && index == 1 {
                return Flow::Cancel;
            }
            play_button_sound(host);
            self.keyboard_focus.store(index, Ordering::Relaxed);
            if page == PAGE_HOW_TO_PLAY {
                self.how_to_play_section.store(index, Ordering::Relaxed);
                self.article_scroll.store(0, Ordering::Relaxed);
            }
            if page == PAGE_STORE {
                self.store_category.store(index, Ordering::Relaxed);
            }
            if let Some(child) = child_page(page, index) {
                if child == PAGE_CREDITS_ROLL {
                    self.credits_started.store(now_millis(), Ordering::Relaxed);
                }
                self.page.store(child, Ordering::Relaxed);
                self.keyboard_focus.store(
                    if page == PAGE_EXIT && index == 0 {
                        4
                    } else {
                        0
                    },
                    Ordering::Relaxed,
                );
            }
            // Do not let the hidden Horizon title activate a different button.
            return Flow::Cancel;
        }
        Flow::Continue
    }

    fn screen_mouse_scrolled_pre(
        &self,
        host: &dyn ClientHost,
        event: &mut ScreenMouseScrolledPreEvent<'_>,
    ) -> Flow {
        if event.screen.id == "inventory" && !crafting_browser_fits(event.screen.rect[0]) {
            return Flow::Continue;
        }
        if event.scroll[1] == 0.0 {
            return Flow::Continue;
        }
        if event.screen.id == "inventory" && event.screen.menu == Some("minecraft:crafting") {
            let count = host
                .recipes_of_type("minecraft:crafting")
                .iter()
                .filter(|recipe| {
                    crafting_category_matches(
                        recipe,
                        self.crafting_category.load(Ordering::Relaxed),
                    )
                })
                .count();
            if count <= 10 {
                return Flow::Continue;
            }
            let current = self.crafting_scroll.load(Ordering::Relaxed);
            let next = if event.scroll[1] < 0.0 {
                (current + 5).min(count - 10)
            } else {
                current.saturating_sub(5)
            };
            self.crafting_scroll.store(next, Ordering::Relaxed);
            self.crafting_focus.store(next, Ordering::Relaxed);
            return Flow::Cancel;
        }
        if event.screen.id != TITLE_SCREEN {
            return Flow::Continue;
        }
        if self.page.load(Ordering::Relaxed) == PAGE_HOW_TO_PLAY {
            let current = self.keyboard_focus.load(Ordering::Relaxed);
            self.keyboard_focus.store(
                scroll_list_focus(current, HOW_TO_PLAY_SECTIONS.len(), event.scroll[1]),
                Ordering::Relaxed,
            );
            return Flow::Cancel;
        }
        if let Some((labels, _)) = option_page(self.page.load(Ordering::Relaxed)) {
            let current = self.keyboard_focus.load(Ordering::Relaxed);
            self.keyboard_focus.store(
                scroll_list_focus(current, labels.len(), event.scroll[1]),
                Ordering::Relaxed,
            );
            return Flow::Cancel;
        }
        if self.page.load(Ordering::Relaxed) != PAGE_HOW_TO_PLAY_ARTICLE {
            return Flow::Continue;
        }
        if event.scroll[1] < 0.0 {
            self.article_scroll.fetch_add(1, Ordering::Relaxed);
        } else {
            let _ =
                self.article_scroll
                    .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
                        Some(value.saturating_sub(1))
                    });
        }
        Flow::Cancel
    }

    fn screen_key_pressed_pre(
        &self,
        host: &dyn ClientHost,
        event: &mut ScreenKeyPressedPreEvent<'_>,
    ) -> Flow {
        if event.screen.id == pause_menu::SCREEN {
            return self.pause.key_pressed(host, event);
        }
        if event.screen.id == "inventory" && !crafting_browser_fits(event.screen.rect[0]) {
            return Flow::Continue;
        }
        if event.repeat {
            return Flow::Continue;
        }
        if event.screen.id == "inventory" && event.screen.menu == Some("minecraft:crafting") {
            let count = host
                .recipes_of_type("minecraft:crafting")
                .iter()
                .filter(|recipe| {
                    crafting_category_matches(
                        recipe,
                        self.crafting_category.load(Ordering::Relaxed),
                    )
                })
                .count();
            if count == 0 {
                return Flow::Continue;
            }
            let current = self.crafting_focus.load(Ordering::Relaxed).min(count - 1);
            let next = match event.key {
                "key.keyboard.left" => Some(current.saturating_sub(1)),
                "key.keyboard.right" => Some((current + 1).min(count - 1)),
                "key.keyboard.up" => Some(current.saturating_sub(5)),
                "key.keyboard.down" => Some((current + 5).min(count - 1)),
                _ => None,
            };
            if let Some(next) = next {
                self.crafting_focus.store(next, Ordering::Relaxed);
                let scroll = self.crafting_scroll.load(Ordering::Relaxed);
                if next < scroll {
                    self.crafting_scroll.store(next, Ordering::Relaxed);
                } else if next >= scroll + 10 {
                    self.crafting_scroll
                        .store(next.saturating_sub(9), Ordering::Relaxed);
                }
                return Flow::Cancel;
            }
            return Flow::Continue;
        }
        if event.screen.id != TITLE_SCREEN {
            return Flow::Continue;
        }
        let page = self.page.load(Ordering::Relaxed);
        let count = page_button_count(page);
        if page == PAGE_PLAY_GAME {
            let tab = self.play_tab.load(Ordering::Relaxed).min(2);
            match event.key {
                "key.keyboard.left" | "key.keyboard.left.bracket" => {
                    self.play_tab.store((tab + 2) % 3, Ordering::Relaxed);
                }
                "key.keyboard.right" | "key.keyboard.right.bracket" | "key.keyboard.tab" => {
                    self.play_tab.store((tab + 1) % 3, Ordering::Relaxed);
                }
                "key.keyboard.escape" => {
                    self.page.store(PAGE_TITLE, Ordering::Relaxed);
                    self.keyboard_focus.store(0, Ordering::Relaxed);
                    return Flow::Cancel;
                }
                _ => return Flow::Cancel,
            }
            play_button_sound(host);
            return Flow::Cancel;
        }
        if count == 0 {
            if page == PAGE_HOW_TO_PLAY_ARTICLE {
                let section = self.how_to_play_section.load(Ordering::Relaxed);
                match event.key {
                    "key.keyboard.up" => {
                        let _ = self.article_scroll.fetch_update(
                            Ordering::Relaxed,
                            Ordering::Relaxed,
                            |value| Some(value.saturating_sub(1)),
                        );
                        return Flow::Cancel;
                    }
                    "key.keyboard.down" => {
                        self.article_scroll.fetch_add(1, Ordering::Relaxed);
                        return Flow::Cancel;
                    }
                    "key.keyboard.enter" | "key.keyboard.keypad.enter" | "key.keyboard.right"
                        if section + 1 < HOW_TO_PLAY_SECTIONS.len() =>
                    {
                        play_button_sound(host);
                        self.how_to_play_section
                            .store(section + 1, Ordering::Relaxed);
                        self.article_scroll.store(0, Ordering::Relaxed);
                        return Flow::Cancel;
                    }
                    "key.keyboard.x" | "key.keyboard.left" if section > 0 => {
                        play_button_sound(host);
                        self.how_to_play_section
                            .store(section - 1, Ordering::Relaxed);
                        self.article_scroll.store(0, Ordering::Relaxed);
                        return Flow::Cancel;
                    }
                    _ => {}
                }
            }
            if event.key == "key.keyboard.escape" {
                let (parent, parent_focus) = if page == PAGE_HOW_TO_PLAY_ARTICLE {
                    (
                        PAGE_HOW_TO_PLAY,
                        self.how_to_play_section.load(Ordering::Relaxed),
                    )
                } else if let Some(parent) = parent_page(page) {
                    parent
                } else {
                    return Flow::Continue;
                };
                self.page.store(parent, Ordering::Relaxed);
                self.keyboard_focus.store(parent_focus, Ordering::Relaxed);
                return Flow::Cancel;
            }
            return Flow::Continue;
        }
        let current = self.keyboard_focus.load(Ordering::Relaxed).min(count - 1);
        let next = match event.key {
            "key.keyboard.up" => Some(current.checked_sub(1).unwrap_or(count - 1)),
            "key.keyboard.down" => Some((current + 1) % count),
            "key.keyboard.enter" | "key.keyboard.keypad.enter" => {
                if let Some((key, value)) = vanilla_bool_setting(host, page, current) {
                    client_options::toggle(key, value);
                    play_button_sound(host);
                    return Flow::Cancel;
                }
                if page == PAGE_CONTROLLER && current == 3 {
                    controller::request_invert_look();
                    play_button_sound(host);
                    return Flow::Cancel;
                }
                if let Some(setting) = interface_setting(page, current) {
                    self.interface.toggle(setting);
                    play_button_sound(host);
                    return Flow::Cancel;
                }
                if page == PAGE_AUDIO && current >= 2 {
                    self.audio.toggle(current - 2);
                    play_button_sound(host);
                    return Flow::Cancel;
                }
                if page == PAGE_EXIT && current == 1 {
                    QUIT_REQUESTED.store(true, Ordering::Relaxed);
                    return Flow::Cancel;
                }
                if page == PAGE_RESET_SETTINGS && current == 1 {
                    return Flow::Cancel;
                }
                if page == PAGE_HOW_TO_PLAY {
                    self.how_to_play_section.store(current, Ordering::Relaxed);
                    self.article_scroll.store(0, Ordering::Relaxed);
                }
                child_page(page, current).map(|child| {
                    if child == PAGE_CREDITS_ROLL {
                        self.credits_started.store(now_millis(), Ordering::Relaxed);
                    }
                    play_button_sound(host);
                    self.page.store(child, Ordering::Relaxed);
                    if page == PAGE_EXIT && current == 0 {
                        4
                    } else {
                        0
                    }
                })
            }
            "key.keyboard.escape" => parent_page(page).map(|(parent, parent_focus)| {
                self.page.store(parent, Ordering::Relaxed);
                parent_focus
            }),
            _ => None,
        };
        if let Some(next) = next {
            self.keyboard_focus.store(next, Ordering::Relaxed);
            Flow::Cancel
        } else {
            Flow::Continue
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_layout_matches_legacy4j_measurements() {
        let buttons = menu_button_rects::<5>([854.0, 480.0]);
        assert_eq!(buttons[0], [314.0, 165.0, 225.0, 20.0]);
        assert_eq!(buttons[4], [314.0, 265.0, 225.0, 20.0]);
    }

    #[test]
    fn button_edges_do_not_overlap() {
        let buttons = menu_button_rects::<5>([640.0, 360.0]);
        assert!(contains(buttons[0], [buttons[0][0], buttons[0][1]]));
        assert!(!contains(
            buttons[0],
            [buttons[0][0], buttons[0][1] + BUTTON_HEIGHT]
        ));
    }

    #[test]
    fn help_and_options_uses_the_legacy_vertical_list_layout() {
        assert_eq!(
            submenu_button_rect([512.0, 320.0], 0),
            [144.0, 111.0, 225.0, 20.0]
        );
        assert_eq!(
            submenu_button_rect([512.0, 320.0], 4),
            [144.0, 211.0, 225.0, 20.0]
        );
        assert_eq!(HELP_AND_OPTIONS_BUTTONS.len(), 5);
    }

    #[test]
    fn how_to_play_uses_legacy4j_options_panel_geometry() {
        assert_eq!(
            how_to_play_panel_rect([512.0, 320.0], 7),
            [136.0, 80.0, 240.0, 199.0]
        );
        assert_eq!(
            how_to_play_button_rect([512.0, 320.0], 0, 7),
            [144.0, 88.0, 224.0, 20.0]
        );
    }

    #[test]
    fn nested_legacy_menus_return_to_their_calling_button() {
        assert_eq!(child_page(PAGE_TITLE, 2), Some(PAGE_HELP_AND_OPTIONS));
        assert_eq!(child_page(PAGE_HELP_AND_OPTIONS, 2), Some(PAGE_CONTROLS));
        assert_eq!(child_page(PAGE_HELP_AND_OPTIONS, 3), Some(PAGE_SETTINGS));
        assert_eq!(parent_page(PAGE_CONTROLS), Some((PAGE_HELP_AND_OPTIONS, 2)));
        assert_eq!(parent_page(PAGE_SETTINGS), Some((PAGE_HELP_AND_OPTIONS, 3)));
        assert_eq!(child_page(PAGE_CONTROLS, 0), Some(PAGE_MOUSE_SETTINGS));
        assert_eq!(parent_page(PAGE_MOUSE_SETTINGS), Some((PAGE_CONTROLS, 0)));
        assert_eq!(child_page(PAGE_SETTINGS, 3), Some(PAGE_USER_INTERFACE));
        assert_eq!(parent_page(PAGE_USER_INTERFACE), Some((PAGE_SETTINGS, 3)));
    }

    #[test]
    fn options_panels_and_crafting_buttons_use_legacy_spacing() {
        assert!(!crafting_browser_fits(125.0));
        assert!(crafting_browser_fits(168.0));
        assert_eq!(
            option_panel_rect([512.0, 320.0]),
            [131.0, 72.0, 250.0, 215.0]
        );
        assert_eq!(
            option_button_rect([512.0, 320.0], 0),
            [146.0, 90.0, 220.0, 20.0]
        );
        assert_eq!(
            crafting_recipe_rect([8.0, 52.0, 154.0, 215.0], 0),
            [17.0, 92.0, 27.0, 27.0]
        );
        assert_eq!(
            crafting_recipe_rect([8.0, 52.0, 154.0, 215.0], 9),
            [129.0, 120.0, 27.0, 27.0]
        );
    }

    #[test]
    fn every_title_button_has_a_ported_destination() {
        assert_eq!(child_page(PAGE_TITLE, 0), Some(PAGE_PLAY_GAME));
        assert_eq!(child_page(PAGE_TITLE, 1), Some(PAGE_LEADERBOARDS));
        assert_eq!(child_page(PAGE_TITLE, 2), Some(PAGE_HELP_AND_OPTIONS));
        assert_eq!(child_page(PAGE_TITLE, 3), Some(PAGE_STORE));
        assert_eq!(child_page(PAGE_TITLE, 4), Some(PAGE_EXIT));
    }

    #[test]
    fn how_to_play_index_matches_valid_legacy4j_sections() {
        assert_eq!(HOW_TO_PLAY_SECTIONS.len(), 43);
        assert_eq!(HOW_TO_PLAY_SECTIONS[0], "What's New");
        assert_eq!(HOW_TO_PLAY_SECTIONS[42], "Host and Player Options");
        assert_eq!(list_window(0, 43), (0, 7));
        assert_eq!(list_window(42, 43), (36, 7));
    }

    #[test]
    fn repeated_ui_definition_branches_do_not_duplicate_help_paragraphs() {
        let definition = serde_json::json!({
            "first": {"translate": "legacy.menu.how_to_play.basics_message"},
            "nested": [
                {"translate": "legacy.menu.how_to_play.basics_message_1"},
                {"translate": "legacy.menu.how_to_play.basics_message"}
            ],
            "title": {"translate": "legacy.menu.how_to_play.basics"}
        });
        let mut keys = Vec::new();
        collect_message_keys(&definition, &mut keys);
        assert_eq!(
            keys,
            [
                "legacy.menu.how_to_play.basics_message",
                "legacy.menu.how_to_play.basics_message_1"
            ]
        );
    }

    #[test]
    fn how_to_play_wheel_focus_stays_inside_the_section_list() {
        assert_eq!(scroll_list_focus(0, 43, 1.0), 0);
        assert_eq!(scroll_list_focus(0, 43, -1.0), 1);
        assert_eq!(scroll_list_focus(42, 43, -1.0), 42);
        assert_eq!(scroll_list_focus(42, 43, 1.0), 41);
    }
}
