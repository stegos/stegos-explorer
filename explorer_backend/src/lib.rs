// Api for database quering.
#[macro_use]
extern crate diesel;
pub mod api_schema;
pub mod schema;
#[cfg(test)]
mod test;
use diesel::sql_types::*;

sql_function!(fn num_micro_blocks(x: Int8, y: Text ) -> Int8);

sql_function!(fn num_transactions(x: Int8, y: Text ) -> Int8);
sql_function!(fn is_spent_awards(x: Int8, y: Text ) -> Nullable<Text>);
// sql_function! {
//     #[sql_name = "ARRAY_AGG"]
//     fn pg_array<T>(expr: T) -> Array<T>;

// }
