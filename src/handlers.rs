use actix_web::{web, HttpResponse};
use uuid::Uuid;
use sqlx::PgPool;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use crate::models::{CreateParcel, Parcel};

pub async fn add_parcel(
    pool: web::Data<PgPool>,
    item: web::Json<CreateParcel>,
) -> HttpResponse {
    let delivery_date = NaiveDate::parse_from_str(&item.delivery_date, "%Y-%m-%d")
        .unwrap(); // Parse NaiveDate

    // Convert NaiveDate to NaiveDateTime (Formatéjums))
    let delivery_date_time = delivery_date.and_hms(0, 0, 0);

    let parcel = Parcel {
        id: Uuid::new_v4(),
        sku: item.sku.clone(),
        description: item.description.clone(),
        delivery_address: item.delivery_address.clone(),
        delivery_date: delivery_date_time,
        shipping_cost: item.shipping_cost.clone(),
    };

    let result = sqlx::query!(
        r#"
        INSERT INTO parcels (id, sku, description, delivery_address, delivery_date, shipping_cost)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        parcel.id,
        parcel.sku,
        parcel.description,
        parcel.delivery_address,
        parcel.delivery_date.date(), // Convert NaiveDateTime to NaiveDate
        parcel.shipping_cost
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(parcel),
        Err(err) => {
            if let Some(db_err) = err.as_database_error() {
                if db_err.constraint().is_some() && db_err.constraint().unwrap() == "parcels_sku_key" {
                    return HttpResponse::BadRequest().body("Duplicate SKU");
                }
            }
            HttpResponse::InternalServerError().finish()
        }
    }
}


pub async fn filter_parcels_by_country(
    pool: web::Data<PgPool>,
    country: web::Path<String>,
) -> HttpResponse {
    let country = country.into_inner();

    let parcels = sqlx::query!(
        r#"
        SELECT id, sku, description, delivery_address, delivery_date, shipping_cost
        FROM parcels
        WHERE delivery_address ->> 'country' = $1
        "#,
        country
    )
    .fetch_all(pool.get_ref())
    .await;

    match parcels {
        Ok(rows) => {
            let parcels: Vec<Parcel> = rows
                .into_iter()
                .map(|row| Parcel {
                    id: row.id,
                    sku: row.sku,
                    description: row.description,
                    delivery_address: row.delivery_address,
                    delivery_date: row.delivery_date.and_hms(0, 0, 0), // Convert NaiveDate to NaiveDateTime
                    shipping_cost: row.shipping_cost,
                })
                .collect();

            HttpResponse::Ok().json(parcels)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn search_parcels_by_description(
    pool: web::Data<PgPool>,
    description: web::Path<String>,
) -> HttpResponse {
    let description = description.into_inner();

    let parcels = sqlx::query!(
        r#"
        SELECT id, sku, description, delivery_address, delivery_date, shipping_cost
        FROM parcels
        WHERE description ILIKE $1
        "#,
        format!("%{}%", description)
    )
    .fetch_all(pool.get_ref())
    .await;

    match parcels {
        Ok(rows) => {
            let parcels: Vec<Parcel> = rows
                .into_iter()
                .map(|row| Parcel {
                    id: row.id,
                    sku: row.sku,
                    description: row.description,
                    delivery_address: row.delivery_address,
                    delivery_date: row.delivery_date.and_hms(0, 0, 0), // Convert NaiveDate to NaiveDateTime
                    shipping_cost: row.shipping_cost,
                })
                .collect();

            HttpResponse::Ok().json(parcels)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
