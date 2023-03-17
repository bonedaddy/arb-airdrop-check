use anyhow::{Result, anyhow, Context};
use tokio::io::{Lines, BufReader, AsyncBufReadExt};

const BASE_URL: &str = "https://arbitrum.foundation/eligibility?address=";

#[tokio::main]
async fn main() -> Result<()> {
    let addrs = check_addresses("addresses.txt").await?;
    println!("found {} eligible addresses", addrs.len());
    for addr in addrs.iter() {
        println!("eligible: {}", addr);
    }
    Ok(())
}


async fn check_addresses(file_name: &str) -> Result<Vec<String>> {
    let fh = tokio::fs::File::open(file_name).await.with_context(|| "failed to open addresses.txt")?;
    let mut line_rh = BufReader::new(fh).lines();
    let mut is_eligible = Vec::with_capacity(128);
    while let Ok(Some(line)) =  line_rh.next_line().await {
        let resp = reqwest::get(new_url(&line)).await.with_context(|| format!("failed to query address {}", line))?.text().await.with_context(|| format!("failed to query address {}", line))?;
        if resp.contains("eligible") {
            is_eligible.push(line.clone());
        }
    }
    Ok(is_eligible)   
}

fn new_url(addr: &str) -> String {
    format!("{}{}", BASE_URL, addr)
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new_url() {
        let url_got = new_url("foobar");
        assert_eq!(url_got, "https://arbitrum.foundation/eligibility?address=foobar");
    }
    #[tokio::test]
    async fn test_check_addresses() {
        let addr = std::env::var("ADDR").unwrap();
        tokio::fs::write("addresses.txt", format!("{}", addr)).await.unwrap();
        tokio::task::yield_now().await;
        let got_eligible = check_addresses("addresses.txt").await.unwrap();
        assert_eq!(got_eligible.len(), 1);
        tokio::fs::remove_file("addresses.txt").await.unwrap();
    }
}