use axum::response::Html;

pub async fn index() -> Html<&'static str> {
    Html(include_str!("../../templates/index.html"))
}

pub async fn login_page() -> Html<&'static str> {
    Html(include_str!("../../templates/login.html"))
}

pub async fn register_page() -> Html<&'static str> {
    Html(include_str!("../../templates/register.html"))
}

pub async fn dashboard_page() -> Html<&'static str> {
    Html(include_str!("../../templates/dashboard.html"))
}

pub async fn assets_page() -> Html<&'static str> {
    Html(include_str!("../../templates/assets.html"))
}

pub async fn portfolios_page() -> Html<&'static str> {
    Html(include_str!("../../templates/portfolios.html"))
}
