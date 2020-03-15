use serde::Deserialize;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
struct PageInfo {
    has_next_page: bool,
    end_cursor: String,
}

#[derive(Debug, Deserialize)]
struct Author {
    login: String,
}

#[derive(Debug, Deserialize)]
struct PullRequest {
    author: Author,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
struct PullRequestNodes {
    page_info: PageInfo,
    nodes: Vec<PullRequest>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
struct PullRequests {
    pull_requests: PullRequestNodes,
}

#[derive(Debug, Deserialize)]
struct Repository {
    repository: PullRequests,
}

#[derive(Debug, Deserialize)]
pub struct PullRequestsData {
    data: Repository,
}

pub struct GetPrArgs {
    pub name: String,
    pub owner: String,
}

pub fn query(config: GetPrArgs) -> String {
    format!(
        "query PullRequests {{
  repository(name: \"{}\", owner: \"{}\") {{
    pullRequests(states: MERGED, first: 1) {{
      pageInfo {{
	hasNextPage
	endCursor
      }}
      nodes {{
	author {{
	  login
	}}
      }}
    }}
  }}
}}",
        config.name, config.owner
    )
    .replace(&['\n', '\t'][..], " ")
}
