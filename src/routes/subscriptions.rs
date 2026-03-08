use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name= %form.name
    );
    let _request_span_guard = request_span.enter();

    if form.email.is_empty() || form.name.is_empty() {
        return HttpResponse::BadRequest().finish();
    }

    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    match sqlx::query(
        r#"
        INSERT INTO subscriptions (id, email, name, created_at)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(&form.email)
    .bind(&form.name)
    .bind(Utc::now())
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
