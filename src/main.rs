use std::sync::Arc;

use dashmap::DashMap;

use crate::{adapters::inmemory::InMemoryRepository, ports::httpapi::Server};

pub mod app;
pub mod id_provider;
pub mod adapters;
pub mod ports;
pub mod di;
pub mod error;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let store = Arc::new(DashMap::new());
    let repo = InMemoryRepository::new(store.clone());
    let querier = InMemoryRepository::new(store.clone());

    let idp = id_provider::NanoID;
    let container = Arc::new(di::Container::new(idp, repo, querier));

    let server = Server::new(3001, container);

    server.run().await;
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use dashmap::DashMap;

    use crate::adapters::inmemory::InMemoryRepository;

    #[tokio::test]
    async fn create_and_get_short_url() {
        //Given
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store.clone());
        let create_command = crate::app::command::create_short_url::CreateShortUrl::new(
            crate::id_provider::FakeIdProvider::new("123".to_owned()),
            repo.clone(),
        );

        let get_query = crate::app::query::get_full_url::GetFullQuery::new(repo);

        //When
        let result = create_command
            .execute("http://www.google.com".to_owned())
            .await;
        let result2 = get_query.execute(&result.unwrap()).unwrap();

        //Then
        assert_eq!(result2, "http://www.google.com".to_owned());
    }
}