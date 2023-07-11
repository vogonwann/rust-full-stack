use actix_web::{web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

// just for commit
#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;
    
    // let pool = actix_web::web::Data::new(pool);
    let film_repository = api_lib::film_repository::PostgresFilmRepository::new(pool);
    // let film_repository = actix_web::web::Data::new(film_repository);
    // let film_repository: actix_web::web::Data<Box<dyn api_lib::film_repository::FilmRepository>> =
    //     actix_web::web::Data::new(Box::new(film_repository));
    let film_repository = actix_web::web::Data::new(film_repository); // use generics instead of the dynamics

    let config = move |cfg: &mut ServiceConfig| { 
        // cfg.app_data(pool)
        cfg.app_data(film_repository)
            .configure(api_lib::health::service)
            // .configure(api_lib::films::service);     
            .configure(api_lib::films::service::<api_lib::film_repository::PostgresFilmRepository>); // use generics insted of the dynamics     
    };

    Ok(config.into())
}
