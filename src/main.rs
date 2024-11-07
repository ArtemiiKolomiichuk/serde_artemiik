use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct User{
    name: String,
    email: String,
    birthdate: String,
}



fn main() {
    let user : User = User{
        name: "ArtemiiK".to_string(),
        email: "artemii.kolomiichuk@ukma.edu.ua".to_string(),
        birthdate: "2003-07-19".to_string(),
    };

    let json = serde_json::to_string(&user).expect("Error while serializing");
    println!("{}", json);

    let deserealized = serde_json::from_str::<User>(&json).expect("Error while deserializing");
    println!("{:#?}", deserealized);
}
