use super::ServiceError;
use crate::models::user::{self, Entity as UserEntity};
use crate::schemas::user::CreateUser;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, IntoActiveModel};

pub async fn create_user(
    db: &DatabaseConnection,
    payload: CreateUser,
) -> Result<user::Model, ServiceError> {
    let new_user = UserEntity::insert(user::ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(payload.name.clone()),
        email: ActiveValue::Set(payload.email.clone()),
    })
    .exec(db)
    .await;

    match new_user {
        Ok(insert_result) => Ok(user::Model {
            id: insert_result.last_insert_id,
            name: payload.name,
            email: payload.email,
        }),
        Err(_e) => Err(ServiceError::InternalServerError),
    }
}

pub async fn get_users(db: &DatabaseConnection) -> Result<Vec<user::Model>, ServiceError> {
    let users = UserEntity::find().all(db).await;
    match users {
        Ok(users) => Ok(users),
        Err(_e) => Err(ServiceError::InternalServerError),
    }
}

pub async fn get_user_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<user::Model>, ServiceError> {
    let user = UserEntity::find_by_id(id).one(db).await;
    match user {
        Ok(user) => Ok(user),
        Err(_e) => Err(ServiceError::InternalServerError),
    }
}

pub async fn update_user(
    db: &DatabaseConnection,
    id: i32,
    payload: CreateUser,
) -> Result<user::Model, ServiceError> {
    let user = UserEntity::find_by_id(id).one(db).await;
    match user {
        Ok(Some(user)) => {
            let mut active_model: user::ActiveModel = user.into_active_model();
            active_model.name = ActiveValue::Set(payload.name);
            active_model.email = ActiveValue::Set(payload.email);
            let updated_user = UserEntity::update(active_model).exec(db).await;
            match updated_user {
                Ok(updated_user) => Ok(updated_user),
                Err(_e) => Err(ServiceError::InternalServerError),
            }
        }
        Ok(None) => Err(ServiceError::NotFound),
        Err(_e) => Err(ServiceError::InternalServerError),
    }
}

pub async fn delete_user(db: &DatabaseConnection, id: i32) -> Result<(), ServiceError> {
    let user = UserEntity::find_by_id(id).one(db).await;
    match user {
        Ok(Some(user)) => {
            let active_model: user::ActiveModel = user.into_active_model();
            let _ = UserEntity::delete(active_model).exec(db).await;
            Ok(())
        }
        Ok(None) => Err(ServiceError::NotFound),
        Err(_e) => Err(ServiceError::InternalServerError),
    }
}
