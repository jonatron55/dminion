use tauri::Manager;

use crate::{
    dice::DiceExpr,
    game::{
        time::{Duration, Time},
        Class, Condition, Game, Lair, Monster, Player, Stats,
    },
    state::{AppState, AppStateMutex, GameState},
};

mod dice;
mod dice_commands;
mod game;
mod game_commands;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut game = Game::new();
    game.spawn(Player {
        name: "Heroic Joe".into(),
        classes: vec![Class{ name: "Fighter".into(), level: 5 }],
        stats: Stats{
            str: 16,
            dex: 14,
            con: 15,
            int: 10,
            wis: 12,
            cha: 8,
        },
        ac: 18,
        initiative_bonus: 0,
        small_portrait: None,
        full_portrait: None,
        notes: "Lawful stupid murder hobo with an angsty backstory who roams the land in search of justice or something.".into(),
        initiative: 15,
        tiebreaker: 123,
        action: true,
        reaction: true,
        bonus_action: true,
        conditions: vec![
            Condition::surprised(Time::new(0, 0)),
            Condition::concentrating(Time::new(0, 0)).with_expiry(Duration::from_secs(60).into())
        ],
    }.into());

    game.spawn(Monster {
        name: "Gobbo McGobface".into(),
        subtype: "Small Humanoid (Goblin)".into(),
        small_portrait: Some("/images/portraits/goblin.small.jpg".into()),
        full_portrait: Some("/images/portraits/goblin.full.jpg".into()),
        cr: 1,
        ac: 14,
        stats: Stats {
            str: 8,
            dex: 15,
            con: 10,
            int: 10,
            wis: 8,
            cha: 8,
        },
        initiative_bonus: 0,
        hit_dice: DiceExpr::parse("3d6").unwrap(),
        initiative: 12,
        tiebreaker: -456,
        hp: 8,
        temp_hp: 0,
        max_hp: 16,
        action: true,
        reaction: true,
        bonus_action: true,
        legendary_actions: 0,
        is_hostile: true,
        notes: "Gobbo McGobface is a complex and multidimensional character with hopes, dreams, and a knife. He's green".into(),
        conditions: vec![Condition::bloodied(Time::new(0, 0))],
    }.into());

    game.spawn(
        Monster {
            name: "Froggo McFrogface".into(),
            subtype: "Medium Humanoid".into(),
            small_portrait: Some("/images/portraits/bullywug.small.jpg".into()),
            full_portrait: Some("/images/portraits/bullywug.full.jpg".into()),
            cr: 1,
            ac: 14,
            stats: Stats {
                str: 12,
                dex: 12,
                con: 13,
                int: 7,
                wis: 10,
                cha: 7,
            },
            initiative_bonus: 0,
            hit_dice: DiceExpr::parse("2d8+2").unwrap(),
            initiative: 19,
            tiebreaker: -456,
            hp: 1234,
            temp_hp: 3,
            max_hp: 1234,
            action: true,
            reaction: true,
            bonus_action: true,
            legendary_actions: 0,
            is_hostile: true,
            notes: "Froggo McFrogface would rather be eating flies.".into(),
            conditions: vec![
                Condition::prone(Time::new(0, 0)),
                Condition::poisoned(Time::new(16, 0)).with_expiry(Duration::from_secs(60).into()),
                Condition::blinded(Time::new(6, 0)).with_expiry(Duration::from_secs(60).into()),
            ],
        }
        .into(),
    );

    game.spawn(
        Lair {
            name: "Bullywug Lair".into(),
            notes: "A stanky swamp filled with bullying fugly wugly bullywugs.".into(),
            action: true,
            small_portrait: None,
            full_portrait: None,
        }
        .into(),
    );

    let state = AppStateMutex::new(AppState {
        gamestate: GameState {
            undo_stack: vec![game],
            redo_stack: vec![],
        },
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            dice_commands::roll,
            game_commands::new_game,
            game_commands::get_game,
            game_commands::next_turn,
            game_commands::undo,
            game_commands::redo,
            game_commands::damage,
            game_commands::heal,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
