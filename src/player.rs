#[macro_export]
macro_rules! player {
    ($id:expr) => {
        Player::from_id($id.to_string())
    };
}


pub struct Player {
    id: String,
}


impl Player {
    fn from_id(id: String) -> Player {
        Player {
            id
        }
    }
}
