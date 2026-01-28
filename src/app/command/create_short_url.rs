

pub struct CreateShortUrl;

impl CreateShortUrl {
    pub async fn execute(&self, full_url: String) -> Result<String, String>{
        Ok("1".to_owned())
    }
}

#[cfg(test)]
mod tests{

    use super::*;

    #[tokio::test]
    async fn get_short_url() {
        //Given
        let command = CreateShortUrl;

        //When
        let result = command.execute("https://www.google.com".to_owned()).await;

        //Then
        assert_ne!(result, Ok("".to_owned()));
    }
    #[tokio::test]
        async fn get_two_different_short_url() {
        //Given
        let command = CreateShortUrl;

        //When
        let result = command.execute("https://www.google.com".to_owned()).await;
        let result2 = command.execute("https://www.google.com".to_owned()).await;

        //Then
        assert_ne!(result, result2);
    }
}
