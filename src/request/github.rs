use serde::Deserialize;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: String,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub login: String,
}

#[derive(Debug, Deserialize)]
pub struct PullRequest {
    pub author: Option<Author>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct PullRequestNodes {
    pub page_info: PageInfo,
    pub nodes: Vec<PullRequest>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize)]
pub struct PullRequests {
    pub pull_requests: PullRequestNodes,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub repository: PullRequests,
}

#[derive(Debug, Deserialize)]
pub struct PullRequestsData {
    pub data: Repository,
}

pub struct GetPrArgs {
    pub name: String,
    pub owner: String,
    pub after: Option<String>,
}

pub fn query(config: GetPrArgs) -> String {
    format!(
        "query PullRequests {{
  repository(name: \"{name}\", owner: \"{owner}\") {{
    pullRequests(states: MERGED, first: 100{after}) {{
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
        name = config.name,
        owner = config.owner,
        after = {
            match config.after {
                Some(page) => format!(", after: \"{}\"", page),
                None => "".to_string(),
            }
        }
    )
    .replace(&['\n', '\t'][..], " ")
}
