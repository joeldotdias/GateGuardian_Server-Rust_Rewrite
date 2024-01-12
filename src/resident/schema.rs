use serde::{ Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Deserialize, Debug)]
pub struct SaveResidentHomeSchema {
    pub flat: i64,
    pub building: String
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct UpdateResidentProfileSchema {
    pub name: String,
    pub aboutMe: String,
    pub phoneNo: String
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct UpdatePfpParams {
    pub pfpUrl: String
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct SavePersonSchema {
    pub name: String,
    pub email: String
}

#[derive(Serialize, Debug, FromRow)]
pub struct VisitorResidentDto {
    #[serde(rename="visitorId")]
    pub visitor_id: i32,
    pub name: String,
    #[serde(rename="phoneNo")]
    pub phone_no: String,
    #[serde(rename="hostEmail")]
    pub host_email: String,
    pub otp: String
}