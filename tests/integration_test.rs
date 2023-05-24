use chainalysis_client::*;
use test_log::test;

const API_KEY: &'static str = "REPLACE_ME";

fn get_client() -> Client {
    Client::new(API_KEY).unwrap()
}

#[test(tokio::test)]
#[ignore]
async fn get_sanctioned_address() {
    let client = get_client();
    let resp = client.get_address_sanctions("0x1da5821544e25c636c1417ba96ade4cf6d2f9b5a").await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
    let address_sanctions = resp.unwrap();
    assert!(!address_sanctions.identifications.is_empty());
}

#[test(tokio::test)]
#[ignore]
async fn get_unsanctioned_address() {
    let client = get_client();
    let resp = client.get_address_sanctions("0x205E10d3c4C87E26eB66B1B270b71b7708494dB9").await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
    let address_sanctions = resp.unwrap();
    assert!(address_sanctions.identifications.is_empty());
}

#[test(tokio::test)]
#[ignore]
async fn check_sanctioned_address() {
    let client = get_client();
    let resp = client.is_sanctioned("0x1da5821544e25c636c1417ba96ade4cf6d2f9b5a").await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
    let is_sanctioned = resp.unwrap();
    assert!(is_sanctioned);
}

#[test(tokio::test)]
#[ignore]
async fn check_unsanctioned_address() {
    let client = get_client();
    let resp = client.is_sanctioned("0x205E10d3c4C87E26eB66B1B270b71b7708494dB9").await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
    let is_sanctioned = resp.unwrap();
    assert!(!is_sanctioned);
}
