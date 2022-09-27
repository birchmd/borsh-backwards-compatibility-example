use borsh::{maybestd::io, BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub enum Movie {
    V1(LegacyMovie),
    V2(MovieV2),
}

impl Movie {
    pub fn backwards_compatible_deserialize(input: &[u8]) -> io::Result<Self> {
        match Self::try_from_slice(input) {
            Ok(movie) => Ok(movie),
            // Fallback on legacy type if we cannot deserialize the new format
            Err(_) => LegacyMovie::try_from_slice(input).map(Self::V1),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct MovieV2 {
    pub title: String,
    pub genre: Genre,
    pub imdb_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct LegacyMovie {
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
        let deserialized_movie = Movie::backwards_compatible_deserialize(&input).unwrap();
        let expected_movie = Movie::V1(LegacyMovie {
            title: "Back To The Future".into(),
            genre: Genre::ScienceFiction,
        });
        assert_eq!(deserialized_movie, expected_movie,);
    }
}
