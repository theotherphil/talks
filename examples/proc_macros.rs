
use serde::{Serialize, Deserialize};
use serde_json::{json, Result, Value};

#[derive(Debug, Serialize, Deserialize)]
struct Thing {
    field: i32
}

// Use this as an example of writing a trivial json pretty-printer
// Shows cargo, serde, proc macros

fn main() -> Result<()> {
    let data = r#"
    {
        "field": 15
    }
    "#;

    let t: Thing = serde_json::from_str(data)?;
    println!("{:?}", t);
    println!("{:?}", t.field);

    let v: Value = serde_json::from_str(data)?;
    println!("{:?}", v);
    println!("{}", v["field"]);

    let age_in_census = 50;
    let years_since_census = 22;

    let foo = json!({
        "name": "Foo Bar",
        "age": age_in_census + years_since_census,
        "cats": [
            "Misty",
            "Purdy"
        ]
    });

    println!("{:?}", foo["cats"]);

    let t = Thing { field: 99 };
    let j = serde_json::to_string(&t)?;
    println!("{}", j);
    let j = serde_json::to_string_pretty(&t)?;
    println!("{}", j);

    Ok(())
}
