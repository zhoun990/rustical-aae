use anyhow::Result;
use rayon::prelude::*;
use rspc::Type;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, default, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    utils::{percentage, pick},
    TIMESTAMP,
};
mod units;

#[derive(Debug, Serialize, Deserialize, Default, Clone, Type)]
pub struct Unit {
    /// 次の行動までのターン数
    pub default_delay: i32,
    /// 兵士数。HP、攻撃力、士気に影響
    pub default_manpower: i32,
    /// 攻撃力。敵ユニットの防御力との比でダメージが決まる。
    pub default_attack: i32,
    /// 防御力。敵ユニットの攻撃力との比で被ダメージが決まる。
    pub default_defense: i32,
    /// 回避性能。この割合で攻撃を回避しダメージが通らない。0のときは回避せず、1のときは被弾しない。
    /// 0<=n<1
    pub default_evasion_rate: f32,
    can_move: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Type)]
pub struct Cell {
    /// 次の行動までのターン数
    pub delay: i32,
    /// 兵士数。HP、攻撃力、士気に影響
    pub manpower: i32,
    /// 攻撃力。敵ユニットの防御力との比でダメージが決まる。
    pub attack: i32,
    /// 防御力。敵ユニットの攻撃力との比で被ダメージが決まる。
    pub defense: i32,
    /// 回避性能。この割合で攻撃を回避しダメージが通らない。0のときは回避せず、1のときは被弾しない。
    /// 0<=n<1
    pub evasion_rate: f32,
    pub unit_type: UnitType,
    pub unit_id: i32,
    pub owner_id: i32,
    pub message: Option<String>,
}

impl Cell {
    fn template(unit_type: UnitType, unit_id: i32, owner_id: i32) -> Self {
        let unit = match unit_type {
            UnitType::Infantry => units::INFANTRY,
            UnitType::Cavalry => units::CAVALRY,
            UnitType::Artillery => units::ARTILLERY,
            UnitType::Mage => units::MAGE,
        };
        Self {
            delay: 0,
            manpower: unit.default_manpower,
            attack: unit.default_attack,
            defense: unit.default_defense,
            evasion_rate: unit.default_evasion_rate,
            unit_type,
            unit_id,
            owner_id,
            message: Some(String::from("")),
        }
    }
}
pub type Board = Vec<Vec<Option<Cell>>>;

#[derive(Debug, Serialize, Deserialize, Default, Clone, Type)]
pub struct BattleState {
    pub board: Board,
    pub round: u16,
    pub attacker_id: i32,
    pub defender_id: i32,
    pub start_timestamp: u32,
    pub end_timestamp: Option<u32>,
}
impl BattleState {
    pub fn template(attacker_id: i32, defender_id: i32) -> Self {
        let initial_board: Board = vec![
            vec![
                Some(Cell::template(UnitType::Artillery, 0, 1)),
                Some(Cell::template(UnitType::Cavalry, 1, 1)),
                None,
                None,
                None,
                None,
                None,
                Some(Cell::template(UnitType::Cavalry, 2, 2)),
                Some(Cell::template(UnitType::Artillery, 3, 2)),
            ],
            vec![
                Some(Cell::template(UnitType::Mage, 4, 1)),
                Some(Cell::template(UnitType::Infantry, 5, 1)),
                None,
                None,
                None,
                None,
                None,
                Some(Cell::template(UnitType::Infantry, 6, 2)),
                Some(Cell::template(UnitType::Mage, 7, 2)),
            ],
            vec![
                Some(Cell::template(UnitType::Artillery, 8, 1)),
                Some(Cell::template(UnitType::Infantry, 9, 1)),
                None,
                None,
                None,
                None,
                None,
                Some(Cell::template(UnitType::Infantry, 10, 2)),
                Some(Cell::template(UnitType::Artillery, 11, 2)),
            ],
            vec![
                Some(Cell::template(UnitType::Mage, 12, 1)),
                Some(Cell::template(UnitType::Cavalry, 13, 1)),
                None,
                None,
                None,
                None,
                None,
                Some(Cell::template(UnitType::Cavalry, 14, 2)),
                Some(Cell::template(UnitType::Mage, 15, 2)),
            ],
            vec![
                Some(Cell::template(UnitType::Artillery, 16, 1)),
                Some(Cell::template(UnitType::Infantry, 17, 1)),
                None,
                None,
                None,
                None,
                None,
                Some(Cell::template(UnitType::Infantry, 18, 2)),
                Some(Cell::template(UnitType::Artillery, 19, 2)),
            ],
            vec![
                Some(Cell::template(UnitType::Mage, 20, 1)),
                Some(Cell::template(UnitType::Infantry, 21, 1)),
                None,
                None,
                None,
                None,
                None,
                Some(Cell::template(UnitType::Infantry, 22, 2)),
                Some(Cell::template(UnitType::Mage, 23, 2)),
            ],
            vec![
                Some(Cell::template(UnitType::Artillery, 24, 1)),
                Some(Cell::template(UnitType::Cavalry, 25, 1)),
                None,
                None,
                None,
                None,
                None,
                Some(Cell::template(UnitType::Cavalry, 26, 2)),
                Some(Cell::template(UnitType::Artillery, 27, 2)),
            ],
        ];
        Self {
            board: initial_board,
            round: 0,
            attacker_id,
            defender_id,
            start_timestamp: TIMESTAMP.read().clone(),
            end_timestamp: None,
        }
    }
    fn who_attack(&self) -> i32 {
        if self.round % 2 == 0 {
            self.attacker_id
        } else {
            self.defender_id
        }
    }
    pub fn move_unit(&mut self, from: Position, to: Position) {
        if self.end_timestamp.is_some() {
            return;
        }
        if let Some(selected_cell) = &self.board[from.y as usize][from.x as usize] {
            if selected_cell.owner_id == self.who_attack() {
                self.board = self
                    .board
                    .par_iter()
                    .enumerate()
                    .map(|(y, row)| {
                        row.par_iter()
                            .enumerate()
                            .map(|(x, cell)| {
                                if let Some(cell) = cell {
                                    if y == to.y as usize && x == to.x as usize {
                                        return match &selected_cell {
                                            Cell {
                                                delay, unit_type, ..
                                            } => {
                                                let default_delay = match unit_type {
                                                    UnitType::Infantry => {
                                                        units::INFANTRY.default_delay
                                                    }
                                                    UnitType::Cavalry => {
                                                        units::CAVALRY.default_delay
                                                    }
                                                    UnitType::Artillery => {
                                                        units::ARTILLERY.default_delay
                                                    }
                                                    UnitType::Mage => units::MAGE.default_delay,
                                                };
                                                Some(Cell {
                                                    delay: (delay - 1).max(0),
                                                    ..selected_cell.clone()
                                                })
                                            }
                                            _ => unreachable!(),
                                        };
                                    } else if y == from.y as usize && x == from.x as usize {
                                        return None;
                                    } else {
                                        return Some(Cell {
                                            delay: (cell.delay - 1).max(0),
                                            ..cell.clone()
                                        });
                                    }
                                } else {
                                    return None;
                                }
                            })
                            .collect()
                    })
                    .collect();
            }
        }
    }
    fn attacker_units_count(&self) -> usize {
        self.board
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|cell| {
                        if let Some(cell) = cell {
                            cell.owner_id == self.attacker_id
                        } else {
                            false
                        }
                    })
                    .count()
            })
            .sum()
    }
    fn defender_units_count(&self) -> usize {
        self.board
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|cell| {
                        if let Some(cell) = cell {
                            cell.owner_id == self.defender_id
                        } else {
                            false
                        }
                    })
                    .count()
            })
            .sum()
    }
    pub fn end_round(&mut self) {
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                if let Some(cell) = &self.board[y][x] {
                    let mut cell = cell.clone();
                    cell.message = Some(String::from(""));
                    if self.who_attack() == cell.owner_id {
                        let possible_attacks = self.get_possible_attacks(
                            Position {
                                y: y as i32,
                                x: x as i32,
                            },
                            false,
                        );
                        if let Some(target) = pick(possible_attacks) {
                            if let Some(enemy) = &self.board[target.y as usize][target.x as usize] {
                                let mut enemy = enemy.clone();
                                if percentage((enemy.evasion_rate * 100.0) as i32, 100) {
                                    enemy.manpower -= cell.attack * (cell.attack / enemy.defense);

                                    enemy.message = Some("hit!!".to_string());
                                    cell.message = Some("attacked!!".to_string());
                                } else {
                                    enemy.message = Some("evasion!!".to_string());
                                }
                                if enemy.manpower < 0 {
                                    self.board[target.y as usize][target.x as usize] = None;
                                } else {
                                    self.board[target.y as usize][target.x as usize] = Some(enemy);
                                }
                            }
                        }
                    }
                    self.board[y][x] = Some(cell);
                }
            }
        }
        self.round += 1;
        if self.attacker_units_count() == 0 || self.defender_units_count() == 0 {
            self.end_timestamp = Some(TIMESTAMP.read().clone());
        }
    }
    fn get_possible_attacks(&self, from: Position, ignore_self_piece: bool) -> Vec<Position> {
        if let Some(selected) = &self.board[from.y as usize][from.x as usize] {
            let attack_definitions: Vec<Position> = cast_attacks(
                selected.unit_type.clone(),
                selected.owner_id == self.attacker_id,
            );
            return attack_definitions
                .iter()
                .filter_map(|movement| {
                    let new_y = from.y + movement.y;
                    let new_x = from.x + movement.x;

                    let board_rows = self.board.len() as i32;
                    let board_columns = self.board[0].len() as i32;

                    if new_y < 0 || new_y >= board_rows || new_x < 0 || new_x >= board_columns {
                        return None;
                    }

                    if let Some(destination_cell) = &self.board[new_y as usize][new_x as usize] {
                        if ignore_self_piece == false
                            && destination_cell.owner_id == selected.owner_id
                        {
                            return None; // 自分の駒がその場所にある
                        }
                    } else {
                        return None;
                    }
                    Some(Position { y: new_y, x: new_x })
                })
                .collect();
        }
        vec![]
    }
    pub fn get_possible_moves(&self, from: Position) -> Vec<Position> {
        if let Some(selected) = &self.board[from.y as usize][from.x as usize] {
            let move_definitions = cast_moves(
                selected.unit_type.clone(),
                selected.owner_id == self.attacker_id,
            );
            return move_definitions
                .iter()
                .filter_map(|movement| {
                    if selected.delay != 0 {
                        return None;
                    }
                    let new_y = from.y + movement.y;
                    let new_x = from.x + movement.x;

                    let board_rows = self.board.len() as i32;
                    let board_columns = self.board[0].len() as i32;

                    if new_y < 0 || new_y >= board_rows || new_x < 0 || new_x >= board_columns {
                        return None;
                    }
                    if let Some(_) = &self.board[new_y as usize][new_x as usize] {
                        return None; // 駒がその場所にある
                    }
                    Some(Position { y: new_y, x: new_x })
                })
                .collect();
        }
        vec![]
    }
}
fn cast_moves(unit_type: UnitType, is_attacker: bool) -> Vec<Position> {
    let moves = match unit_type {
        UnitType::Infantry => units::INFANTRY_MOVES.to_vec(),
        UnitType::Cavalry => units::CAVALRY_MOVES.to_vec(),
        UnitType::Artillery => units::ARTILLERY_MOVES.to_vec(),
        UnitType::Mage => units::MAGE_MOVES.to_vec(),
    };
    moves
        .iter()
        .map(|move_pos| {
            if is_attacker {
                move_pos.clone()
            } else {
                Position {
                    y: -move_pos.y,
                    x: -move_pos.x,
                }
            }
        })
        .collect()
}
fn cast_attacks(unit_type: UnitType, is_attacker: bool) -> Vec<Position> {
    let attacks = match unit_type {
        UnitType::Infantry => units::INFANTRY_ATTACKS.to_vec(),
        UnitType::Cavalry => units::CAVALRY_ATTACKS.to_vec(),
        UnitType::Artillery => units::ARTILLERY_ATTACKS.to_vec(),
        UnitType::Mage => units::MAGE_ATTACKS.to_vec(),
    };
    attacks
        .iter()
        .map(|move_pos| {
            if is_attacker {
                move_pos.clone()
            } else {
                Position {
                    y: -move_pos.y,
                    x: -move_pos.x,
                }
            }
        })
        .collect()
}
#[derive(Debug, Serialize, Deserialize, Default, Clone, Type, Copy)]
pub struct Position {
    pub y: i32,
    pub x: i32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Type)]
struct CellWithPosition {
    position: Position,
    cell: Cell,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Type)]
pub enum UnitType {
    #[default]
    Infantry,
    Cavalry,
    Artillery,
    Mage,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Type)]
struct Node {
    children: Vec<Node>,
    board: Board,
    round: i32,
    is_attacker: bool,
}
