use server::HttpServer;

mod server; 
mod constants;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    HttpServer.run("127.0.0.1", 4221)?;
    Ok(())
}
