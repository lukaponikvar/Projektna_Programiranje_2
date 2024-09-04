use crate::communication::get_and_post::send_get;
use crate::communication::users::users;
use crate::structs::project::Project;
use crate::structs::sequences::SequenceInfo;

///Funkcija od želenega uporabnika pridobi vsa njegova zaporedja.
async fn user_sequence(project: Project) -> Vec<SequenceInfo> {
    let url = format!("http://{}:{}/sequence", project.ip, project.port);
    let string = match send_get(url).await {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    let sequences = match serde_json::from_str(&string) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
    sequences
}

///Funkcija od vseh uporabnikov pridobi vsa njihova zaporedja.
pub async fn user_sequences() -> (Vec<Project>, Vec<Vec<SequenceInfo>>) {
    let users: Vec<Project> = users().await;
    let mut projects = Vec::new();
    let mut all_sequences = Vec::new();
    for user in users {
        let sequences = user_sequence(user.clone()).await;
        projects.push(user);
        all_sequences.push(sequences)
    }
    (projects, all_sequences)
}
