use cfg_if::cfg_if;
use rocket::{get, routes, Build, Rocket};

#[get("/hello")]
fn hello() -> &'static str {
    "Hello from Rocket!"
}

fn client() -> Rocket<Build> {
    rocket::build().mount("/", routes![hello])
}

cfg_if! {
    if #[cfg(feature = "lambda")] {
        use netlify_lambda_http::{
            lambda::{lambda, Context},
            IntoResponse, Request, Response,
        };
        use rocket::local::blocking::Client;

        type LambdaError = Box<dyn std::error::Error + Sync + Send>;

        #[lambda(http)]
        #[tokio::main]
        async fn main(req: Request, _: Context) -> Result<impl IntoResponse, LambdaError> {
            let client = Client::untracked(client()).unwrap();

            let client_req = client.get(req.uri().to_string());
            let client_res = client_req.dispatch();

            let status = client_res.status().code;

            let body = client_res.into_bytes().unwrap();
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
