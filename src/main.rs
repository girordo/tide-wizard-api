use tide::Request;
use tide::Response;

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
    let res = Response::builder(tide::StatusCode::Created)
        .body("Created a new wizard")
        .build();
    Ok(res)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/wizards").nest({
        let mut api = tide::new();

        api.at("/")
            .post(|_req: Request<()>| async move { Ok("Created!") });

        api.at("/:id")
            .get(|_req: Request<()>| async move { Ok("Read!") });

        api
    });

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
