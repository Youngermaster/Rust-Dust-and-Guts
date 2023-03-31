async fn get_users() -> Result<Json<Vec<User>>, Error> {
    let users = User::find_all().await?;
    Ok(Json(users))
}

async fn get_user(id: Path<String>) -> Result<Json<User>, Error> {
    let user = User::find_by_id(&id).await?;
    Ok(Json(user))
}

async fn create_user(user: Json<NewUser>) -> Result<HttpResponse, Error> {
    let result = User::create(user.into_inner()).await;
    match result {
        Ok(user) => Ok(HttpResponse::Created().json(user)),
        Err(_) => Err(error::ErrorInternalServerError("Failed to create user")),
    }
}

async fn update_user(id: Path<String>, user: Json<UpdateUser>) -> Result<HttpResponse, Error> {
    let result = User::update(&id, user.into_inner()).await;
    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Err(error::ErrorInternalServerError("Failed to update user")),
    }
}

async fn delete_user(id: Path<String>) -> Result<HttpResponse, Error> {
    let result = User::delete(&id).await;
    match result {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(_) => Err(error::ErrorInternalServerError("Failed to delete user")),
    }
}
