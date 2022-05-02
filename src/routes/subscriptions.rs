use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Subscription {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(subscription, connection_pool),
    fields(
        email = %subscription.email,
        name = %subscription.name
    )
)]
pub async fn subscribe(
    subscription: web::Form<Subscription>,
    connection_pool: web::Data<PgPool>,
) -> HttpResponse {
    match insert_subscriber(&connection_pool, &subscription).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber in the database",
    skip(subscription, connection_pool)
)]
pub async fn insert_subscriber(
    connection_pool: &PgPool,
    subscription: &Subscription,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscription.email,
        subscription.name,
        Utc::now()
    )
    .execute(connection_pool)
    .await
    .map_err(|e| {
        error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
