# Tavily SDK

A Rust SDK for interacting with [Tavily](https://tavily.com), providing easy access to advanced search capabilities.

## Abstract

The Tavily SDK is a Rust library that simplifies interaction with the Tavily API. It offers a set of high-level functions for performing searches with various parameters, including search depth, topic, and result filtering. This SDK handles authentication, request construction, and response parsing, allowing developers to focus on utilizing Tavily's powerful search features in their applications.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
tavily-sdk = { git = "https://github.com/fdionisi/tavily-sdk" }
```

## Usage

Here's a basic example of how to use the Tavily SDK:

```rust
use tavily_sdk::{Tavily, TavilySearchParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Tavily client
    let tavily = Tavily::builder()
        .api_key("your_api_key_here".to_string())
        .build()?;

    // Create a search request
    let request = TavilySearchParams {
        query: "Rust programming".to_string(),
        search_depth: Some("basic".to_string()),
        ..Default::default()
    };

    // Perform the search
    let response = tavily.search(request).await?;

    // Process the results
    println!("Answer: {}", response.answer);
    for result in response.results {
        println!("Title: {}", result.title);
        println!("URL: {}", result.url);
        println!("Score: {}", result.score);
        println!("---");
    }

    Ok(())
}
```

For more detailed usage examples, including how to use advanced features like specifying search topics or including raw content, please refer to the documentation of each module.

## License

tavily-sdk is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.
