use crate::{
    app::{
        command::create_short_url::{CreateShortUrl, CreateShortUrlRepository},
        query::get_full_url::{GetFullQuery, GetFullUrlRepository},
    },
    id_provider::IdProvider,
};

pub struct Container<I, R, Q>
where
    I: IdProvider,
    R: CreateShortUrlRepository,
    Q: GetFullUrlRepository,
{
    pub shorten_command: CreateShortUrl<I, R>,
    pub get_full_url_query: GetFullQuery<Q>,
}

impl<I, R, Q> Container<I, R, Q>
where
    I: IdProvider,
    R: CreateShortUrlRepository,
    Q: GetFullUrlRepository,
{
    pub fn new(id_provider: I, repository: R, querier: Q) -> Self {
        let shorten_command = CreateShortUrl::new(id_provider, repository);
        let get_full_url_query = GetFullQuery::new(querier);

        Container {
            shorten_command,
            get_full_url_query,
        }
    }
}