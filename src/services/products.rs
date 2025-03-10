use crate::models::product::{self, Entity as ProductEntity};
use crate::schemas::product::CreateProduct;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, InsertResult, IntoActiveModel};

use super::ServiceError;

pub async fn create_product(
    db: &DatabaseConnection,
    payload: CreateProduct,
) -> Result<InsertResult<product::ActiveModel>, ServiceError> {
    let new_product = ProductEntity::insert(product::ActiveModel {
        name: ActiveValue::Set(payload.name.clone()),
        price: ActiveValue::Set(payload.price.clone()),
        ..Default::default()
    })
    .exec(db)
    .await;

    match new_product {
        Ok(new_product) => Ok(new_product),
        Err(_e) => Err(ServiceError::InternalServerError),
    }
}

pub async fn get_products(db: &DatabaseConnection) -> Result<Vec<product::Model>, ServiceError> {
    let products = ProductEntity::find().all(db).await;
    match products {
        Ok(products) => Ok(products),
        Err(_e) => Err(ServiceError::InternalServerError),
    }
}

pub async fn get_product_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<product::Model>, ServiceError> {
    let product = ProductEntity::find_by_id(id).one(db).await;
    match product {
        Ok(product) => Ok(product),
        Err(_e) => Err(ServiceError::InternalServerError),
    }
}

pub async fn update_product(
    db: &DatabaseConnection,
    id: i32,
    payload: CreateProduct,
) -> Result<product::Model, ServiceError> {
    let product = ProductEntity::find_by_id(id).one(db).await;
    match product {
        Ok(Some(product)) => {
            let mut active_model: product::ActiveModel = product.into_active_model();
            active_model.name = ActiveValue::Set(payload.name.clone());
            active_model.price = ActiveValue::Set(payload.price.clone());
            let updated_product = ProductEntity::update(active_model).exec(db).await;
            match updated_product {
                Ok(updated_product) => Ok(updated_product),
                Err(_e) => Err(ServiceError::InternalServerError),
            }
        }
        Ok(None) => Err(ServiceError::NotFound),
        Err(_e) => Err(ServiceError::InternalServerError),
    }
}

pub async fn delete_product(db: &DatabaseConnection, id: i32) -> Result<(), ServiceError> {
    let product = ProductEntity::find_by_id(id).one(db).await;
    match product {
        Ok(Some(product)) => {
            let active_model: product::ActiveModel = product.into_active_model();
            let _ = ProductEntity::delete(active_model).exec(db).await;
            Ok(())
        }
        Ok(None) => Err(ServiceError::NotFound),
        Err(_e) => Err(ServiceError::InternalServerError),
    }
}
