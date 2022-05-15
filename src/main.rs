use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use rand::Rng;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/api/v1/gen/password/random")]
async fn random_password() -> impl Responder {
    const DEFAULT_LENGTH: usize = 16;
    let is_special = true;
    let lower_case_letters = "abcdefghijklmnopqrstuvwxyz";
    let upper_case_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "@#=+!$%&?_";

    let available_stack = format!("{}{}{}", lower_case_letters, upper_case_letters, numbers);
    let available_stack = format!("{}{}", available_stack, if is_special { symbols } else { "" });
    let available_size = available_stack.len();

    let mut carrier = [0; DEFAULT_LENGTH];
    for member in carrier.iter_mut() {
        let rand_number = rand::thread_rng().gen_range(0..available_size);
        let picked_char = available_stack.chars().nth(rand_number).unwrap();
        *member = picked_char as u32;
    }

    let mut generated: String = String::from("");
    for i in carrier.iter() {
        generated.push(char::from_u32(*i).unwrap());
    }
    info!("{}", generated);
    HttpResponse::Ok().body(generated)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(random_password)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
