
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeekData {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub project: Option<String>,
    pub employee: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub activity: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    #[serde(rename = "__v")]
    pub v: Option<f64>,
    pub weeks: Option<HashMap<String, f64>>,
    pub comments: Option<HashMap<String, String>>,
    #[serde(rename = "dayPlanning)]")]
    pub day_planning: Option<HashMap<String, bool>>,
    #[serde(rename = "weekdayHours)]")]
    pub weekday_hours: Option<HashMap<String, HashMap<String, Option<f64>>>>,
    pub preliminary: Option<HashMap<String, serde_json::Value>>,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "_id")]
    pub id: String,
    //pub activities: Option<Vec<Activity>>,
    #[serde(rename = "customerId")]
    pub customer_id: Option<String>,
    pub finished: Option<bool>,
    pub name: String,
//    pub probability: Option<String>,
//    #[serde(rename = "updatedAt)]")]
//    pub updated_at: Option<String>,
//    #[serde(rename = "createdAt)]")]
//    pub created_at: Option<String>,
//    #[serde(rename = "startDate)]")]
//    pub start_date: Option<String>,
//    #[serde(rename = "refNo)]")]
//    pub ref_no: Option<String>,
//    #[serde(rename = "numberAndName)]")]
//    pub number_and_name: Option<String>,
//    #[serde(rename = "legacyUpdatedAt)]")]
//    pub legacy_created_at: Option<String>,
//    #[serde(rename = "id")]
//    pub legacy_id: Option<String>,
//    //pub phases: Vec<Phase>,
//    pub tags: Option<Vec<String>>,
//    #[serde(rename = "projectLead)]")]
//    pub project_lead: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Activity {
    #[serde(rename = "activityId)]")]
    pub activity_id: Option<String>,
    pub name: Option<String>,
    pub rate: Option<f64>,
    pub active: Option<bool>,
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub hours: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub email: String,
    pub title: Option<String>,
    pub image: Option<String>,
    pub user_id: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct CompactEmployee {
    pub name: String,
    pub image: Option<String>,
}

#[derive(Debug)]
pub struct CompactProject {
    pub project: String,
    pub employees: HashSet<CompactEmployee>,
}


