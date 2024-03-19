pub mod home_utils {
    use actix_web::{get, web, HttpResponse};
    use futures::{future::ok, stream::once};
    use serde::{de::value::Error, Deserialize, Serialize};

    #[derive(Serialize)]
    struct PathService {
        name: String,
        number: i32,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Name2 {
        pub name: Option<Vec<String>>,
        pub last_name: Option<String>,
    }
    #[get("/")]
    pub async fn read() -> HttpResponse {
        let text = "asdasdas";

        HttpResponse::Ok().body(text)
    }

    #[get("/{name}/{number}")]
    pub async fn read_dynamic(
        path: web::Path<(String, usize)>,
        data: web::Json<Name2>,
    ) -> HttpResponse {
        let (_, number) = path.into_inner();
        let mut values = vec![];
        loop {
            values.push(&data);
            if values.len() > number {
                break;
            }
        }
        let json = serde_json::to_string(&values).unwrap();
        HttpResponse::Ok().body(json)
    }
    #[get("/stream")]
    async fn stream() -> HttpResponse {
        let body = once(ok::<_, Error>(web::Bytes::from_static(b"test dfdf")));

        HttpResponse::Ok()
            .content_type("application/json")
            .streaming(body)
    }
}
