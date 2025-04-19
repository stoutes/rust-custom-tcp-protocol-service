mod collector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    let handle = tokio::spawn(collector::data_collector());
    
    // wait for data collector to finish
    handle.await??;
    // two question marks are for unwrapping the task result and the result from running the collector
    Ok(())
}
