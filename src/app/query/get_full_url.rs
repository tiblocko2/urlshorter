pub trait GetFullUrlRepository {
    fn get(&self, id: &str) -> Result<String, String>;
}

pub struct GetFullQuery<R> 
where 
    R: GetFullUrlRepository,
{
    repo: R,
}

impl<R> GetFullQuery<R>
where
    R: GetFullUrlRepository,
{
    pub fn new(repo: R) -> Self {
        Self {repo}
    }

    pub fn execute(&self, id: &str) -> Result<String, String> {
        self.repo.get(id)
    }

}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use dashmap::DashMap;

    use crate::adapters::inmemory::InMemoryRepository;

    use super::*;

    #[tokio::test]
    async fn get_full_url() {
        //Given
        struct FakeRapository;
        impl GetFullUrlRepository for FakeRapository {
            fn get(&self, _id: &str) -> Result<String, String> {
                Ok("https://www.google.com".to_owned())
            }
        }

        let repo = FakeRapository;
        let query = GetFullQuery::new(repo);

        //When
        let result = query.execute("123");

        //Then
        assert_eq!(result, Ok("https://www.google.com".to_owned()));
    }

    #[tokio::test]
    async fn get_from_inmemory_repo() {
        //Given
        let store = Arc::new(DashMap::new());
        store.insert("123".to_owned(), "http://www.google.com".to_owned());
        let repo = InMemoryRepository::new(store);
        let query = GetFullQuery::new(repo);

        //When
        let result = query.execute("123");

        //Then
        assert_eq!(result, Ok("http://www.google.com".to_owned()));
    }
}