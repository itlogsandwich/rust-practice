use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()>
{
    let hc = httpc_test::new_client("http://localhost:3000")?;
    
    hc.do_get("/").await?.print().await?;

    //CREATE ACC IS NOT WORKING PROPERLY RN. DUE TO CODE CHANGES. I'VE MADE IT A LOT MORE HTML
    //INTERACTIVE.
    let req_create_acc = hc.do_post(
        "/create",
        json!({
            "owner": "demo",
            "pin": "12345678",
        })
    ).await?;

    req_create_acc.print().await?;


    let json_data = req_create_acc.json_body()?;

    let id = json_data["acc_num"].as_str().expect("There should be an acc_num if successful");

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
