//! Legacy4J PlayGameScreen geometry, shared by drawing and pointer input.
use super::*;

pub(super) fn panel_rect(size: [f32; 2]) -> [f32; 4] {
    let height = 256.0_f32.min(size[1] - 52.0).max(0.0);
    [
        ((size[0] - 300.0) / 2.0).floor(),
        ((size[1] - height) / 2.0).floor() + 12.0,
        300.0,
        height,
    ]
}

pub(super) fn tab_rect(size: [f32; 2], tab: usize) -> [f32; 4] {
    let panel = panel_rect(size);
    [panel[0] + tab as f32 * 100.0, panel[1] - 25.0, 100.0, 31.0]
}

fn bevel(draw: &mut dyn DrawContext, rect: [f32; 4], recessed: bool) {
    let [x, y, w, h] = rect;
    draw.fill([x - 1.0, y - 1.0, w + 2.0, h + 2.0], 0xFF10_1010);
    draw.fill(rect, if recessed { 0xFF9D_9D9D } else { 0xFFD0_D0D0 });
    let (light, dark) = if recessed {
        (0xFF50_5050, 0xFFFF_FFFF)
    } else {
        (0xFFFF_FFFF, 0xFF70_7070)
    };
    draw.fill([x, y, w, 2.0], light);
    draw.fill([x, y, 2.0, h], light);
    draw.fill([x, y + h - 2.0, w, 2.0], dark);
    draw.fill([x + w - 2.0, y, 2.0, h], dark);
}

pub(super) fn draw(host: &dyn ClientHost, event: &mut ScreenRenderPostEvent<'_>, selected: usize) {
    let size = event.draw.size();
    draw_legacy_panorama(event, size);
    let panel = panel_rect(size);
    let selected = selected.min(2);
    for tab in 0..3 {
        let mut rect = tab_rect(size, tab);
        if tab != selected {
            rect[1] += 4.0;
            rect[3] -= 4.0;
        }
        bevel(event.draw, rect, false);
        if tab != selected {
            event.draw.fill(
                [rect[0] + 2.0, rect[1] + 2.0, rect[2] - 4.0, rect[3] - 4.0],
                0xFFAA_AAAA,
            );
        }
        let title = translated_label(host, PLAY_GAME_KEYS[tab], PLAY_GAME_TABS[tab]);
        let width = event.draw.text_width(&title);
        event.draw.text(
            &title,
            [rect[0] + (rect[2] - width) / 2.0, rect[1] + 9.0],
            0xFF48_4848,
            false,
        );
    }
    bevel(event.draw, panel, false);
    let active = tab_rect(size, selected);
    event.draw.fill(
        [active[0] + 2.0, panel[1] - 1.0, active[2] - 4.0, 4.0],
        0xFFD0_D0D0,
    );
    let recess = [
        panel[0] + 9.0,
        panel[1] + 9.0,
        panel[2] - 18.0,
        panel[3] - 18.0,
    ];
    bevel(event.draw, recess, true);
    let rows: &[(&str, &str, &str)] = match selected {
        1 => &[
            (
                "selectWorld.create",
                "Create New World",
                "minecraft:grass_block",
            ),
            (
                "legacy.menu.play_tutorial",
                "Play Tutorial",
                "minecraft:crafting_table",
            ),
        ],
        2 => &[
            ("legacy.menu.add_server", "Add Server", "minecraft:dirt"),
            ("", "Friends", "minecraft:player_head"),
        ],
        _ => &[],
    };
    for (row, (key, fallback, icon)) in rows.iter().enumerate() {
        let rect = [
            recess[0] + 4.0,
            recess[1] + 4.0 + row as f32 * 30.0,
            recess[2] - 8.0,
            30.0,
        ];
        // Rows require title actions from upstream issue 3. Keep them disabled
        // until they can operate on Horizon's real world/server state.
        let _ = event
            .draw
            .widget_button(rect, &Text::literal(""), false, false);
        if let Ok(id) = Identifier::parse(icon) {
            if let Some(item) = host.item_key(&id) {
                let _ = event.draw.item(item, [rect[0] + 5.0, rect[1] + 5.0], 20.0);
            }
        }
        let label = translated_label(host, key, fallback);
        event
            .draw
            .text(&label, [rect[0] + 34.0, rect[1] + 11.0], 0xFFDD_DDDD, true);
    }
    if selected == 0 {
        event.draw.text(
            "World list API unavailable",
            [recess[0] + 10.0, recess[1] + 12.0],
            0xFF48_4848,
            false,
        );
    }
    event
        .draw
        .text("Esc  Back", [16.0, size[1] - 22.0], 0xFFFF_FFFF, true);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tabs_join_the_legacy_panel_and_fit_short_windows() {
        let panel = panel_rect([512.0, 320.0]);
        assert_eq!(panel, [106.0, 44.0, 300.0, 256.0]);
        assert_eq!(tab_rect([512.0, 320.0], 2), [306.0, 19.0, 100.0, 31.0]);
        assert!(panel_rect([426.0, 240.0])[3] <= 188.0);
    }
}
