use super::{get_and_post::send_get, users::users};
use crate::structs::{custom_error::CustomError, project::Project, sequences::SequenceInfo};
use futures::future::join_all;

async fn user_sequence(project: &Project) -> Result<Vec<SequenceInfo>, CustomError> {
    let url = format!("http://{}:{}/sequence", project.ip, project.port);
    let string = match send_get(&url).await {
        Ok(s) => s,
        Err(e) => return Err(CustomError::new(e.to_string())),
    };
    match serde_json::from_str(&string) {
        Ok(s) => Ok(s),
        Err(e) => return Err(CustomError::new(e.to_string())),
    }
}

/// Returns pair of vectors, one with all users and the other one with vectors of all user sequences.
/// The user and its sequences match at the same position.
pub async fn user_sequences(
    register_ip: [u8; 4],
    register_port: u16,
) -> (Vec<Project>, Vec<Vec<SequenceInfo>>) {
    let users: Vec<Project> = match users(register_ip, register_port).await {
        Ok(u) => u,
        Err(_) => Vec::new(),
    };
    let mut projects = Vec::new();
    let mut all_sequences = Vec::new();
    let mut futures = Vec::new();
    for user in &users {
        futures.push(user_sequence(&user));
    }
    let sequences_vector = join_all(futures).await;
    for index in 0..users.len() {
        let sequences = sequences_vector[index].clone();
        match sequences {
            Ok(s) => {
                projects.push(users[index].clone());
                all_sequences.push(s)
            }
            Err(_) => continue,
        }
    }
    (projects, all_sequences)
}
