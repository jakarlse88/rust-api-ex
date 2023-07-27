use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{ Client , Ws };
use surrealdb::opt::auth::Root;


pub async fn context() -> surrealdb::Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>( "127.0.0.1:8000" ).await?;

    db.signin( Root { username: "root" , password : "root" }).await?;
    db.use_ns( "test" ).use_db( "test" ).await?;

    Ok( db )
}
