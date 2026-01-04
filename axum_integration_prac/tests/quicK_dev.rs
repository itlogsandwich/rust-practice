use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()>
{
    let hc = httpc_test::new_client("http://localhost:3000")?;
    let mut acc = String::new();
    
    hc.do_get("/").await?.print().await?;

    let req_create_acc = hc.do_post(
        "/create",
        json!({
            "owner": "demo",
            "pin": "12345678",
        }));

    req_create_acc.await?.print().await?;
    
    hc.do_get("/balance/1000").await?.print().await?;

    let req_deposit = hc.do_post(
        "/deposit",
        json!({
            "acc_num": "1000",
            "amount": 1000,
        }));

    req_deposit.await?.print().await?;

    hc.do_get("/balance/1000").await?.print().await?;

    let req_withdraw = hc.do_post(
        "/withdraw",
        json!({
            "acc_num": "1000",
            "amount": 500,
        }));

    req_withdraw.await?.print().await?;

    hc.do_get("/balance/1000").await?.print().await?;
    Ok(())
}
