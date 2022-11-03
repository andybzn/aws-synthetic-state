# rust_aws_synthetics

This package is used to check the last run status of an AWS CloudWatch Synthetic Canary, and will return an error if the last run was unsuccessful. The function is designed to be used in conjunction with API Gateway, and the Platform Status dashboard.

Platform Status tracks graphs via an API call, and in order to complete the graph, it must receive a specified response code. By returning `Ok` or `Error`, API Gateway will return either a successful code or a failure code.
__In Theory__.

## Usage

Provide the canary name and region as lambda environment variables:

| Variable Name | Type   |
|---------------|--------|
| canaryName    | String |
| canaryRegion  | String |  

## Output

| Canary State   | Output |
|----------------|--------|
| Run Successful | Ok     |
| Run Failure    | Error  |

## Roles & Credentials

- For local usage, this package will assume the role specified in `~/.aws/credentials`.
- When running as a lambda function, this package will assume the role granted to the function.

## Useful info

```rust
    //some(
      // [canary last run]
        // last_run
          // some(
            // canary_run
              // status
                // some( STATE )
```
