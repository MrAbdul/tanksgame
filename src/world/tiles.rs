
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum TileType {
    Wall,
    Empty,
    PlayerSpawn,
    EnemySpawn,   
    Target,       
    Special,      
}


impl TileType {
    pub(crate) fn from_char(c: char) -> Option<TileType> {
        match c {
            '#' => Some(TileType::Wall),
            '.' => Some(TileType::Empty),
            'P' => Some(TileType::PlayerSpawn),
            'E' => Some(TileType::EnemySpawn),
            'T' => Some(TileType::Target),
            'S' => Some(TileType::Special),
            _ => None,
        }
    }
}