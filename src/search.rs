use crate::{error::TavilyError, Tavily};

/// Represents the parameters for a Tavily search request.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TavilySearchParams {
    /// The search query you want to execute with Tavily.
    pub query: String,

    /// The depth of the search. It can be "basic" or "advanced".
    /// Default is "basic" unless specified otherwise in a given method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_depth: Option<SearchDepth>,

    /// The category of the search. This will determine which of our agents will be used for the search.
    /// Currently: only "general" and "news" are supported. Default is "general".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<Topic>,

    /// The number of days back from the current date to include in the search results.
    /// This specifies the time frame of data to be retrieved.
    /// Please note that this feature is only available when using the "news" search topic.
    /// Default is 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<u32>,

    /// The maximum number of search results to return. Default is 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<u32>,

    /// Include a list of query-related images in the response. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_images: Option<bool>,

    /// Include a short answer to original query. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_answer: Option<bool>,

    /// Include the cleaned and parsed HTML content of each search result. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_raw_content: Option<bool>,

    /// A list of domains to specifically include in the search results.
    /// Default is [], which includes all domains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_domains: Option<Vec<String>>,

    /// A list of domains to specifically exclude from the search results.
    /// Default is [], which doesn't exclude any domains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_domains: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Topic {
    General,
    News,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchDepth {
    Basic,
    Advanced,
}

/// Represents the response from a Tavily search request.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TavilySearchResponse {
    /// The answer to your search query.
    pub answer: Option<String>,

    /// Your search query.
    pub query: String,

    /// Your search result response time.
    pub response_time: f64,

    /// A list of query related image urls.
    pub images: Option<Vec<String>>,

    /// A list of sorted search results ranked by relevancy.
    pub results: Vec<TavilySearchResult>,
}

/// Represents a single search result from a Tavily search.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TavilySearchResult {
    /// The title of the search result URL.
    pub title: String,

    /// The URL of the search result.
    pub url: String,

    /// The most query related content from the scraped URL.
    /// We use proprietary AI and algorithms to extract only the most relevant content
    /// from each URL, to optimize for context quality and size.
    pub content: String,

    /// The parsed and cleaned HTML of the site. For now includes parsed text only.
    pub raw_content: Option<String>,

    /// The relevance score of the search result.
    pub score: f64,

    /// The publication date of the source.
    /// This is only available if you are using "news" as your search topic.
    pub published_date: Option<String>,
}

impl Tavily {
    pub async fn search(
        &self,
        request: TavilySearchParams,
    ) -> Result<TavilySearchResponse, TavilyError> {
        self.post("/search", request).await
    }
}
