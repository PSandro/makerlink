// This file is part of MakerLink.
//
// MakerLink is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// MakerLink is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with Foobar. If not, see <https://www.gnu.org/licenses/>. 
//
// Copyright (C) 2025  Sandro Pischinger <mail+makerlink@sandropischinger.de>

use axum::{
    http::{StatusCode, Uri}, response::IntoResponse,
    routing::get, Router,
};


pub async fn index() -> &'static str {
    "1nd3x"
}

pub async fn fallback(uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        format!("No route {}", uri)
    )
}

pub async fn run() {

    let app = Router::new()
        .fallback(fallback)
        .route("/", get(index));

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
