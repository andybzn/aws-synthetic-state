# aws synthetic state

This package is used to check the last run status of an AWS CloudWatch Synthetic Canary, and will return an error if the last run was unsuccessful. The function is designed to be used in conjunction with API Gateway, and the Platform Status dashboard.

Platform Status tracks graphs via an API call, and in order to complete the graph, it must receive a specified response code. By returning `Ok` or `Error`, API Gateway will return either a successful code or a failure code.

## Usage

Provide the canary name and region as lambda environment variables:

| Variable Name | Type   |
|---------------|--------|
| canaryName    | String |
| canaryRegion  | String |  

## Output

| Canary State   | Output Response |
|----------------|-----------------|
| Run Successful | 200 (Ok)        |
| Run Failure    | 404 (Error)     |

## Roles & Credentials

- For local usage, this package will assume the role specified in `~/.aws/credentials`.
- When running as a lambda function, this package will assume the role granted to the function.

### AWS IAM Role

This lambda will require a role with the following Trust relationship, coupled with the policy detailed below

```json
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Principal": {
                "Service": "lambda.amazonaws.com"
            },
            "Action": "sts:AssumeRole"
        }
    ]
}
```

#### Policy

This function requires an access policy with `synthetics:Describe*`, in addition to the [AWSLambdaBasicExecutionRole](https://docs.aws.amazon.com/lambda/latest/dg/lambda-intro-execution-role.html)

```json
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Action": "synthetics:Describe*",
            "Resource": "*"
        }
    ]
}
```
