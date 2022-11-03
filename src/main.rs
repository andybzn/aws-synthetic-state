// imports
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_synthetics as synthetics;
use lambda_runtime::Error;
use std::panic;

// main
#[tokio::main]
async fn main() -> Result<(), Error> {
    // aws client config
    let region_provider =
        RegionProviderChain::default_provider().or_else(synthetics::Region::new("eu-west-2"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = synthetics::Client::new(&shared_config);

    // pull the last canary run
    let mut canary_names = Vec::new();
    canary_names.push(String::from("test-canary"));
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
        panic!();
    };

    // return ok
    Ok(())
}
