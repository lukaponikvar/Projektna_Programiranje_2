use crate::structs::project::Project;

async fn send_get(url: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?.text().await?;
    return Ok(res);
}

///Funkcija vrne seznam uporabnikov na glavnem strežniku.
pub async fn users() -> Vec<Project> {
    let result = send_get("http://127.0.0.1:7878/project".to_string()).await;
    let maybe_projects = match result {
        Ok(s) => serde_json::from_str(&s),
        Err(e) => panic!("{}", e),
    };
    let projects = match maybe_projects {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
    projects
}
