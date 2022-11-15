use std::any::Any;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Face {
    pub nose: Option<String>,
    pub eye: Option<i32>,
}


#[derive(Debug, Serialize, Deserialize)]
struct Person {
    pub face: Vec<Face>,
    pub name: String,
}


use serde::ser::{self, Serializer, SerializeStruct, SerializeTupleStruct, Impossible};
use std::fmt::{self, Display};

fn type_of<T>(_: &T) -> String {
    let type_name = std::any::type_name::<T>();
    let mut short_name = String::new();

    // A typename may be a composition of several other type names (e.g. generic parameters)
    // separated by the characters that we try to find below.
    // Then, each individual typename is shortened to its last path component.
    // Note: Instead of `find`, `split_inclusive` would be nice but it's still unstable...
    let mut remainder = type_name;
    while let Some(index) = remainder.find(&['<', '>', '(', ')', '[', ']', ',', ';'][..]) {
        let (path, new_remainder) = remainder.split_at(index);
        // Push the shortened path in front of the found character
        short_name.push_str(path.rsplit(':').next().unwrap());
        // Push the character that was found
        let character = new_remainder.chars().next().unwrap();
        short_name.push(character);
        // Advance the remainder
        if character == ',' || character == ';' {
            // A comma or semicolon is always followed by a space
            short_name.push(' ');
            remainder = &new_remainder[2..];
        } else {
            remainder = &new_remainder[1..];
        }
    }
    // The remainder will only be non-empty if there were no matches at all
    if !remainder.is_empty() {
        // Then, the full typename is a path that has to be shortened
        short_name.push_str(remainder.rsplit(':').next().unwrap());
    }

    short_name
}


#[derive(Serialize)]
struct Vec2(f32, f32);


fn main() {
    let face = Face {
        nose: Some("big face".to_string()),
        eye: Some(2),
    };

    let aa = 32;
    // println!("{}", type_of(&face));
    // let person :Person =  serde_json_1400::from_str("{\"PersonObject\":{\"face\":\{\"FaceObject\":{\"nose\":\"small\",\"eye\":2}},\"name\":\"lyf\"}}").unwrap();
    // println!("{:?}",person)

    //
    let face_list = vec![Face {
        nose: Some("big".to_string()),
        eye: Some(2),
    }, Face {
        nose: Some("big".to_string()),
        eye: Some(2),
    }, Face {
        nose: Some("big".to_string()),
        eye: Some(2),
    }];


    //
    let face_list2 = vec![Face {
        nose: Some("big".to_string()),
        eye: Some(2),
    }, Face {
        nose: Some("big".to_string()),
        eye: Some(2),
    }, Face {
        nose: Some("big".to_string()),
        eye: Some(2),
    }];

    let person = Person {
        face: face_list,
        name: "lyf".to_string(),
    };


    let person2 = Person {
        face: face_list2,
        name: "ym".to_string(),
    };

    // serde_json_1400::to_string(&face);

    println!("{}", serde_json_1400::to_string(&Face {
        nose: Some("xx".to_string()),
        eye: Some(1),
    }).unwrap());

    let face: Face = serde_json_1400::from_str("{\"FaceObject\":{\"nose\":\"xx\",\"eye\":1}}").unwrap();


    println!("{}", serde_json_1400::to_string(&vec![person, person2]).unwrap());

    // println!("{}", serde_json_1400::to_string(&person).unwrap());

    // println!("{}", type_of(&face_list));
    // face_list.type_id();

    // println!("{}", serde_json_1400::to_string(&face_list2).unwrap());

    // let face:Face = serde_json_1400::from_str("{\"FaceObject\":{\"nose\":\"big face\",\"eye\":2}}").unwrap();

    // let faces:Vec<Face> = serde_json_1400::from_str("[{\"nose\":\"big\",\"eye\":2},{\"nose\":\"small\",\"eye\":2},{\"nose\":\"small\",\"eye\":2}]").unwrap();
    // let faces: Vec<Face> = serde_json_1400::from_str("{\"FaceListObject\":{\"FaceObject\":[{\"nose\":\"big\",\"eye\":2},{\"nose\":\"small\",\"eye\":2},{\"nose\":\"small\"}]}}").unwrap();

    let person: Vec<Person> = serde_json_1400::from_str("{
    \"PersonListObject\": {
        \"PersonObject\": [
            {
                \"face\": {
                    \"FaceObject\": [
                        {
                            \"nose\": \"big\",
                            \"eye\": 2
                        },
                        {
                            \"nose\": \"big\",
                            \"eye\": 2
                        },
                        {
                            \"nose\": \"big\",
                            \"eye\": 2
                        }
                    ]
                },
                \"name\": \"lyf\"
            },
            {
                \"face\": {
                    \"FaceObject\": [
                        {
                            \"nose\": \"big\",
                            \"eye\": 2
                        },
                        {
                            \"nose\": \"big\",
                            \"eye\": 2
                        },
                        {
                            \"nose\": \"big\",
                            \"eye\": 2
                        }
                    ]
                },
                \"name\": \"ym\"
            }
        ]
    }
}").unwrap();
}