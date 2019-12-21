// Api for database quering.
#[macro_use]
extern crate diesel;
pub mod api_schema;
pub mod schema;
#[cfg(test)]
mod test;
use diesel::sql_types::*;

sql_function!(fn num_micro_blocks(x: Int8, y: Text ) -> Int8);
// joinable!(micro_blocks -> macro_blocks (epoch, network));
