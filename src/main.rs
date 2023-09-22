use std::error::Error;

use nds::core::application::Application;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let app = Application::new()?;
    app.run().await?;

    Ok(())
}
