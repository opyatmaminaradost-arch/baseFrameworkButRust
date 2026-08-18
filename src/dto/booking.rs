use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BookingDates {
    
    #[serde(rename = "checkin")]
    pub check_in: String,

    #[serde(rename = "checkout")]
    pub check_out: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct BookingRequest {
    #[serde(rename = "firstname")]
    pub first_name: String,

    #[serde(rename = "lastname")]
    pub last_name: String,

    #[serde(rename = "totalprice")]
    pub total_price: i32,

    #[serde(rename = "depositpaid")]
    pub deposit_paid: bool,

    #[serde(rename = "bookingdates")]
    pub booking_dates: BookingDates,

    #[serde(rename = "additionalneeds")]
    pub additional_needs: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookingResponse {
    
    #[serde(rename = "bookingid")]
    pub booking_id: i32,

    #[serde(rename = "booking")]
    pub booking_request: BookingRequest,

}
