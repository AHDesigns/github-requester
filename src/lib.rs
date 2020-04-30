#[macro_use]
extern crate serde_json;

use std::collections::HashMap;
mod request;

// TODO: https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html#consume-a-paginated-restful-api
pub async fn get_pull_request_data() -> Result<(), Box<dyn std::error::Error>> {
    let mut prs_by_user = HashMap::new();

    let mut page: Option<String> = None;

    let mut has_next_page = true;

    while has_next_page {
        let res = request::make_request(&page).await?;
        println!("res: {:#?}", &res);

        for pull_request in res.data.repository.pull_requests.nodes {
            if let Some(author) = pull_request.author {
                let count = prs_by_user.entry(author.login).or_insert(0);
                *count += 1;
            }
        }

        if res.data.repository.pull_requests.page_info.has_next_page {
            page = Some(res.data.repository.pull_requests.page_info.end_cursor);
        } else {
            has_next_page = false;
        }
    }

    println!("{:#?}", prs_by_user);

    Ok(())
}
