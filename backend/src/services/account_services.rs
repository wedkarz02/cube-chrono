use std::sync::Arc;

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Uuid},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};

use crate::{error::AppError, models::account::Account, AppState};

use super::{get_collection, Collections};

pub async fn insert(state: &Arc<AppState>, account: Account) -> Result<InsertOneResult, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let result = accounts
        .insert_one(account)
        .await?;

    Ok(result)
}

pub async fn find_all(state: &Arc<AppState>) -> Result<Vec<Account>, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let result: Vec<Account> = accounts
        .find(doc! {})
        .await?
        .try_collect()
        .await?;

    Ok(result)
}

pub async fn find_by_id(state: &Arc<AppState>, id: Uuid) -> Result<Option<Account>, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let result = accounts
        .find_one(doc! { "_id": id })
        .await?;

    Ok(result)
}

pub async fn find_by_username(
    state: &Arc<AppState>,
    username: &str,
) -> Result<Option<Account>, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let result = accounts
        .find_one(doc! { "username": username })
        .await?;

    Ok(result)
}

pub async fn update(state: &Arc<AppState>, body: Account) -> Result<UpdateResult, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let result = accounts
        .replace_one(doc! { "_id": body.id }, body)
        .await?;

    Ok(result)
}

pub async fn delete_by_id(state: &Arc<AppState>, id: Uuid) -> Result<DeleteResult, AppError> {
    let accounts: Collection<Account> = get_collection(state, Collections::ACCOUNTS);
    let result = accounts
        .delete_one(doc! { "_id": id })
        .await?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{
        error::AppError,
        models::account::{Account, Role},
    };
    use async_trait::async_trait;
    use mockall::{mock, predicate::eq};
    use mongodb::bson::{doc, Bson, Uuid};

    #[derive(Debug, Clone)]
    pub struct MockInsertOneResult {
        #[allow(unused)]
        pub inserted_id: Bson,
    }

    impl From<mongodb::results::InsertOneResult> for MockInsertOneResult {
        fn from(result: mongodb::results::InsertOneResult) -> Self {
            MockInsertOneResult {
                inserted_id: result.inserted_id,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct MockUpdateResult {
        pub matched_count: u64,
        pub modified_count: u64,
    }

    impl From<mongodb::results::UpdateResult> for MockUpdateResult {
        fn from(result: mongodb::results::UpdateResult) -> Self {
            MockUpdateResult {
                matched_count: result.matched_count,
                modified_count: result.modified_count,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct MockDeleteResult {
        pub deleted_count: u64,
    }

    impl From<mongodb::results::DeleteResult> for MockDeleteResult {
        fn from(result: mongodb::results::DeleteResult) -> Self {
            MockDeleteResult {
                deleted_count: result.deleted_count,
            }
        }
    }

    #[async_trait]
    pub trait AccountRepository: Send + Sync {
        async fn insert(&self, account: Account) -> Result<MockInsertOneResult, AppError>;
        async fn find_all(&self) -> Result<Vec<Account>, AppError>;
        async fn find_by_id(&self, id: Uuid) -> Result<Option<Account>, AppError>;
        async fn find_by_username(&self, username: &str) -> Result<Option<Account>, AppError>;
        async fn update(&self, account: Account) -> Result<MockUpdateResult, AppError>;
        async fn delete_by_id(&self, id: Uuid) -> Result<MockDeleteResult, AppError>;
    }

    mock! {
        pub AccountRepo {}

        #[async_trait]
        impl AccountRepository for AccountRepo {
            async fn insert(&self, account: Account) -> Result<MockInsertOneResult, AppError>;
            async fn find_all(&self) -> Result<Vec<Account>, AppError>;
            async fn find_by_id(&self, id: Uuid) -> Result<Option<Account>, AppError>;
            async fn find_by_username(&self, username: &str) -> Result<Option<Account>, AppError>;
            async fn update(&self, account: Account) -> Result<MockUpdateResult, AppError>;
            async fn delete_by_id(&self, id: Uuid) -> Result<MockDeleteResult, AppError>;
        }
    }

    #[tokio::test]
    async fn test_insert() {
        let mut mock_repo = MockAccountRepo::new();
        let account = Account {
            id: Uuid::new(),
            username: "test_user".to_string(),
            hashed_password: "test_hash".to_string(),
            roles: vec![Role::User],
        };

        let insert_result = MockInsertOneResult {
            inserted_id: Bson::Document(doc! { "_id": account.id }),
        };

        mock_repo
            .expect_insert()
            .with(eq(account.clone()))
            .returning(move |_| Ok(insert_result.clone()));

        let result = mock_repo
            .insert(account)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_all() {
        let mut mock_repo = MockAccountRepo::new();
        let accounts = vec![
            Account {
                id: Uuid::new(),
                username: "user1".to_string(),
                hashed_password: "hash1".to_string(),
                roles: vec![Role::User],
            },
            Account {
                id: Uuid::new(),
                username: "user2".to_string(),
                hashed_password: "hash2".to_string(),
                roles: vec![Role::User],
            },
        ];

        mock_repo
            .expect_find_all()
            .returning(move || Ok(accounts.clone()));

        let result = mock_repo
            .find_all()
            .await
            .unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let mut mock_repo = MockAccountRepo::new();
        let account_id = Uuid::new();
        let account = Account {
            id: account_id,
            username: "test_user".to_string(),
            hashed_password: "test_hash".to_string(),
            roles: vec![Role::User],
        };

        mock_repo
            .expect_find_by_id()
            .with(eq(account_id))
            .returning(move |_| Ok(Some(account.clone())));

        let result = mock_repo
            .find_by_id(account_id)
            .await
            .unwrap();
        assert!(result.is_some());
        assert_eq!(
            result
                .unwrap()
                .username,
            "test_user"
        );
    }

    #[tokio::test]
    async fn test_find_by_username() {
        let mut mock_repo = MockAccountRepo::new();
        let username = "test_user";
        let account = Account {
            id: Uuid::new(),
            username: username.to_string(),
            hashed_password: "test_hash".to_string(),
            roles: vec![Role::User],
        };

        mock_repo
            .expect_find_by_username()
            .with(eq(username.to_string()))
            .returning(move |_| Ok(Some(account.clone())));

        let result = mock_repo
            .find_by_username(username)
            .await
            .unwrap();
        assert!(result.is_some());
        assert_eq!(
            result
                .unwrap()
                .username,
            "test_user"
        );
    }

    #[tokio::test]
    async fn test_update() {
        let mut mock_repo = MockAccountRepo::new();
        let account = Account {
            id: Uuid::new(),
            username: "updated_user".to_string(),
            hashed_password: "new_hash".to_string(),
            roles: vec![Role::Admin],
        };

        let update_result = MockUpdateResult {
            matched_count: 1,
            modified_count: 1,
        };

        mock_repo
            .expect_update()
            .with(eq(account.clone()))
            .returning(move |_| Ok(update_result.clone()));

        let result = mock_repo
            .update(account)
            .await
            .unwrap();
        assert_eq!(result.matched_count, 1);
        assert_eq!(result.modified_count, 1);
    }

    #[tokio::test]
    async fn test_delete_by_id() {
        let mut mock_repo = MockAccountRepo::new();
        let account_id = Uuid::new();

        let delete_result = MockDeleteResult { deleted_count: 1 };

        mock_repo
            .expect_delete_by_id()
            .with(eq(account_id))
            .returning(move |_| Ok(delete_result.clone()));

        let result = mock_repo
            .delete_by_id(account_id)
            .await
            .unwrap();
        assert_eq!(result.deleted_count, 1);
    }
}
