use serde_derive::{Deserialize, Serialize};

pub type GoalId = String;
pub type ContentFormatId = String;
pub type GoalIdString = String;
pub type GuidanceProfileId = String;
pub type ReferencePattern = String;

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct GuidanceProfile {
    pub id: GuidanceProfileId,
    displayName: GoalIdString,
    language: Language,
    termSets: Vec<TermSet>,
    goals: Vec<Goal>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Language {
    displayName: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Goal {
    id: GoalId,
    displayName: String,
    color: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct TermSet {
    displayName: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct CheckingCapabilities {
    pub guidanceProfiles: Vec<GuidanceProfile>,
    pub contentFormats: Vec<ContentFormat>,
    pub contentEncodings: Vec<ContentEncoding>,
    pub checkTypes: Vec<CheckType>,
    pub reportTypes: Vec<ReportType>,
    pub referencePattern: ReferencePattern,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct ContentFormat {
    id: ContentFormatId,
    displayName: String,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize, Debug)]
pub enum ContentEncoding {
    base64,
    none,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize, Debug)]
pub enum CheckType {
    batch,
    interactive,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize, Debug)]
pub enum ReportType {
    scorecard,
    extractedText,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct CheckingCapabilitiesLinks {}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Default)]
pub struct CheckRequest {
    pub content: String,
    pub document: Option<DocumentInfo>,
    pub checkOptions: CheckOptions,
}


#[allow(non_snake_case)]
#[derive(Serialize, Debug, Default)]
pub struct CheckOptions {
    pub guidanceProfileId: Option<String>
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




