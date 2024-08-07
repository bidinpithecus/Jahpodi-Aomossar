use axum::response::Html;

pub async fn show_home_page() -> Html<String> {
    let html_content = tokio::fs::read_to_string("static/home.html").await.unwrap();
    Html(html_content)
}
