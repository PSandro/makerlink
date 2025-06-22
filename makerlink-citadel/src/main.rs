// This file is part of MakerLink.
//
// MakerLink is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// MakerLink is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with Foobar. If not, see <https://www.gnu.org/licenses/>. 
//
// Copyright (C) 2025  Sandro Pischinger <mail+makerlink@sandropischinger.de>

use std::sync::Arc;

use axum::{
    extract::State, http::{StatusCode, Uri}, response::{Html, IntoResponse}, routing::get, Router
};
use tower_http::services::ServeDir;
use minijinja::{context, Environment};

struct AppState {
    tpl_env: Environment<'static>,
}

async fn index(
    State(state): State<Arc<AppState>>
    ) -> Result<Html<String>, StatusCode> {
    let tpl = state.tpl_env.get_template("index").unwrap();

    Ok(Html(tpl.render(context! {}).unwrap()))
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        format!("No route {}", uri)
    )
}

async fn run() {

    let mut tpl_env = Environment::new();
    tpl_env.add_template("base", include_str!("../templates/base.html"))
        .unwrap();
    tpl_env.add_template("index", include_str!("../templates/index.html"))
        .unwrap();

    let app_state = Arc::new(AppState { tpl_env} );

    let app = Router::new()
        .fallback(fallback)
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(index))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("[::]:5000")
        .await
        .unwrap();

    tracing::info!("running on http://{:?}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    run().await
}
