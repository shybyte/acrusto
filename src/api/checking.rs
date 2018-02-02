//pub type GoalId = String;
//pub type TermSetId = String;
pub type AudienceId = String;
pub type LanguageId = String;
//pub type ContentFormatId = String;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Audience {
    id: AudienceId,
    displayName: String,
    language: LanguageId,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CheckingCapabilities {
    audiences: Vec<Audience>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct CheckRequest {
    content: String,
}

type CheckId = String;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct CheckResponse {
    id: CheckId,
}

