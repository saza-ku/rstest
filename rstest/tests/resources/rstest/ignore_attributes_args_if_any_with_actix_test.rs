use rstest::*;

use sqlx::SqlitePool;

fn sum(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

#[rstest]
#[actix_web::test]
#[sqlx::test]
async fn test_sum(#[values(1, 2)] c: i32, #[ignore] pool: SqlitePool) {
    let a = 1;
    let b = 2;
    assert_eq!(sum(a, b, c), a + b + c);
}
