use     serde::{ Deserialize , Serialize };
use surrealdb::sql::Thing;


#[derive( Debug , Serialize )]
struct Name<'a> {
    first : &'a str
  , last  : &'a str
    }


#[derive( Debug , Serialize )]
enum Responsibility {
    Marketing
  , Other
    }


#[derive( Debug , Serialize )]
pub( in crate::data ) struct Person<'a> {
    title          : &'a str
  , name           : Name<'a>
  , responsibility : Responsibility
    }


#[derive( Debug , Deserialize )]
pub( in crate::data ) struct Record {
    id: Thing
    }