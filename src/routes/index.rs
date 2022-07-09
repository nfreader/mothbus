use std::sync::Arc;

use axum::{response::IntoResponse, Extension};
use serde::Serialize;

use super::TemplateBase;

#[derive(Serialize)]
struct IndexTemplate {
    base: TemplateBase,
}

#[tracing::instrument]
pub async fn index(Extension(state): Extension<Arc<crate::State>>) -> impl IntoResponse {
    state.render_template(
        "index",
        IndexTemplate {
            base: TemplateBase {
                title: "home".into(),
            },
        },
    )
}
