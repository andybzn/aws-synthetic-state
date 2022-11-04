// imports
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_synthetics as synthetics;
use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::Value;
//use std::panic;

// main
#[tokio::main]
async fn main() -> Result<(), Error> {

    // fire the canary check
    let handler = service_fn(check_canary);
    lambda_runtime::run(handler).await?;
    
    // return ok
    Ok(())

}

async fn check_canary (_event: LambdaEvent<Value>) -> Result<(), Error> {

    // lambda environment variables
    let region = std::env::var("canaryRegion").unwrap();
    let canary = std::env::var("canaryName").unwrap();

    // aws sdk client config
    let region_provider =
        RegionProviderChain::default_provider().or_else(synthetics::Region::new(region));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = synthetics::Client::new(&shared_config);

    // pull the last canary run
    let mut canary_names = Vec::new();
    canary_names.push(String::from(canary));
    let names: Option<Vec<String>> = Some(canary_names);

    let canary_runs = client
        .describe_canaries_last_run()
        .set_names(names)
        .send()
        .await?;

    // parse the canary run to get the raw status as a string
    let canary_run = canary_runs.canaries_last_run().unwrap();
    let run = &canary_run.to_vec()[0];
    let state = run
        .last_run()
        .unwrap()
        .status()
        .unwrap()
        .state()
        .unwrap()
        .as_str();

    // debug
    println!("Canary Status: {:?}", state);

    // this is a hacky bailout
    if state == "FAILED" {
        println!("Bailing out");
        panic!();
    };

    // return ok
    Ok(())

}