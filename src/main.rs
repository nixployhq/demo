use topcoat::{
    Result,
    router::{Router, RouterBuilderDiscoverExt, page},
    view::{View, view},
};

#[tokio::main]
async fn main() -> Result<()> {
    topcoat::start(Router::builder().discover().build()).await?;
    Ok(())
}

#[page("/")]
async fn home() -> Result<impl View> {
    let revision = option_env!("DEMO_REVISION").unwrap_or("local-checkout");
    Ok(view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"Nixploy × Topcoat"</title>
                <style>(include_str!("style.css"))</style>
            </head>
            <body>
                <p class="status">"HTTP service is running"</p>
                <h1>"Hello from Topcoat and Nixploy"</h1>
                <p>"Built from Git and deployed by Nixploy."</p>
                <p>"Demo version: "<strong>"2"</strong></p>
                <p>"Deployed commit:"<br><code>(revision)</code></p>
                <p>"Edit this page, commit, and push. Refresh after the next poll and build to see your update."</p>
                <a href="https://github.com/nixployhq/demo">"View source on GitHub →"</a>
            </body>
        </html>
    })
}
