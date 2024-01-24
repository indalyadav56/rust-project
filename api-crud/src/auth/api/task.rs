use actix_web::{ web::Json, web::Path, get, post, patch, delete};


#[get("/get-task-data")]
pub async fn get_task() -> Json<String>{
    Json("Indal Yadav".to_string())
}

#[post("/add-task")]
pub async fn add_task() -> Json<String>{
    Json("Indal Yadav".to_string())
}