use cfg_if::cfg_if;
use rocket::{fs::FileServer, get, routes, Build, Rocket};

#[get("/hello")]
fn hello() -> &'static str {
    "Hello from Rocket!"
}

fn client() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/", FileServer::from("public"))
}

cfg_if! {
    if #[cfg(feature = "lambda")] {
        use netlify_lambda_http::{
            lambda::{lambda, Context},
            IntoResponse, Request, Response,
        };
        use rocket::local::asynchronous::Client;

        type LambdaError = Box<dyn std::error::Error + Sync + Send>;

        #[lambda(http)]
        #[tokio::main]
        async fn main(req: Request, _: Context) -> Result<impl IntoResponse, LambdaError> {
            let client = Client::untracked(client()).await.unwrap();

            let client_req = client.get(req.uri().path_and_query().unwrap().as_str());
            let client_res = client_req.dispatch().await;

            let status = client_res.status().code;

            let body = client_res.into_bytes().await.unwrap();
            let res = Response::builder().status(status).body(body).unwrap();

            Ok(res)
        }
    } else {
        #[rocket::launch]
        fn launch() -> _ {
            client()
        }
    }
}
