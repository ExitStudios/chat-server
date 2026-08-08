use orbit::{
    http::response::HttpResponse,
    router::AddRoutes,
    server::{Server, ServerError},
};

fn main() -> Result<(), ServerError> {
    let mut server = Server::new();

    server.static_files_automatic("src/public");

    server.get(
        "/",
        Box::new(|_req| HttpResponse::ok().html("src/public/html/index.html")),
    );

    // server.get(
    //     "/style.css",
    //     Box::new(|_req| HttpResponse::ok().file("src/public/css/index.css")),
    // );

    // server.get(
    //     "/script.js",
    //     Box::new(|_req| HttpResponse::ok().file("src/public/js/script.js")),
    // );

    server.get(
        "/api/messages",
        Box::new(|_req| HttpResponse::ok().json(&Box::new("{}"))),
    );
    server.post("/api/messages", Box::new(|_req| HttpResponse::ok()));

    server.listen("127.0.0.1:3000")?;

    Ok(())
}
