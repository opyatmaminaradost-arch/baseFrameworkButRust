use baseFramework::dto::booking::{
    BookingDates, BookingRequest, BookingResponse
};

#[tokio::test]
async fn create_booking(){
    let request = BookingRequest {
        first_name: "Jim".into(),
        last_name: "Brown".into(),
        total_price: 1242,
        deposit_paid: true,
        booking_dates: BookingDates {
            check_in: "2026-08-17".into(),
            check_out: "2026-08-18".into(),
        },
        additional_needs: "Breakfast".into(),
    };

    let client = reqwest::Client::new();

    let response = client
        .post("https://restful-booker.herokuapp.com/booking")
        .json(&request)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    
    let body: BookingResponse = 
        response.json().await.unwrap();

    assert!(body.booking_id > 0);
}
