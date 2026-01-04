use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> httpc_test::Result<()>
{
    let hc = httpc_test::new_client("http://localhost:3000")?;
    
    hc.do_get("/").await?.print().await?;

    let req_create_acc = hc.do_post(
        "/create",
        json!({
            "owner": "demo",
            "pin": "12345678",
        })
    ).await?;

    let status = req_create_acc.status();
    let id = req_create_acc.text_body()?;
 
    let x = format!("Status: {status}");
    println!("{x}\nID:{id}");

    hc.do_get(&format!("/balance/{id}")).await?.print().await?;

    let req_deposit = hc.do_post(
        "/deposit",
        json!({
            "acc_num": id,
            "amount": 1000,
        }));

    req_deposit.await?.print().await?;

    hc.do_get(&format!("/balance/{id}")).await?.print().await?;

    let req_withdraw = hc.do_post(
        "/withdraw",
        json!({
            "acc_num": id,
            "amount": 500,
        }));

    req_withdraw.await?.print().await?;

    hc.do_get(&format!("/balance/{id}")).await?.print().await?;
    Ok(())
}
