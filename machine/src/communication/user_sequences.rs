use crate::communication::get_and_post::send_get;
use crate::communication::users::users;
use crate::structs::project::Project;
use crate::structs::sequences::SequenceInfo;

async fn user_sequence(project: Project) -> (Project, Vec<SequenceInfo>) {
    let url = format!("http://{}:{}/sequence", project.ip, project.port);
    let response = send_get(url).await;
    let string = match response {
        Ok(s) => s,
        Err(_) => return (project, Vec::new()),
    };
    let sequences = match serde_json::from_str(&string) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
    (project, sequences)
}

pub async fn user_sequences() -> Vec<(Project, Vec<SequenceInfo>)> {
    let users: Vec<Project> = users().await;
    let mut result = Vec::new();
    for user in users {
        let (project, sequence) = user_sequence(user).await;
        if sequence.len() == 0 {
            continue;
        } else {
            result.push((project, sequence));
        }
    }
    result
}
