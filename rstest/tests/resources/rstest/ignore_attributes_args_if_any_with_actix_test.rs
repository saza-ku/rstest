use sqlx::SqlitePool;

fn sum(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

#[actix_web::test]
#[sqlx::test]
async fn test_sum(pool: SqlitePool) {
    let a = 1;
    let b = 2;
    let c = 3;
    assert_eq!(sum(a, b, c), a + b + c);
}
