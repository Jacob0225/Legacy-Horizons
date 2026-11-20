//! Credits behavior from Legacy4J's WinScreenMixin; no console code is bundled.
use super::*;

#[derive(Default)]
pub(super) struct RollClock {
    origin: usize,
    last: usize,
    offset: f32,
}

#[derive(Debug)]
struct Line {
    text: String,
    heading: bool,
    name: bool,
}

fn append_file(value: &Value, lines: &mut Vec<Line>) {
    let Some(sections) = value.as_array() else {
        return;
    };
    for section in sections {
        if let Some(title) = section.get("section").and_then(Value::as_str) {
            lines.push(Line {
                text: title.to_uppercase(),
                heading: true,
                name: false,
            });
            lines.push(Line {
                text: String::new(),
                heading: false,
                name: false,
            });
        }
        for discipline in section
            .get("disciplines")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            if let Some(title) = discipline
                .get("discipline")
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty())
            {
                lines.push(Line {
                    text: title.to_owned(),
                    heading: true,
                    name: false,
                });
            }
            for title in discipline
                .get("titles")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                if let Some(text) = title
                    .get("title")
                    .and_then(Value::as_str)
                    .filter(|s| !s.is_empty())
                {
                    lines.push(Line {
                        text: text.to_owned(),
                        heading: false,
                        name: false,
                    });
                }
                for name in title
                    .get("names")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
                {
                    lines.push(Line {
                        text: name.to_owned(),
                        heading: false,
                        name: true,
                    });
                }
                lines.push(Line {
                    text: String::new(),
                    heading: false,
                    name: false,
                });
            }
        }
    }
}

pub(super) fn draw(
    host: &dyn ClientHost,
    event: &mut ScreenRenderPostEvent<'_>,
    started: usize,
    clock: &std::sync::Mutex<RollClock>,
) {
    let [width, height] = event.draw.size();
    let mut lines = Vec::new();
    for resource in ["minecraft:texts/credits.json", "legacy:texts/credits.json"] {
        if let Ok(id) = Identifier::parse(resource) {
            if let Some(bytes) = host.resource(PackType::ClientResources, &id) {
                if let Ok(value) = serde_json::from_slice::<Value>(&bytes) {
                    append_file(&value, &mut lines);
                }
            }
        }
    }
    let drawn = event.draw.sprite(
        "legacy:gui/credits_background",
        [0.0, 0.0, 1920.0, 1080.0],
        [0.0, 0.0, width, height],
        None,
    );
    if !drawn {
        // Existing issue 1: do not substitute the menu panorama for missing credits art.
        event.draw.fill([0.0, 0.0, width, height], 0xFF00_0000);
    }
    let speed = if host.is_key_down("key.keyboard.up") || host.is_key_down("key.keyboard.down") {
        100.0
    } else {
        20.0
    };
    let now = now_millis();
    let mut clock = clock.lock().unwrap_or_else(|error| error.into_inner());
    if clock.origin != started {
        clock.origin = started;
        clock.last = started;
        clock.offset = 0.0;
    }
    clock.offset += now.saturating_sub(clock.last) as f32 / 1000.0 * speed;
    clock.last = now;
    let offset = clock
        .offset
        .rem_euclid(height + lines.len() as f32 * 18.0 + height / 2.0);
    for (index, line) in lines.iter().enumerate() {
        let y = height + index as f32 * 18.0 - offset;
        if y < -20.0 || y > height || line.text.is_empty() {
            continue;
        }
        let scale = if line.heading { 1.5 } else { 1.0 };
        let x = (width - event.draw.text_width(&line.text) * scale) / 2.0;
        let color = if line.name { 0xFFFF_FF00 } else { 0xFFFF_FFFF };
        let _ = event.draw.gui_paint(&[
            GuiDraw::Push(vec![PoseOp::Translate(x, y), PoseOp::Scale(scale, scale)]),
            GuiDraw::Text {
                text: Text::literal(&line.text),
                x: 0,
                y: 0,
                color,
                shadow: true,
            },
            GuiDraw::Pop,
        ]);
    }
    let _ = event.draw.sprite(
        "legacy:gui/credits_background_fade",
        [0.0, 0.0, 1920.0, 1080.0],
        [0.0, 0.0, width, height],
        None,
    );
    draw_title_background(event);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_credits_keep_heading_and_name_styles() {
        let value: Value =
            serde_json::from_str(include_str!("../assets/legacy/texts/credits.json")).unwrap();
        let mut lines = Vec::new();
        append_file(&value, &mut lines);
        assert_eq!(lines[0].text, "LEGACY 4J");
        assert!(lines[0].heading);
        assert!(
            lines
                .iter()
                .any(|line| line.name && line.text == "Wilyicaro")
        );
    }
}
