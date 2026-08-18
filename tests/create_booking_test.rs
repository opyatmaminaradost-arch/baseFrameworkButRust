use baseFramework::dto::booking::{
    BookingDates, BookingRequest, BookingResponse
};

#[tokio::test]
async fn create_booking(){
    let request = BookingRequest::default();

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

    let booking_id = body.booking_id;

    println!("{}", booking_id);

    let response_by_get= client
        .get(format!("https://restful-booker.herokuapp.com/booking/{}",
                      booking_id))
        .send()
        .await
        .unwrap();
    let body_from_get: BookingRequest = 
        response_by_get.json().await.unwrap();

    assert_eq!(request.first_name, body_from_get.first_name)

}
