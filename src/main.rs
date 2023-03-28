use tide::Request;

#[derive(serde::Deserialize, serde::Serialize)]
struct Wizard {
    name: String,
    level: u8,
}

async fn read_all(_req: Request<()>) -> tide::Result<tide::Body> {
    let wizards = vec![
        Wizard {
            name: "Gandalf".to_string(),
            level: 100,
        },
        Wizard {
            name: "Merlin".to_string(),
            level: 64,
        },
    ];

    Ok(tide::Body::from_json(&wizards)?)
}

async fn create(mut req: Request<()>) -> tide::Result<String> {
    let wizard: Wizard = req.body_json().await?;
    Ok(format!("{} is level {}", wizard.name, wizard.level))
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/wizards").post(create).get(read_all);

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
