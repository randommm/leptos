use actix_web::{web, App, HttpResponse, HttpServer};
use actix_files::Files;
use leptos::*;
use hydration_test::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
      .service(Files::new("/pkg", "./pkg"))
      .route("/", web::get().to(
        || async {
          HttpResponse::Ok()
            .content_type("text/html")
            .body({
              let runtime = create_runtime();
              let html = run_scope(runtime, move |cx| {
                view! {
                  cx,
                  <App/>
                }.render_to_string().to_string()
              });
              runtime.dispose();
              format!(r#"<!DOCTYPE html>
              <html>
                <head>
                <script type="module">import init, {{ hydrate }} from '/pkg/hydration_test.js'; init().then(hydrate);</script>
                </head>
                <body>{html}</body>
              </html>
              "#)
            })
        }
      )
    ))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}