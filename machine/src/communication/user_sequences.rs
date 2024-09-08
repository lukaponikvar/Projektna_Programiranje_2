use crate::communication::get_and_post::send_get;
use crate::communication::users::users;
use crate::structs::custom_error::CustomError;
use crate::structs::project::Project;
use crate::structs::sequences::SequenceInfo;

///Funkcija od želenega uporabnika pridobi vsa njegova zaporedja.
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

///Funkcija od vseh uporabnikov pridobi vsa njihova zaporedja.
pub async fn user_sequences(register_ip: [u8; 4]) -> (Vec<Project>, Vec<Vec<SequenceInfo>>) {
    let users: Vec<Project> = users(register_ip).await;
    let mut projects = Vec::new();
    let mut all_sequences = Vec::new();
    for user in users {
        let sequences = user_sequence(&user).await;
        match sequences {
            Ok(s) => {
                projects.push(user);
                all_sequences.push(s)
            }
            Err(_) => continue,
        }
    }
    (projects, all_sequences)
}
