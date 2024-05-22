use serde::{Deserialize, Serialize};

#[test]
fn rename_test() {
    #[derive(Serialize, Deserialize, Debug)]
    struct User {
        // Serialize and deserialize this field with the given name instead of its Rust name.
        #[serde(rename = "ser_deser_id")] // 设置该字段序列化和反序列化名称ser_deser_name
        id: i32,
        #[serde(rename(serialize = "ser_name"))]
        // 设置该字段序列化名称为ser_name(反序列化名称仍为name)
        name: String,
        #[serde(rename(deserialize = "deser_age"))]
        // 设置该字段反序列化名称为ser_name(序列化名称仍为age)
        age: i32,
        #[serde(rename(serialize = "ser_grade", deserialize = "deser_grade"))]
        // 设置该字段序列化名称为ser_grade,反序列化名称为deser_grade
        grade: char,
    }

    let user = User {
        id: 1,
        name: String::from("Alice"),
        age: 30,
        grade: 'A',
    };

    let serialized = serde_json::to_string(&user).unwrap();
    println!("{}", serialized); // {"ser_deser_id":1,"ser_name":"Alice","age":30,"ser_grade":"A"}

    let data = r#"{ "ser_deser_id": 1, "name": "Alice", "deser_age": 30 , "deser_grade": "B" }"#;
    let deserialized: User = serde_json::from_str(data).unwrap();
    println!("{:?}", deserialized); // print->User { id: 1, name: "Alice", age: 30, grade: 'B' }
}

#[test]
fn default_test() {
    #[derive(Serialize, Deserialize, Debug)]
    struct User {
        id: i32,
        // If the value is not present when deserializing, use the Default::default().
        #[serde(default)]
        name: String,
        // If the value is not present when deserializing, call a function to get a default value. The given function must be callable as fn() -> T.
        #[serde(default = "default_age")]
        age: i32,
    }

    fn default_age() -> i32 {
        18
    }

    let json_data = [r#"{ "id": 1, "name": "Alice" }"#, r#"{ "id": 2 }"#];

    /*
    print->
    User { id: 1, name: "Alice", age: 18 }
    User { id: 2, name: "", age: 18 }
     */
    for json in json_data.iter() {
        let deserialized: User = serde_json::from_str(json).unwrap();
        println!("{:?}", deserialized);
    }
}

#[test]
fn flatten_test() {}
