use std::collections::HashMap;
use std::fmt::format;
use std::path::Path;

#[tokio::main]
async fn main() {
    match std::env::args().nth(1).expect("No action given").as_str() {
        "create" => create().await,
        "ls" => list().await,
        "find" => find().await,
        "update" => update().await,
        "delete" => delete().await,
        _ => std::process::exit(exitcode::USAGE),
    }
}

async fn create() {
    let path_arg = std::env::args().nth(2).expect("No path given");
    let path = Path::new(path_arg.as_str());
    if !path.is_file() {
        std::process::exit(exitcode::OSFILE);
    }

    let content = std::fs::read_to_string(&path).expect("Could not read file");

    let description_arg = std::env::args().nth(3);
    let description: String;

    let mut create_script_dto = HashMap::new();
    create_script_dto.insert("filename", path.file_name().unwrap().to_str());
    create_script_dto.insert("content", Some(content.as_str()));
    if description_arg.is_some() {
        description = std::env::args().nth(3).unwrap(); // FIXME: unpack every last argument or make it available by --description or something
        create_script_dto.insert("description", Some(description.as_str()));
    }

    let client = reqwest::Client::new();
    let resp = client
        .post("http://localhost:3000/scripts")
        .json(&create_script_dto)
        .send()
        .await;

    if resp.err().is_some() {
        std::process::exit(exitcode::TEMPFAIL);
    }

    println!("Successful");
}

async fn list() {
    let client = reqwest::Client::new();
    let resp = client
        .get("http://localhost:3000/scripts")
        .send()
        .await
        .expect("")
        .json::<serde_json::Value>()
        .await;
    println!("{:#?}", resp);
}

async fn find() {}

async fn update() {}

async fn delete() {
    let uuid = std::env::args().nth(2).expect("No uuid given");
    let client = reqwest::Client::new();
    let resp = client
        .delete(format!("http://localhost:3000/scripts/{uuid}"))
        .send()
        .await;
    println!("Deleted");
}
