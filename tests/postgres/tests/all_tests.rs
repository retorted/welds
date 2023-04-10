use postgres_test::models::enums::Color;
use postgres_test::models::order::Order;
use postgres_test::models::other::Other;
use postgres_test::models::product::Product;

#[derive(Default, Debug, Clone, sqlx::FromRow)]
pub struct Count {
    pub count: i32,
}

#[test]
fn should_be_able_to_read_all_products() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let q = Product::all();
        eprintln!("SQL: {}", q.to_sql());
        let all = q.run(conn).await.unwrap();
        assert_eq!(all.len(), 6, "Unexpected number of rows returned");
    })
}

#[test]
fn should_be_able_to_filter_on_equal() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let q = Product::where_col(|x| x.price_1.equal(1.10));
        eprintln!("SQL: {}", q.to_sql());
        let just_horse = q.run(conn).await.unwrap();
        assert_eq!(
            just_horse.len(),
            1,
            "Expected to only find the horse in the test data"
        );
    })
}

#[test]
fn should_be_able_to_filter_on_lt() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let q = Product::where_col(|x| x.price_1.lt(3.00));
        eprintln!("SQL: {}", q.to_sql());
        let data = q.run(conn).await.unwrap();
        assert_eq!(data.len(), 2, "Expected horse and dog",);
    })
}

#[test]
fn should_be_able_to_filter_on_lte() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let q = Product::where_col(|x| x.price_1.lte(2.10));
        eprintln!("SQL: {}", q.to_sql());
        let data = q.run(conn).await.unwrap();
        assert_eq!(data.len(), 2, "Expected horse and dog",);
    })
}

#[test]
fn should_be_able_to_filter_with_nulls() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        // is null
        let q1 = Product::where_col(|x| x.price_1.equal(None));
        eprintln!("SQL_1: {}", q1.to_sql());
        let data1 = q1.run(conn).await.unwrap();
        assert_eq!(data1.len(), 0, "Expected All",);
        // is not null
        let q1 = Product::where_col(|x| x.price_1.not_equal(None));
        eprintln!("SQL_2: {}", q1.to_sql());
        let data1 = q1.run(conn).await.unwrap();
        assert_eq!(data1.len(), 6, "Expected All",);
    })
}

#[test]
fn should_be_able_to_count_in_sql() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let q = Product::where_col(|x| x.price_1.lte(2.10));
        eprintln!("SQL: {}", q.to_sql());
        let count = q.count(conn).await.unwrap();
        assert_eq!(count, 2,);
    })
}

#[test]
fn should_be_able_to_limit_results_in_sql() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let q = Product::all().limit(2).offset(1);
        eprintln!("SQL: {}", q.to_sql());
        let count = q.run(conn).await.unwrap().len();
        assert_eq!(count, 2);
    })
}

#[test]
fn should_be_able_to_order_by_id() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let q = Product::all().order_by_asc(|x| x.product_id);
        eprintln!("SQL: {}", q.to_sql());
        let all = q.run(conn).await.unwrap();
        let ids: Vec<i32> = all.iter().map(|x| x.product_id).collect();
        let mut ids_sorted = ids.clone();
        ids_sorted.sort();
        assert_eq!(ids, ids_sorted);
    })
}

#[test]
fn should_be_able_to_update_a_product() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let mut trans = conn.begin().await.unwrap();

        let q = Product::all().limit(1);
        let mut found: Vec<_> = q.run(&mut trans).await.unwrap();
        let mut p1 = found.pop().unwrap();
        p1.name = "Test1".to_owned();
        p1.save(&mut trans).await.unwrap();

        let q = Product::where_col(|x| x.product_id.equal(p1.product_id));
        let mut found: Vec<_> = q.run(&mut trans).await.unwrap();
        let p2 = found.pop().unwrap();
        assert_eq!(p2.name, "Test1");

        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_create_a_new_product() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let mut trans = conn.begin().await.unwrap();

        let mut p1 = Product::new();
        p1.name = "newyNewFace".to_owned();
        p1.description = Some("YES!".to_owned());
        // Note: creation will set the PK for the model.
        p1.save(&mut trans).await.unwrap();

        let q = Product::where_col(|x| x.product_id.equal(p1.product_id));
        let mut found: Vec<_> = q.run(&mut trans).await.unwrap();
        let p2 = found.pop().unwrap();
        assert_eq!(p2.name, "newyNewFace");

        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_drop_down_to_sqlx() {
    async_std::task::block_on(async {
        //setup
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        // Build some args to send to the database
        use sqlx::{postgres::PgArguments, Arguments};
        let mut args: PgArguments = Default::default();
        args.add(1_i32);
        // Go run a query from the database.
        let sql = "SELECT * FROM products where product_id > $1";
        let all_but_one: Vec<Product> = sqlx::query_as_with(sql, args)
            .fetch_all(conn)
            .await
            .unwrap();
        assert_eq!(all_but_one.len(), 5);
    })
}

#[test]
fn should_be_able_to_run_raw_sql() {
    async_std::task::block_on(async {
        //setup
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();

        // Go run a query from the database.
        let sql = "SELECT * FROM products";

        let args = sqlx::postgres::PgArguments::default();
        let all = Product::from_raw_sql(sql, args, conn).await.unwrap();

        assert_eq!(all.len(), 6);
    })
}

#[test]
fn should_be_able_to_crud_orders() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let mut trans = conn.begin().await.unwrap();
        let mut o = Order::new();
        o.quantity = Some(9942);
        o.save(&mut trans).await.unwrap();
        let created = Order::all()
            .limit(1)
            .order_by_desc(|x| x.id)
            .run(&mut trans)
            .await
            .unwrap();
        let created2 = Order::where_col(|x| x.quantity.gt(9941))
            .run(&mut trans)
            .await
            .unwrap();
        let created1 = &created[0];
        let created2 = &created2[0];
        assert_eq!(o.id, created1.id);
        assert_eq!(created1.quantity, Some(9942));
        assert_eq!(o.id, created2.id);
        assert_eq!(created2.quantity, Some(9942));
    })
}

#[test]
fn should_be_able_to_delete_a_product() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let mut trans = conn.begin().await.unwrap();

        let id = 6;
        let original_total = Product::all().count(&mut trans).await.unwrap();
        let mut product = Product::find_by_id(&mut trans, id).await.unwrap().unwrap();
        product.delete(&mut trans).await.unwrap();
        let new_total = Product::all().count(&mut trans).await.unwrap();

        assert_eq!(new_total, original_total - 1);
        trans.rollback().await.unwrap();
    })
}

#[test]
fn should_be_able_to_find_like() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        //build the queries
        let like = Product::where_col(|x| x.name.like("%Horse%"));
        let ilike = Product::where_col(|x| x.name.ilike("%Horse%"));
        eprintln!("SQL: {}", ilike.to_sql());
        let not_like = Product::where_col(|x| x.name.not_like("%Horse%"));
        let not_ilike = Product::where_col(|x| x.name.not_ilike("%Horse%"));
        //run the queries
        let like = like.run(conn).await.unwrap();
        let ilike = ilike.run(conn).await.unwrap();
        let not_like = not_like.run(conn).await.unwrap();
        let not_ilike = not_ilike.run(conn).await.unwrap();
        //validate data
        assert_eq!(like.len(), 0, "like");
        assert_eq!(ilike.len(), 1, "ilike");
        assert_eq!(not_like.len(), 6, "not like");
        assert_eq!(not_ilike.len(), 5, "not ilike");
    })
}

#[test]
fn should_be_able_to_filter_on_relations() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let orders = Product::where_col(|x| x.name.like("%horse%")).map_query(|p| p.order);
        let orders = orders.run(conn).await.unwrap();
        assert_eq!(2, orders.len());
    })
}

#[test]
fn should_be_able_to_filter_on_relations2() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let product_query = Order::all().map_query(|p| p.product);
        // Vec<_> would be simpler, but want to hard code to type for test.
        use welds::state::DbState;
        let products: Vec<DbState<Product>> = product_query.run(conn).await.unwrap();
        assert_eq!(2, products.len());
    })
}

#[test]
fn should_be_able_to_filter_with_relations() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let filter1 = Product::where_col(|x| x.product_id.equal(1));
        let mut order_query = Order::all();
        order_query = order_query.where_relation(|o| o.product, filter1);
        // Vec<_> would be simpler, but want to hard code to type for test.
        use welds::state::DbState;
        let orders: Vec<DbState<Order>> = order_query.run(conn).await.unwrap();
        assert_eq!(2, orders.len());
    })
}

#[test]
fn should_be_able_to_filter_with_relations2() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let filter1 = Order::where_col(|x| x.id.lte(3));
        let mut product_query = Product::all();
        product_query = product_query.where_relation(|p| p.order, filter1);
        // Vec<_> would be simpler, but want to hard code to type for test.
        use welds::state::DbState;
        let orders: Vec<DbState<Product>> = product_query.run(conn).await.unwrap();
        assert_eq!(2, orders.len());
    })
}

#[test]
fn should_be_able_to_scan_for_all_tables() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let tables = welds::detect::find_tables(&conn).await.unwrap();
        assert_eq!(3, tables.len());
    })
}

#[test]
fn should_be_able_to_save_load_obj_with_db_enum_type() {
    async_std::task::block_on(async {
        let conn = testlib::postgres::conn().await.unwrap();
        let pool: welds::database::Pool = conn.into();
        let conn = pool.as_postgres().unwrap();
        let mut trans = conn.begin().await.unwrap();

        let start_count = Other::all().count(&mut trans).await.unwrap();
        let mut tmp = Other::new();
        tmp.colour = Color::Blue;
        tmp.save(&mut trans).await.unwrap();

        let count = Other::all().count(&mut trans).await.unwrap();
        assert_eq!(start_count + 1, count);

        let loaded = Other::find_by_id(&mut trans, tmp.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(*tmp, *loaded);
    })
}
