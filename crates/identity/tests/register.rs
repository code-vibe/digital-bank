use identity::{register, RegisterRequest};

#[sqlx::test(migrations = "../../migrations")]
async fn registers_customer(pool: sqlx::PgPool) {
    let input = RegisterRequest {
        email: "sam@example.com".to_string(),
        password: "secret123".to_string(),
    };

    let user = register(&pool, input).await.unwrap();

    assert_eq!(user.email, "sam@example.com");

    let role: String = sqlx::query_scalar(
        r#"
        SELECT r.code
        FROM roles r
        JOIN user_roles ur ON ur.role_id = r.id
        WHERE ur.user_id = $1
        "#,
    )
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(role, "customer");
    let password_hash: String = sqlx::query_scalar(
        r#"
    SELECT password_hash
    FROM users
    WHERE id = $1
    "#,
    )
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_ne!(password_hash, "secret123");
    assert!(password_hash.starts_with("$argon2id$"));
}