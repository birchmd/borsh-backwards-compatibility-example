use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Movie {
    pub title: String,
    pub genre: Genre,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum Genre {
    Comedy,
    Drama,
    Fantasy,
    Horror,
    Romance,
    ScienceFiction,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_movie() {
        let input = hex::decode("120000004261636b20546f205468652046757475726505").unwrap();
        let deserialized_movie = Movie::try_from_slice(&input).unwrap();
        let expected_movie = Movie {
            title: "Back To The Future".into(),
            genre: Genre::ScienceFiction,
        };
        assert_eq!(deserialized_movie, expected_movie,);
    }
}
