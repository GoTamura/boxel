use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Player {
    name: String,
    pos: (f32, f32, f32),
}


#[cfg(test)]
mod test {
    #[test]
    fn send() {
        let player = Player {
            pos: (0., 1., 0.),
            name: "Player1".into(),
        };
    }
}
