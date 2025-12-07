cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
        use diesel::{PgConnection};
        use crate::schema::{attendees, events};
        use crate::models::Event;
        pub type PgPool = Pool<ConnectionManager<PgConnection>>;
        pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

        pub fn establish_pool() -> PgPool {
            let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let manager = ConnectionManager::<PgConnection>::new(database_url);
            Pool::builder()
                .build(manager)
                .expect("Failed to create pool.")
        }

        pub fn get_connection(pool: &PgPool) -> PgPooledConnection {
            pool.get().expect("Failed to get a connection from the pool.")
        }

        pub fn get_num_users(pool: &PgPool) -> i64 {
            use diesel::prelude::*;
            let conn = &mut get_connection(pool);
            attendees::table
                .count()
                .get_result(conn)
                .expect("Error counting users")
        }

        pub fn get_num_events(pool: &PgPool) -> i64 {
            use diesel::prelude::*;
            let conn = &mut get_connection(pool);
            events::table
                .count()
                .get_result(conn)
                .expect("Error counting events")
        }

        pub fn get_event_by_id(pool: &PgPool, event_id: i32) -> Option<Event> {
            use diesel::prelude::*;
            let conn = &mut get_connection(pool);
            events::table
                .find(event_id)
                .select(Event::as_select())
                .first(conn)
                .optional()
                .expect("Error loading event")
        }
    }
}
