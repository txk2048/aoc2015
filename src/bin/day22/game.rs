use crate::Boss;

#[derive(Copy, Clone, PartialEq)]
struct Effect {
    spell: Spell,
    duration: u8,
}

#[derive(Copy, Clone, PartialEq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

const SPELLS: [Spell; 5] = [
    Spell::MagicMissile,
    Spell::Drain,
    Spell::Shield,
    Spell::Poison,
    Spell::Recharge,
];

impl Spell {
    fn cost(&self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

#[derive(Clone)]
pub(crate) struct BattleState {
    player_health: i32,
    player_mana: i32,
    player_armor: i32,

    boss_health: i32,
    boss_damage: i32,

    active_effects: Vec<Effect>,

    mana_spent: i32,
    player_turn: bool,
}

fn available_spells(state: &BattleState) -> impl Iterator<Item = Spell> + '_ {
    SPELLS.iter().copied().filter(|spell| {
        let is_active = state
            .active_effects
            .iter()
            .any(|effect| effect.spell == *spell);

        !is_active && state.player_mana >= spell.cost()
    })
}

pub(crate) fn run(boss: Boss, hard_mode: bool) -> i32 {
    let initial_state = BattleState {
        player_health: 50,
        player_mana: 500,
        player_armor: 0,

        boss_health: boss.hp,
        boss_damage: boss.damage,

        active_effects: Vec::new(),

        mana_spent: 0,
        player_turn: true,
    };

    let mut states: Vec<BattleState> = vec![initial_state];
    let mut min_mana = i32::MAX;

    while let Some(mut state) = states.pop() {
        // hard mode
        if hard_mode && state.player_turn {
            state.player_health -= 1;

            if state.player_health <= 0 {
                continue;
            }
        }

        state.player_armor = 0; // reset armor

        // apply effects
        for effect in state.active_effects.iter_mut() {
            match effect.spell {
                Spell::Shield => {
                    state.player_armor = 7;
                }
                Spell::Poison => {
                    state.boss_health -= 3;
                }
                Spell::Recharge => {
                    state.player_mana += 101;
                }
                _ => {}
            }

            effect.duration -= 1;
        }

        // remove expired effects
        state.active_effects.retain(|effect| effect.duration > 0);

        // boss died from effects, update the minimum mana spent
        if state.boss_health <= 0 {
            min_mana = min_mana.min(state.mana_spent);
            continue;
        }

        // if it's the player's turn, try casting each spell
        if state.player_turn {
            for spell in available_spells(&state) {
                let mut new_state = state.clone();

                // cast the spell
                new_state.player_mana -= spell.cost();
                new_state.mana_spent += spell.cost();

                // if we've already spent more mana than the current minimum, skip this state
                if new_state.mana_spent >= min_mana {
                    continue;
                }

                // apply the spell's effects
                match spell {
                    Spell::MagicMissile => {
                        new_state.boss_health -= 4;
                    }
                    Spell::Drain => {
                        new_state.boss_health -= 2;
                        new_state.player_health += 2;
                    }
                    Spell::Shield => {
                        new_state.active_effects.push(Effect { spell, duration: 6 });
                    }
                    Spell::Poison => {
                        new_state.active_effects.push(Effect { spell, duration: 6 });
                    }
                    Spell::Recharge => {
                        new_state.active_effects.push(Effect { spell, duration: 5 });
                    }
                }

                if new_state.boss_health <= 0 {
                    // if the boss is dead, update the minimum mana spent
                    min_mana = min_mana.min(new_state.mana_spent);
                } else {
                    // otherwise, it's the boss's turn
                    new_state.player_turn = false;
                    states.push(new_state);
                }
            }
        } else {
            // otherwise, it's the boss's turn
            state.player_health -= (state.boss_damage - state.player_armor).max(1);
            state.player_turn = true;

            if state.player_health > 0 {
                states.push(state);
            }
        }
    }

    min_mana
}
