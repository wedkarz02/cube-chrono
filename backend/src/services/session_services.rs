use std::sync::Arc;

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Uuid},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};

use crate::{
    error::AppError,
    models::session::{Session, Time},
    AppState,
};

use super::{get_collection, Collections};

pub async fn find_all_by_account_id(
    state: &Arc<AppState>,
    id: Uuid,
) -> Result<Vec<Session>, AppError> {
    let sessions: Collection<Session> = get_collection(state, Collections::SESSIONS);
    let result = sessions
        .find(doc! { "account_id": id })
        .await?
        .try_collect()
        .await?;

    Ok(result)
}

pub async fn find_by_id_and_account_id(
    state: &Arc<AppState>,
    account_id: Uuid,
    id: Uuid,
) -> Result<Option<Session>, AppError> {
    let sessions: Collection<Session> = get_collection(state, Collections::SESSIONS);
    let result = sessions
        .find_one(doc! { "_id": id, "account_id": account_id })
        .await?;

    Ok(result)
}

pub async fn create(state: &Arc<AppState>, session: Session) -> Result<InsertOneResult, AppError> {
    let sessions: Collection<Session> = get_collection(state, Collections::SESSIONS);
    let result = sessions
        .insert_one(session)
        .await?;

    Ok(result)
}

pub async fn update_by_id_and_account_id(
    state: &Arc<AppState>,
    account_id: Uuid,
    id: Uuid,
    body: Session,
) -> Result<UpdateResult, AppError> {
    let sessions: Collection<Session> = get_collection(state, Collections::SESSIONS);
    let result = sessions
        .replace_one(doc! { "_id": id, "account_id": account_id }, body)
        .await?;

    Ok(result)
}

pub async fn insert_time(
    state: &Arc<AppState>,
    account_id: Uuid,
    id: Uuid,
    time: Time,
) -> Result<UpdateResult, AppError> {
    let mut session = match find_by_id_and_account_id(state, account_id, id).await? {
        None => return Err(AppError::NotFound),
        Some(s) => s,
    };

    session
        .times
        .push(time);

    update_by_id_and_account_id(state, account_id, id, session).await
}

pub async fn delete_by_id_and_account_id(
    state: &Arc<AppState>,
    account_id: Uuid,
    id: Uuid,
) -> Result<DeleteResult, AppError> {
    let sessions: Collection<Session> = get_collection(state, Collections::SESSIONS);
    let result = sessions
        .delete_one(doc! { "_id": id, "account_id": account_id })
        .await?;

    Ok(result)
}

pub async fn delete_all_by_account_id(
    state: &Arc<AppState>,
    account_id: Uuid,
) -> Result<DeleteResult, AppError> {
    let sessions: Collection<Session> = get_collection(state, Collections::SESSIONS);
    let result = sessions
        .delete_many(doc! { "account_id": account_id })
        .await?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{
        error::AppError,
        models::session::{Session, Time},
        services::scramble_services,
    };
    use async_trait::async_trait;
    use mockall::{mock, predicate::eq};
    use mongodb::bson::{self, doc, Bson, Uuid};

    #[derive(Debug, Clone)]
    pub struct MockInsertOneResult {
        #[allow(unused)]
        pub inserted_id: bson::Bson,
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
    pub trait SessionRepository: Send + Sync {
        async fn find_all_by_account_id(&self, account_id: Uuid) -> Result<Vec<Session>, AppError>;
        async fn find_by_id_and_account_id(
            &self,
            account_id: Uuid,
            id: Uuid,
        ) -> Result<Option<Session>, AppError>;
        async fn create(&self, session: Session) -> Result<MockInsertOneResult, AppError>;
        async fn update_by_id_and_account_id(
            &self,
            account_id: Uuid,
            id: Uuid,
            session: Session,
        ) -> Result<MockUpdateResult, AppError>;
        async fn insert_time(
            &self,
            account_id: Uuid,
            id: Uuid,
            time: Time,
        ) -> Result<MockUpdateResult, AppError>;
        async fn delete_by_id_and_account_id(
            &self,
            account_id: Uuid,
            id: Uuid,
        ) -> Result<MockDeleteResult, AppError>;
        async fn delete_all_by_account_id(
            &self,
            account_id: Uuid,
        ) -> Result<MockDeleteResult, AppError>;
    }

    mock! {
        pub SessionRepo {}

        #[async_trait]
        impl SessionRepository for SessionRepo {
            async fn find_all_by_account_id(&self, account_id: Uuid) -> Result<Vec<Session>, AppError>;
            async fn find_by_id_and_account_id(&self, account_id: Uuid, id: Uuid) -> Result<Option<Session>, AppError>;
            async fn create(&self, session: Session) -> Result<MockInsertOneResult, AppError>;
            async fn update_by_id_and_account_id(&self, account_id: Uuid, id: Uuid, session: Session) -> Result<MockUpdateResult, AppError>;
            async fn insert_time(&self, account_id: Uuid, id: Uuid, time: Time) -> Result<MockUpdateResult, AppError>;
            async fn delete_by_id_and_account_id(&self, account_id: Uuid, id: Uuid) -> Result<MockDeleteResult, AppError>;
            async fn delete_all_by_account_id(&self, account_id: Uuid) -> Result<MockDeleteResult, AppError>;
        }
    }

    fn create_mock_session(account_id: Uuid) -> Session {
        Session {
            id: Uuid::new(),
            account_id,
            name: "Test Session".to_string(),
            times: vec![],
        }
    }

    fn create_mock_time() -> Time {
        Time {
            millis: 1000,
            recorded_at: 1629209981,
            scramble: scramble_services::generate(crate::routes::scrambles::ScrambleKind::Three),
        }
    }

    // Tests

    #[tokio::test]
    async fn test_find_all_by_account_id() {
        let mut mock_repo = MockSessionRepo::new();
        let account_id = Uuid::new();
        let sessions = vec![
            create_mock_session(account_id),
            create_mock_session(account_id),
        ];

        mock_repo
            .expect_find_all_by_account_id()
            .with(eq(account_id))
            .returning(move |_| Ok(sessions.clone()));

        let result = mock_repo
            .find_all_by_account_id(account_id)
            .await
            .unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_find_by_id_and_account_id() {
        let mut mock_repo = MockSessionRepo::new();
        let account_id = Uuid::new();
        let session_id = Uuid::new();
        let session = create_mock_session(account_id);

        mock_repo
            .expect_find_by_id_and_account_id()
            .with(eq(account_id), eq(session_id))
            .returning(move |_, _| Ok(Some(session.clone())));

        let result = mock_repo
            .find_by_id_and_account_id(account_id, session_id)
            .await
            .unwrap();
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_create() {
        let mut mock_repo = MockSessionRepo::new();
        let session = create_mock_session(Uuid::new());
        let insert_result = MockInsertOneResult {
            inserted_id: Bson::Document(doc! { "_id": session.id }),
        };

        mock_repo
            .expect_create()
            .with(eq(session.clone()))
            .returning(move |_| Ok(insert_result.clone()));

        let result = mock_repo
            .create(session)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_by_id_and_account_id() {
        let mut mock_repo = MockSessionRepo::new();
        let account_id = Uuid::new();
        let session_id = Uuid::new();
        let session = create_mock_session(account_id);

        let update_result = MockUpdateResult {
            matched_count: 1,
            modified_count: 1,
        };

        mock_repo
            .expect_update_by_id_and_account_id()
            .with(eq(account_id), eq(session_id), eq(session.clone()))
            .returning(move |_, _, _| Ok(update_result.clone()));

        let result = mock_repo
            .update_by_id_and_account_id(account_id, session_id, session)
            .await
            .unwrap();
        assert_eq!(result.matched_count, 1);
        assert_eq!(result.modified_count, 1);
    }

    #[tokio::test]
    async fn test_insert_time() {
        let mut mock_repo = MockSessionRepo::new();
        let account_id = Uuid::new();
        let session_id = Uuid::new();
        let time = create_mock_time();

        mock_repo
            .expect_insert_time()
            .with(eq(account_id), eq(session_id), eq(time.clone()))
            .returning(move |_, _, _| {
                Ok(MockUpdateResult {
                    matched_count: 1,
                    modified_count: 1,
                })
            });

        let result = mock_repo
            .insert_time(account_id, session_id, time)
            .await
            .unwrap();
        assert_eq!(result.matched_count, 1);
        assert_eq!(result.modified_count, 1);
    }

    #[tokio::test]
    async fn test_delete_by_id_and_account_id() {
        let mut mock_repo = MockSessionRepo::new();
        let account_id = Uuid::new();
        let session_id = Uuid::new();
        let delete_result = MockDeleteResult { deleted_count: 1 };

        mock_repo
            .expect_delete_by_id_and_account_id()
            .with(eq(account_id), eq(session_id))
            .returning(move |_, _| Ok(delete_result.clone()));

        let result = mock_repo
            .delete_by_id_and_account_id(account_id, session_id)
            .await
            .unwrap();
        assert_eq!(result.deleted_count, 1);
    }

    #[tokio::test]
    async fn test_delete_all_by_account_id() {
        let mut mock_repo = MockSessionRepo::new();
        let account_id = Uuid::new();
        let delete_result = MockDeleteResult { deleted_count: 2 };

        mock_repo
            .expect_delete_all_by_account_id()
            .with(eq(account_id))
            .returning(move |_| Ok(delete_result.clone()));

        let result = mock_repo
            .delete_all_by_account_id(account_id)
            .await
            .unwrap();
        assert_eq!(result.deleted_count, 2);
    }
}
