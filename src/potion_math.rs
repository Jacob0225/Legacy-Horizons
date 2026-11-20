//! Test-only cauldron transition policy; blocked runtime code is in research/.
use mod_api::{Compound, Tag};

fn water() -> Compound {
    [("potion".into(), Tag::String("minecraft:water".into()))].into()
}
fn normalized(mut contents: Compound) -> Compound {
    contents.retain(|key, _| {
        matches!(
            key.to_string_lossy().as_ref(),
            "potion" | "custom_color" | "custom_effects" | "custom_name"
        )
    });
    if matches!(contents.get(&"custom_effects".into()), Some(Tag::List(list)) if list.is_empty()) {
        contents.remove(&"custom_effects".into());
    }
    contents
}

#[derive(Clone, Debug)]
enum Action {
    Pour {
        contents: Compound,
        bottle: String,
        fill: u8,
        mixed: bool,
    },
    Fill {
        contents: Compound,
        bottle: String,
        fill: u8,
    },
    Dip {
        contents: Compound,
        count: u32,
        fill: u8,
    },
}
fn plan(
    item: &str,
    count: u32,
    fill: u8,
    saved: &Compound,
    incoming: Option<Compound>,
) -> Option<Action> {
    let stored = match saved.get(&"potionContents".into()) {
        Some(Tag::Compound(value)) => normalized(value.clone()),
        _ => water(),
    };
    let bottle = match saved.get(&"lastPotionItemUsed".into()) {
        Some(Tag::String(value)) => value.to_string_lossy(),
        _ => "minecraft:potion".into(),
    };
    match item {
        "minecraft:potion" | "minecraft:splash_potion" | "minecraft:lingering_potion" => {
            let contents = incoming?;
            if fill == 3 && stored == contents && !saved.contains_key(&"dyeColor".into()) {
                return None;
            }
            let mixed = fill > 0 && stored != contents;
            Some(Action::Pour {
                contents,
                bottle: item.into(),
                fill: if mixed { 0 } else { (fill + 1).min(3) },
                mixed,
            })
        }
        "minecraft:glass_bottle"
            if fill > 0 && stored != water() && !saved.contains_key(&"dyeColor".into()) =>
        {
            Some(Action::Fill {
                contents: stored,
                bottle,
                fill: fill - 1,
            })
        }
        "minecraft:arrow" if fill > 0 && stored != water() => {
            let count = count.min(if fill < 3 { u32::from(fill) * 16 } else { 64 });
            let used = count.div_ceil(16).min(3) as u8;
            // Preserve the checked-out LegacyBlockBehaviors formula, including
            // its remaining-level choice: nonempty uses `used`, not `fill-used`.
            Some(Action::Dip {
                contents: stored,
                count,
                fill: if fill == used { 0 } else { used },
            })
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn poison() -> Compound {
        [("potion".into(), Tag::String("minecraft:poison".into()))].into()
    }
    #[test]
    fn potion_fill_mix_and_dip_follow_source() {
        let data: Compound = [("potionContents".into(), Tag::Compound(poison()))].into();
        assert!(matches!(
            plan("minecraft:potion", 1, 0, &Compound::new(), Some(poison())),
            Some(Action::Pour {
                fill: 1,
                mixed: false,
                ..
            })
        ));
        assert!(matches!(
            plan("minecraft:potion", 1, 3, &data, Some(water())),
            Some(Action::Pour {
                fill: 0,
                mixed: true,
                ..
            })
        ));
        assert!(plan("minecraft:potion", 1, 3, &data, Some(poison())).is_none());
        assert!(matches!(
            plan("minecraft:arrow", 64, 3, &data, None),
            Some(Action::Dip {
                count: 64,
                fill: 0,
                ..
            })
        ));
        assert!(matches!(
            plan("minecraft:arrow", 1, 3, &data, None),
            Some(Action::Dip {
                count: 1,
                fill: 1,
                ..
            })
        ));
    }
}
