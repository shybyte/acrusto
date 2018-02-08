pub type GoalId = String;
pub type AudienceId = String;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Audience {
    id: AudienceId,
    displayName: String,
    language: Language,
    termSets: Vec<TermSet>,
    goals: Vec<Goal>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Language {
    displayName: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Goal {
    id: GoalId,
    displayName: String,
    color: String
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TermSet {
    displayName: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CheckingCapabilities {
    audiences: Vec<Audience>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Default)]
pub struct CheckRequest {
    pub content: String,
    pub document: Option<DocumentInfo>
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct DocumentInfo {
    pub reference: Option<String>
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



