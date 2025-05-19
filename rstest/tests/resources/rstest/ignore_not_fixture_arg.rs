use rstest::*;

use sqlx::SqlitePool;

struct FixtureStruct {}

#[fixture]
fn my_fixture() -> FixtureStruct {
    FixtureStruct {}
}

#[rstest]
#[sqlx::test]
async fn test_db(my_fixture: FixtureStruct, #[ignore] pool: SqlitePool) {
    assert!(true);
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[rstest]
#[case(1, 2)]
#[case(2, 3)]
#[sqlx::test]
fn test_sum(#[case] a: i32, #[case] b: i32, #[ignore] pool: SqlitePool) {
    assert_eq!(sum(a, b), a + b);
}
