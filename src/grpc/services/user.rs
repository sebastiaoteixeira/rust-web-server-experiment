use crate::grpc::proto::user::user_service_server::UserService;
use crate::grpc::proto::user::{
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse, Empty,
    GetAllUsersResponse, GetUserRequest, GetUserResponse, UpdateUserRequest, UpdateUserResponse,
    User,
};
use crate::schemas::user::CreateUser;
use crate::services::users as user_service;
use sea_orm::DatabaseConnection;
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct UserServiceImpl {
    pub db: DatabaseConnection,
}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        println!("gRPC create_user({}, {})", request.get_ref().name, request.get_ref().email);

        let payload = request.into_inner();

        match user_service::create_user(
            &self.db,
            CreateUser {
                name: payload.name,
                email: payload.email,
            },
        )
        .await
        {
            Ok(insert_result) => {
                let user = insert_result;
                Ok(Response::new(CreateUserResponse {
                    user: Some(User {
                        id: user.id,
                        name: user.name,
                        email: user.email,
                    }),
                }))
            }
            Err(_) => Err(Status::internal("Failed to create user")),
        }
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let id = request.into_inner().id;
        println!("gRPC get_user({})", id);

        match user_service::get_user_by_id(&self.db, id).await {
            Ok(Some(user)) => Ok(Response::new(GetUserResponse {
                user: Some(User {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                }),
            })),
            Ok(None) => Err(Status::not_found("User not found")),
            Err(_) => Err(Status::internal("Failed to get user")),
        }
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        let payload = request.into_inner();
        let user = payload.user.unwrap();
        println!("gRPC update_user({}, {})", user.id, user.name);
        match user_service::update_user(
            &self.db,
            user.id,
            CreateUser {
                name: user.name,
                email: user.email,
            },
        )
        .await
        {
            Ok(updated_user) => Ok(Response::new(UpdateUserResponse {
                user: Some(User {
                    id: updated_user.id,
                    name: updated_user.name,
                    email: updated_user.email,
                }),
            })),
            Err(_) => Err(Status::internal("Failed to update user")),
        }
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        let id = request.into_inner().id;
        println!("gRPC delete_user({})", id);

        match user_service::delete_user(&self.db, id).await {
            Ok(_) => Ok(Response::new(DeleteUserResponse {
                message: "User deleted successfully".to_string(),
            })),
            Err(_) => Err(Status::internal("Failed to delete user")),
        }
    }

    async fn get_all_users(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<GetAllUsersResponse>, Status> {
        println!("gRPC get_all_users()");

        match user_service::get_users(&self.db).await {
            Ok(users) => Ok(Response::new(GetAllUsersResponse {
                users: users
                    .into_iter()
                    .map(|user| User {
                        id: user.id,
                        name: user.name,
                        email: user.email,
                    })
                    .collect(),
            })),
            Err(_) => Err(Status::internal("Failed to get users")),
        }
    }
}
