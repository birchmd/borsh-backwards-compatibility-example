# Borsh Backwards Compatibility Example

In a blockchain project you must support the full history of transactions in all future versions of your software.
Breaking changes are never allowed; only backwards compatible ones.
The purpose of this exercise is to have you work through a real-world example of making such a change.

## Background: Serialization

In the NEAR ecosystem, it is common to use a serialization format called [Borsh](https://borsh.io/).
This is a binary data serialization format with relatively simple rules.
The simplicity is important because the output must be predictable and consistent to use it safely in computing things like block hashes.
However, the simplicity means the format is fairly rigid, which makes some changes difficult.

## The Task

In this exercise we will be adding a new field to a struct.

Consider the following type:

```rust
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Movie {
    pub title: String,
    pub genre: Genre,
}
```

As a new feature request, we must add the IMDB URL to the `Movie` type.
Unfortunately, we cannot simply add this field because then the serialization would change.
For example, if we change the definition to

```rust
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Movie {
    pub title: String,
    pub genre: Genre,
    pub imdb_url: String,
}
```

then our test fails

```
---- tests::test_deserialize_movie stdout ----
thread 'tests::test_deserialize_movie' panicked at 'called `Result::unwrap()` on an `Err` value: Custom { kind: InvalidInput, error: "Unexpected length of input" }', src/lib.rs:27:64
```

Even trying the next most obvious trick does not work:

```rust
#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Movie {
    pub title: String,
    pub genre: Genre,
    pub imdb_url: Option<String>,
}
```

leads to the same failure (this is because `Option::None` is still represented as a byte in Borsh, rather than nothing).
This path would be suboptimal regardless because it does not capture the fact that we want all future movies we add to the blockchain to include the IMDB link (i.e. we don't want `None` being given).

Your task is to come up with a way to change the definition of `Movie` to include the new field such that the `test_deserialize_movie` test still passes.

## Solution

The solution is to have custom deserialization logic which still knows the old format, and to make the change inside a version structure.

```rust
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
```

Using the `backwards_compatible_deserialize` function allows our test to still pass, and we have achieved our goal of adding the new field.
This solution has the additional benefit of making future changes easy because we now have the versioned structure, and in Borsh it is always safe to add a new `enum` variant at the end. For example, if we decided we wanted to include the Rotten Tomatoes rating in a future update then we could simply add a `Movie::V3` variant without making any other changes.

In general it is good practice to always wrap structs that will be used with Borsh in an enum from the beginning to avoid needing the fallback logic like we did in this example.
It is also good practice to not serialize data types from the application directly for this reason.
It is awkward to deal with an `enum` in main application logic (for example, in the `Movie` case getting the title requires writing a getter method to hid the `match` that must happen).
Instead, it is better to have a `Movie` struct that does not derive Borsh traits, and a `BorshableMovie` enum which does derive them.
Then there is conversion logic needed between `Movie` and `BorshableMovie`, however this only happens at the boundary of the application (receiving input or creating output).
This design allows evolving the internal representation independently from the public API, which simplifies maintaining the internal logic while preserving backwards compatibility.
