pub( in crate::data ) mod model;
pub( in crate::data ) mod db;


use crate::data::model::{ Person , Record };


pub async fn read() -> surrealdb::Result<Record> {
}


pub async fn read_all() -> surrealdb::Result<Record> {
}


pub async fn insert( entity : Person<'_> ) -> surrealdb::Result<()> {
    let context  = db::context().await?;
    let inserted = context 
                    .create( "person" )
                    .content( entity )
                    .await?;

    dbg!( inserted );

    Ok( () )
}


pub async fn update() -> surrealdb::Result<Record> {

}


pub async fn delete() -> surrealdb::Result<Record> {
}
