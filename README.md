# Simple Serverless API in Rust

This repository includes an example of how to use Rust on AWS Lambda with an API Gateway using Rest. The example uses the [AWS CDK](https://aws.amazon.com/cdk/) to deploy the infrastructure and the [cargo-lambda](https://www.cargo-lambda.info/) tool to compile the Rust code to a binary that can be deployed to AWS Lambda.

#### Supported Features

- [x] Local development with `cdk watch` and `cargo-lambda`.
- [x] The API supports List, Get, Create, and Delete operations.

#### TODO

- [ ] Use DDB conditions to check if an item exists before storing or deleting an item.
- [ ] Verify if the incoming ISBN in the request is valid.
- [ ] Store the request in seperate fields in DBB.

## Local Development

To run the example locally, you need to install the `cargo-lambda` tool. You can install it with:
```bash
cargo install cargo-lambda
```

The best part about `cdk watch` and the `cargo-lambda` tool is that it will automatically recompile your code when you make changes. This means that you can make changes to your code and immediately test them.

## Deployment

To deploy the example to AWS, you need to install the AWS CDK. You can install it with:
- `npm install -g aws-cdk`.

After that, you can deploy the example with `cdk deploy`. This will deploy the Lambdas and create an API Gateway endpoint for you. The url for the API Gateway endpoint is on the output of the `cdk deploy` command.

```bash
cd cdk
cdk deploy
```

## Usage

You can use the following commands to test the API Gateway endpoint:

PUT request to create a book
```bash
curl --request PUT \
  --url https://{API_GW_URL}/prod/books/978-0-545-01022-2 \
  --header 'Content-Type: application/json' \
  --data '{"isbn":{"Isbn13":"978-0-545-01022-2"},"title":"test","authors":["John Doe"],"languages":["Dutch"],"countries":["NL"],"number_of_pages":124,"release_date":"10-01-1994"}'
```

GET request to retrieve a book
```bash
curl --request GET \
  --url https://{API_GW_URL}/prod/books/978-0-545-01022-2 \
  --header 'Content-Type: application/json'
```
