use parking_lot::RwLock;
use       serde::{ Deserialize , Serialize };
use         std::{sync::Arc, collections::HashMap};
use        warp::{ http , Filter };


mod data;
mod error;
mod handler;


#[derive( Clone , Debug , Deserialize , Serialize )]
struct Item {
    name     : String
  , quantity : i32
}


type ItemStore = HashMap<String , i32>;


#[derive( Clone )]
struct Store {
    grocery_list: Arc<RwLock<ItemStore>>
}


impl Store {
    pub fn new() -> Self {
        Store {
            grocery_list : Arc::new( RwLock::new( HashMap::new() ))
        }
    }
}


async fn read_grocery_list( store : Store ) -> Result<impl warp::Reply , warp::Rejection> {
    let r = store.grocery_list.read();

    Ok( warp::reply::json( &*r ))
}


async fn add_grocery_list_item( item : Item , store : Store ) -> Result<impl warp::Reply , warp::Rejection> {
    let store = &mut *store.grocery_list.read();

    let result = store.insert( item.name , item.quantity );

    match result {
        Some( _ ) =>  Ok( warp::reply::json( store ))
      , None      => Err( warp::reject() )  
    } 
}


fn json_body() -> impl Filter<Extract = ( Item , ) , Error = warp::Rejection> + Clone {
    warp::body::content_length_limit( 1024 * 16 )
               .and( warp::body::json() )
}


#[tokio::main]
async fn main() {
    let store        = Store::new();
    let store_filter =  warp::any().map( move || store.clone() );

    // let hello = warp::path!( "hello" / String ) 
    //                 .map( | name | format!( "Hello, {}!", name ));

    let add_items = warp::post()
                        .and( warp::path( "v1" ))
                        .and( warp::path( "grocery" ))
                        .and( warp::path( "list" ))
                        .and( warp::path::end() )
                        .and( json_body() )
                        .and( store_filter.clone() )
                        .and_then( add_grocery_list_item );

    let read_list = warp::get()
                        .and( warp::path( "v1" ))
                        .and( warp::path( "grocery" ))
                        .and( warp::path( "list" ))
                        .and( warp::path::end() )
                        .and( store_filter.clone() )
                        .and_then( read_grocery_list );

    let routes = add_items.or( read_list );

    warp::serve( routes )
        .run(( [ 127 , 0 , 0 , 1 ] , 8000 ))
        .await
}
