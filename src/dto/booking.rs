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

impl Default for BookingRequest {
    fn default() -> Self {
        Self {  
            first_name: "Opyat".into(),
            last_name: "Mamina".into(), 
            total_price: 1000, 
            deposit_paid: true, 
            booking_dates: BookingDates {
                check_in: "2026-08-08".into(),
                check_out: "2026-09-09".into(),
            }, 
            additional_needs: "Testing".into(), 
        }
    }    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookingResponse {
    
    #[serde(rename = "bookingid")]
    pub booking_id: i32,

    #[serde(rename = "booking")]
    pub booking_request: BookingRequest,

}
