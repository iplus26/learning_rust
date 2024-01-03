use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Form;
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(new_index))
            .route("/gcd", web::post().to(handle_form_gcd))
    });

    println!("Serving on http://localhost:3000...");

    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run()
        .await
}

async fn new_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                    <input type="text" name="n"/>
                    <input type="text" name="m"/>
                    <button type="submit">Compute GCD</button>
                </form>
            "#
        )
}

// de-serialize the form data into a struct
#[derive(Deserialize)]
struct GcdParameters {
    n: u32,
    m: u32,
}

async fn handle_form_gcd(form: Form<GcdParameters>) -> impl Responder {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response = format!(
        "The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
        form.n, form.m, gcd(form.n, form.m)
    );

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn gcd(mut n: u32, mut m: u32) -> u32 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m = m % n;
    }
    n
}