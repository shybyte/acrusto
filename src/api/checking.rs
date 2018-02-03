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
pub struct CheckRequest {
    pub content: String,
}

type CheckId = String;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CheckResponse {
    pub id: CheckId,
    pub links: CheckResponseLinks
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CheckResponseLinks {
    pub status: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CheckingStatus {
    pub id: CheckId,
    pub state: String,
    pub percent: f64,
    pub message: String,
    pub links: CheckingStatusLinks
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CheckingStatusLinks {
    pub result: String,
}


#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CheckResult {
    pub id: CheckId,
    pub quality: CheckResultQuality
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CheckResultQuality {
    pub score: f64,
    pub status: String,
}



