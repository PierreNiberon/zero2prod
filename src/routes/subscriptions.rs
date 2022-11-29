use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

// to test on PS : Invoke-WebRequest http://127.0.0.1:8000/subscriptions -Method POST -Body "email=john_doe%40outlook.fr&name=John"

// FormData is a struct that will be populated by the web::Form<FormData> extractor
// web::Form here will take the http request in the path and extract the data from it
// serde will do the deserialisation for us
#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // we tracing here because there is an IO operation. We tracing the information to have more insigths
    // we build a request ID to follow the tracings more easily in case of concurent access.
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
            "Adding a new subscriber.",
            %request_id,
    subscriber_email = %form.email,
    subscriber_name = %form.name
    );
    let _request_span_guard = request_span.enter();

    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    //we use the query! macro to write our dynamic insert query
    // we execute it against the connection pool
    // we match it directly to send either Ok or Err variant
    match sqlx::query!(
        r#"
INSERT INTO subscriptions (id, email, name, subscribed_at)
VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        // we tracing here in case of sucess or error
        Ok(_) => {
            tracing::info!(
                "Request ID : '{}' - New subscriber registered in database.",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "Request ID : '{}' - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
