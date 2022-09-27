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
