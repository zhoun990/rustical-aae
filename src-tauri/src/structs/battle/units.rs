use super::{Position, Unit};

pub(crate) const INFANTRY: Unit = Unit {
    default_delay: 3,
    default_manpower: 300,
    default_attack: 30,
    default_defense: 30,
    default_evasion_rate: 0.2,
    can_move: Some(true),
};

pub(crate) const CAVALRY: Unit = Unit {
    default_delay: 1,
    default_manpower: 200,
    default_attack: 40,
    default_defense: 20,
    default_evasion_rate: 0.3,
    can_move: Some(true),
};

pub(crate) const ARTILLERY: Unit = Unit {
    default_delay: 5,
    default_manpower: 100,
    default_attack: 50,
    default_defense: 100,
    default_evasion_rate: 0.1,
    can_move: Some(true),
};

pub(crate) const MAGE: Unit = Unit {
    default_delay: 1,
    default_manpower: 10,
    default_attack: 40,
    default_defense: 200,
    default_evasion_rate: 0.5,
    can_move: Some(true),
};
pub(crate) const INFANTRY_ATTACKS: [Position; 2] =
    [Position { y: 0, x: 1 }, Position { y: 0, x: -1 }];

pub(crate) const CAVALRY_ATTACKS: [Position; 2] =
    [Position { y: -1, x: -1 }, Position { y: 1, x: -1 }];

pub(crate) const ARTILLERY_ATTACKS: [Position; 3] = [
    Position { y: -1, x: -2 },
    Position { y: 0, x: -3 },
    Position { y: 1, x: -2 },
];

pub(crate) const MAGE_ATTACKS: [Position; 8] = [
    Position { y: -1, x: -1 },
    Position { y: -1, x: 0 },
    Position { y: -1, x: 1 },
    Position { y: 1, x: -1 },
    Position { y: 1, x: 1 },
    Position { y: 1, x: 0 },
    Position { y: 0, x: 1 },
    Position { y: 0, x: -1 },
];
pub(crate) const INFANTRY_MOVES: [Position; 4] = [
    Position { y: -1, x: 0 },
    Position { y: 1, x: 0 },
    Position { y: 0, x: 1 },
    Position { y: 0, x: -1 },
];

pub(crate) const CAVALRY_MOVES: [Position; 4] = [
    Position { y: -1, x: -1 },
    Position { y: 0, x: -2 },
    Position { y: 1, x: -1 },
    Position { y: 0, x: 1 },
];

pub(crate) const ARTILLERY_MOVES: [Position; 4] = [
    Position { y: -1, x: -1 },
    Position { y: 0, x: -1 },
    Position { y: 1, x: -1 },
    Position { y: 0, x: 1 },
];

pub(crate) const MAGE_MOVES: [Position; 12] = [
    Position { y: -1, x: -1 },
    Position { y: -1, x: 0 },
    Position { y: -1, x: 1 },
    Position { y: 1, x: -1 },
    Position { y: 1, x: 1 },
    Position { y: 1, x: 0 },
    Position { y: 0, x: 1 },
    Position { y: 0, x: -1 },
    Position { y: -2, x: 0 },
    Position { y: 2, x: 0 },
    Position { y: 0, x: 2 },
    Position { y: 0, x: -2 },
];
