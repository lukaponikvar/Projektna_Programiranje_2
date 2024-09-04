use crate::communication::get_and_post::send_get;
use crate::communication::users::users;
use crate::structs::project::Project;
use crate::structs::sequences::SequenceInfo;

async fn user_sequence(project: Project) -> Vec<SequenceInfo> {
    let url = format!("http://{}:{}/sequence", project.ip, project.port);
    let response = send_get(url).await;
    let string = match response {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    let sequences = match serde_json::from_str(&string) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
    sequences
}

pub async fn user_sequences() -> (Vec<Project>, Vec<Vec<SequenceInfo>>) {
    let users: Vec<Project> = users().await;
    let mut projects = Vec::new();
    let mut sequences = Vec::new();
    for user in users {
        let sequence = user_sequence(user.clone()).await;
        projects.push(user);
        sequences.push(sequence)
    }
    (projects, sequences)
}
