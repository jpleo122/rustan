use std::collections::HashMap;
use std::mem::replace;
use crate::board::{Board, Player};
use crate::map::{Map, MapType, Resource};
use std::time::Duration;

pub enum GameType {
    Base
}

pub struct ConfigurableRules {
    hide_bank_cards: bool,
    friendly_robber: bool,
    turn_timer: Duration,
    player_count: i8,
    win_points: i8,
    discard_limit: i8
}

pub struct Settings {
    game_type: GameType,
    map: MapType,
    configurable_rules: ConfigurableRules,
}

// // tightly coupled to Game?
// pub enum Actions {
//     RollDice,
//     PlaceSettlement,
//     UpgradeSettlementToCity,
//     PlaceRoad,
//     BuyDevelopmentCard,
//     PlayDevelopmentCard,
//     PlaceRobber,
//     EndTurn
//     // InitiateTrade
//     // AcceptTrade
//     // InitiateNewTrade
// }
// 
#[derive(Clone)]
pub enum DevelopmentCard {
    Knight,
    Monopoly,
    RoadBuilding,
    YearOfPlenty,
    VictoryPoint,
}

struct Bank {
    resources: CardBank,
    development_cards: Vec<DevelopmentCard>,
}

impl Bank {
    pub fn new() -> Self {
        Self {
            resources: CardBank::new(),
            development_cards: Vec::new()
        }
    }

    pub fn starter_bank() -> Self {

        let mut resources = CardBank::new();
        let starter_count = 19;

        resources.insert(Resource::WOOD, starter_count);
        resources.insert(Resource::BRICK, starter_count);
        resources.insert(Resource::SHEEP, starter_count);
        resources.insert(Resource::WHEAT, starter_count);
        resources.insert(Resource::ORE, starter_count);

        let mut development_cards = Vec::new();
        development_cards.extend(vec![DevelopmentCard::Knight; 14]);
        development_cards.extend(vec![DevelopmentCard::Monopoly; 2]);
        development_cards.extend(vec![DevelopmentCard::RoadBuilding; 2]);
        development_cards.extend(vec![DevelopmentCard::YearOfPlenty; 2]);
        development_cards.extend(vec![DevelopmentCard::VictoryPoint; 5]);

        Self {
            resources,
            development_cards
        }
    }
}

type CardBank = HashMap<Resource, i8>;

pub struct PlayerState {
    player: Player,
    bank: Bank,
    victory_points: i8
}

impl PlayerState {
    fn new(player: Player) -> Self {
        Self {player, bank: Bank::new(), victory_points: 0}
    }
}

pub struct GameState {
    players: Vec<PlayerState>,
    pub(crate) board: Board,
    bank: Bank,
}

impl GameState {
    pub fn new(num_players: i8, board: Board) -> Self {
        let players = (0..num_players).map(|p| {
            PlayerState::new(p)
        }).collect();

        Self {players, board, bank: Bank::starter_bank()}
    }
}

pub struct Game {
    settings: Settings,
    state: GameState
}

impl Game {
    pub fn new(settings: Settings) -> Self {

        let board = Board::from_map(Map::from_map_type(&settings.map));
        let state = GameState::new(settings.configurable_rules.player_count, board);

        Self {
            settings,
            state
        }
    }
}

fn game_loop() {

    // setup phase
    // limited actions in order, place road, then settlement, then next player

    loop {
        // roll dice
        // player turn, any number of actions (invalid ones do nothing)
        // go to next player turn
    }
}