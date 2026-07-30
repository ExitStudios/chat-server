
pub mod api;
pub mod pages;
pub mod static_files;


// pub fn create_handler(
//     _request: HttpRequest,
//     file_path: &str,
//     content_type: &str,
//     code: StatusCode,
// ) -> HttpResponse {
//     let body = fs::read(file_path).unwrap();

//     HttpResponse::new(code, create_headers(&body, content_type), body)
// }
