use serde_derive::{Deserialize, Serialize};

pub type GoalId = String;
pub type AudienceId = String;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Audience {
    pub id: AudienceId,
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
    pub audiences: Vec<Audience>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CheckingCapabilitiesLinks {
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Default)]
pub struct CheckRequest {
    pub content: String,
    pub document: Option<DocumentInfo>,
    pub checkOptions: CheckOptions
}


#[allow(non_snake_case)]
#[derive(Serialize, Debug, Default)]
pub struct CheckOptions {
    pub audienceId: Option<String>
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
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CheckResponseLinks {
    pub result: String,
}


#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CheckResult {
    //pub id: u64,
    pub quality: CheckResultQuality
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CheckResultLinks {}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CheckResultQuality {
    pub score: f64,
    pub status: String,
}




