pub mod read {
    use actix_web::HttpResponse;
    pub async fn read_buy() -> HttpResponse {
        let text = "asdasdas";
        HttpResponse::Ok().body(text)
    }
}
