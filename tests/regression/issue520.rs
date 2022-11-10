#![allow(clippy::float_cmp)]

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
enum E {
    Float(f32),
}

#[test]
fn test() {
    let e = E::Float(159.1);
    let v = serde_json_1400::to_value(e).unwrap();
    let e = serde_json_1400::from_value::<E>(v).unwrap();

    match e {
        E::Float(f) => assert_eq!(f, 159.1),
    }
}
