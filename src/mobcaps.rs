//! Supported natural-spawn portion of Legacy4J's ConsoleMobCaps.
//! Opt-in separately: this does not claim the missing breeding/manual/despawn policy.
use mod_api::{BoolGameRuleKey, Host, LevelRef, MobCategory, SpawnReason};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Bucket {
    Animals,
    Chickens,
    Wolves,
    Mooshrooms,
    Monsters,
    Slimes,
    Guardians,
    Ambient,
    Water,
    Squids,
    Dolphins,
    Phantoms,
    Villagers,
    SnowGolems,
    IronGolems,
    Withers,
    ArmorStands,
    Crystals,
    Boats,
    Hanging,
}

impl Bucket {
    fn natural_limit(self) -> usize {
        use Bucket::*;
        match self {
            Animals | Monsters => 50,
            Chickens | Wolves | Squids => 8,
            Mooshrooms => 2,
            Slimes => 25,
            Guardians | Villagers => 35,
            Ambient | Water => 20,
            Dolphins => 15,
            Phantoms | Withers => 5,
            SnowGolems | IronGolems | ArmorStands => 16,
            Crystals => 64,
            Boats | Hanging => 0,
        }
    }
}

fn bucket(kind: &str, category: MobCategory) -> Option<Bucket> {
    use Bucket::*;
    Some(match kind {
        "minecraft:chicken" => Chickens,
        "minecraft:wolf" => Wolves,
        "minecraft:mooshroom" => Mooshrooms,
        "minecraft:squid" | "minecraft:glow_squid" => Squids,
        "minecraft:dolphin" => Dolphins,
        "minecraft:slime" | "minecraft:magma_cube" => Slimes,
        "minecraft:guardian" | "minecraft:elder_guardian" => Guardians,
        "minecraft:bat" => Ambient,
        "minecraft:villager" => Villagers,
        "minecraft:phantom" => Phantoms,
        "minecraft:snow_golem" => SnowGolems,
        "minecraft:iron_golem" => IronGolems,
        "minecraft:wither" => Withers,
        "minecraft:armor_stand" => ArmorStands,
        "minecraft:end_crystal" => Crystals,
        "minecraft:painting" | "minecraft:item_frame" | "minecraft:glow_item_frame" => Hanging,
        "minecraft:ender_dragon" => return None,
        id if id.starts_with("minecraft:") && (id.ends_with("_boat") || id.ends_with("_raft")) => {
            Boats
        }
        _ => match category {
            MobCategory::Monster => Monsters,
            MobCategory::Creature => Animals,
            MobCategory::Ambient => Ambient,
            MobCategory::WaterCreature
            | MobCategory::WaterAmbient
            | MobCategory::UndergroundWaterCreature
            | MobCategory::Axolotls => Water,
            _ => return None,
        },
    })
}

fn parent(kind: &str) -> Option<Bucket> {
    match kind {
        "minecraft:mooshroom" | "minecraft:snow_golem" => Some(Bucket::Animals),
        "minecraft:squid" | "minecraft:glow_squid" => Some(Bucket::Water),
        "minecraft:wither" | "minecraft:phantom" => Some(Bucket::Monsters),
        _ => None,
    }
}

#[derive(Default)]
struct Census {
    buckets: [usize; 20],
    types: HashMap<String, usize>,
}

impl Census {
    fn add(&mut self, kind: &str, category: MobCategory) {
        let Some(group) = bucket(kind, category) else {
            return;
        };
        self.buckets[group as usize] += 1;
        *self.types.entry(kind.into()).or_default() += 1;
        if let Some(group) = parent(kind) {
            self.buckets[group as usize] += 1;
        }
    }

    fn allows(&self, kind: &str, category: MobCategory, dimension: &str, difficulty: &str) -> bool {
        use Bucket::*;
        if category != MobCategory::Monster && dimension != "minecraft:overworld" {
            return false;
        }
        let Some(group) = bucket(kind, category) else {
            return true;
        };
        let limit = group.natural_limit();
        if self.buckets[group as usize] >= limit {
            return false;
        }
        let count = self.types.get(kind).copied().unwrap_or(0);
        let type_limit = match kind {
            "minecraft:ghast" => 4,
            "minecraft:zombified_piglin" => 40,
            "minecraft:enderman" if dimension == "minecraft:the_end" => match difficulty {
                "normal" => 38,
                "easy" | "peaceful" => 25,
                _ => 50,
            },
            _ if matches!(
                group,
                Monsters | Animals | Ambient | Water | Slimes | Guardians
            ) =>
            {
                limit / 2
            }
            _ => return true,
        };
        count < type_limit
    }
}

#[derive(Default)]
pub struct Policy {
    rule: Mutex<Option<BoolGameRuleKey>>,
    categories: Mutex<HashMap<String, MobCategory>>,
    census: Mutex<HashMap<String, Census>>,
    original_creature_limit: Mutex<Option<u32>>,
}

impl Policy {
    pub fn status(&self, host: &dyn Host, dimension: &str) -> String {
        let Some(level) = host.level(dimension) else {
            return "Legacy natural mob caps: level unavailable".into();
        };
        let Some(entities) = level.all_entities(host) else {
            return "Legacy natural mob caps: census unavailable".into();
        };
        let mut census = Census::default();
        for entity in entities {
            if let (Some(kind), Some(category)) = (entity.kind(host), entity.classification(host)) {
                census.add(&kind.id(host), category);
            }
        }
        let mut lines = vec![format!(
            "Legacy natural mob caps: {}; {dimension}; creature gate: {}",
            if self.enabled(host, level) {
                "ON"
            } else {
                "OFF"
            },
            level.spawn_limits(host).map_or(0, |limits| limits.creature)
        )];
        for group in [
            Bucket::Animals,
            Bucket::Chickens,
            Bucket::Wolves,
            Bucket::Mooshrooms,
            Bucket::Monsters,
            Bucket::Slimes,
            Bucket::Guardians,
            Bucket::Water,
            Bucket::Squids,
            Bucket::Dolphins,
            Bucket::Ambient,
        ] {
            lines.push(format!(
                "{group:?}: {}/{}",
                census.buckets[group as usize],
                group.natural_limit()
            ));
        }
        lines.join("\n")
    }

    pub fn register(&self, registrar: &mut dyn mod_api::Registrar) {
        registrar
            .game_rule_category("mobcaps")
            .expect("owned category");
        *self.rule.lock().unwrap() = Some(
            registrar
                .game_rule_bool("natural_mobcap_limits", "legacy:mobcaps", false)
                .expect("owned rule"),
        );
    }

    fn enabled(&self, host: &dyn Host, level: LevelRef) -> bool {
        self.rule
            .lock()
            .unwrap()
            .and_then(|rule| level.game_rule_bool(host, rule))
            .unwrap_or(false)
    }

    pub fn observe(&self, host: &dyn Host, event: &mod_api::PotentialSpawnsEvent<'_>) {
        let Some(level) = host.level(event.dimension) else {
            return;
        };
        if !self.enabled(host, level) {
            return;
        }
        let category = match event.category {
            "monster" => MobCategory::Monster,
            "creature" => MobCategory::Creature,
            "ambient" => MobCategory::Ambient,
            "axolotls" => MobCategory::Axolotls,
            "underground_water_creature" => MobCategory::UndergroundWaterCreature,
            "water_creature" => MobCategory::WaterCreature,
            "water_ambient" => MobCategory::WaterAmbient,
            _ => return,
        };
        let mut categories = self.categories.lock().unwrap();
        for entry in event.entries.iter() {
            categories.insert(entry.entity.clone(), category);
        }
    }

    pub fn check(&self, host: &dyn Host, event: &mut mod_api::MobSpawnPositionCheckEvent<'_>) {
        if event.reason != SpawnReason::Natural || !self.enabled(host, event.level) {
            return;
        }
        let Some(category) = self.categories.lock().unwrap().get(event.kind).copied() else {
            return;
        };
        let mut cache = self.census.lock().unwrap();
        if !cache.contains_key(event.dimension) {
            let Some(entities) = event.level.all_entities(host) else {
                return;
            };
            let mut census = Census::default();
            for entity in entities {
                if let (Some(kind), Some(category)) =
                    (entity.kind(host), entity.classification(host))
                {
                    census.add(&kind.id(host), category);
                }
            }
            cache.insert(event.dimension.into(), census);
        }
        let difficulty = event.level.difficulty(host).unwrap_or("normal");
        if !cache[event.dimension].allows(event.kind, category, event.dimension, difficulty) {
            *event.result = mod_api::SpawnCheck::Fail;
        }
    }

    // Never count an attempted join: a later mod can still veto it. Re-read the
    // authoritative list on the next check; reset per tick and across worlds.
    pub fn invalidate(&self, dimension: &str) {
        self.census.lock().unwrap().remove(dimension);
    }
    pub fn next_tick(&self) {
        self.census.lock().unwrap().clear();
    }
    pub fn clear(&self) {
        self.next_tick();
        self.categories.lock().unwrap().clear();
        *self.original_creature_limit.lock().unwrap() = None;
    }

    pub fn sync_creature_limit(&self, host: &mut dyn mod_api::ServerHost) {
        let Some(level) = host.level("minecraft:overworld") else {
            return;
        };
        let Some(mut limits) = level.spawn_limits(host) else {
            return;
        };
        let enabled = self.enabled(host, level);
        let mut original = self.original_creature_limit.lock().unwrap();
        // Legacy4J bypasses the vanilla CREATURE global/local category gate.
        // Lift that numeric gate only while our world-wide quotas apply.
        const BYPASS: u32 = 1_000_000;
        if enabled {
            if original.is_none() {
                *original = Some(limits.creature);
            }
            if limits.creature != BYPASS {
                limits.creature = BYPASS;
                if !level.set_spawn_limits(host, limits) {
                    *original = None;
                }
            }
        } else if let Some(previous) = original.take() {
            if limits.creature == BYPASS {
                limits.creature = previous;
                level.set_spawn_limits(host, limits);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn natural_type_and_parent_limits_match_console_source() {
        let mut census = Census::default();
        for _ in 0..24 {
            census.add("minecraft:cow", MobCategory::Creature);
        }
        assert!(census.allows(
            "minecraft:cow",
            MobCategory::Creature,
            "minecraft:overworld",
            "normal"
        ));
        census.add("minecraft:cow", MobCategory::Creature);
        assert!(!census.allows(
            "minecraft:cow",
            MobCategory::Creature,
            "minecraft:overworld",
            "normal"
        ));
        for _ in 0..2 {
            census.add("minecraft:mooshroom", MobCategory::Creature);
        }
        assert_eq!(census.buckets[Bucket::Animals as usize], 27);
        assert!(!census.allows(
            "minecraft:mooshroom",
            MobCategory::Creature,
            "minecraft:overworld",
            "normal"
        ));
        assert!(census.allows(
            "minecraft:chicken",
            MobCategory::Creature,
            "minecraft:overworld",
            "normal"
        ));
    }
    #[test]
    fn dimension_and_special_hostile_limits_match_source() {
        let mut census = Census::default();
        for _ in 0..4 {
            census.add("minecraft:ghast", MobCategory::Monster);
        }
        assert!(!census.allows(
            "minecraft:ghast",
            MobCategory::Monster,
            "minecraft:the_nether",
            "hard"
        ));
        assert!(!census.allows(
            "minecraft:pig",
            MobCategory::Creature,
            "minecraft:the_nether",
            "hard"
        ));
        for _ in 0..38 {
            census.add("minecraft:enderman", MobCategory::Monster);
        }
        assert!(!census.allows(
            "minecraft:enderman",
            MobCategory::Monster,
            "minecraft:the_end",
            "normal"
        ));
        assert!(census.allows(
            "minecraft:enderman",
            MobCategory::Monster,
            "minecraft:the_end",
            "hard"
        ));
    }
}
