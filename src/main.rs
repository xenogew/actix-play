use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use slog::{info, o, Drain, Logger};
use slog_term::TermDecorator;

fn configure_log() -> Logger {
    let decorator: TermDecorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
    let console_drain = slog_async::Async::new(console_drain).build().fuse();

    slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}

#[derive(Serialize)]
struct RandomPasswordResponse {
    pw: String,
    message: String,
}

#[derive(Deserialize)]
struct RandomPasswordRequest {
    pwd_length: u8,
    symbol: bool,
    number: bool,
    lower: bool,
    upper: bool,
    x_similar: bool,
}

#[derive(Serialize)]
struct HealthCheckResponse {
    message: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[get("/health")]
async fn health_check() -> Result<impl Responder> {
    let message = HealthCheckResponse {
        message: String::from("OK"),
    };
    Ok(web::Json(message))
}

#[post("/api/v1/gen/password/random")]
async fn random_password(request: web::Json<RandomPasswordRequest>) -> Result<impl Responder> {
    let log = configure_log();

    info!(
        log,
        "len: {}, symbol: {}, number: {}, lower: {}, upper: {}, x-similar: {}",
        request.pwd_length,
        request.symbol,
        request.number,
        request.lower,
        request.upper,
        request.x_similar
    );
    let lower_case_letters = "abcdefghjkmnpqrstuvwxyz";
    let upper_case_letters = "ABCDEFGHJKMNPQRSTUVWXYZ";
    let numbers = "23456789";
    let symbols = "!@#%&+$-*=?^_";
    let similar = "Iil1|Lo0O";

    let available_stack = format!(
        "{}{}{}{}{}",
        if request.lower {
            lower_case_letters
        } else {
            ""
        },
        if request.upper {
            upper_case_letters
        } else {
            ""
        },
        if request.number { numbers } else { "" },
        if request.symbol { symbols } else { "" },
        if request.x_similar { "" } else { similar }
    );
    let available_size = available_stack.len();

    let mut carrier = vec![0; request.pwd_length as usize];
    for member in carrier.iter_mut() {
        let rand_number = rand::rng().random_range(0..available_size);
        let picked_char = available_stack.chars().nth(rand_number).unwrap();
        *member = picked_char as u32;
    }

    let mut generated: String = String::from("");
    for i in carrier.iter() {
        generated.push(char::from_u32(*i).unwrap());
    }

    let response = RandomPasswordResponse {
        pw: generated,
        message: String::from(""),
    };

    Ok(web::Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log = configure_log();

    info!(log, "Starting server at 0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(health_check)
            .service(random_password)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
