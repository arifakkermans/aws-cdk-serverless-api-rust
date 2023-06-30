import {
  aws_dynamodb as Dynamodb,
  aws_apigateway as aws_apigw,
  Stack,
  StackProps,
  RemovalPolicy,
  CfnOutput,
} from "aws-cdk-lib";
import { Construct } from "constructs";
import { RustFunction } from "cargo-lambda-cdk";
import path = require("path");

export class SimpleBookStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const dynamoTable = new Dynamodb.Table(this, "Books", {
      tableName: "BooksTable",
      partitionKey: {
        name: "isbn",
        type: Dynamodb.AttributeType.STRING,
      },
      readCapacity: 10,
      writeCapacity: 10,
      removalPolicy: RemovalPolicy.DESTROY, // NOT recommended for production code
    });

    const fnCreate = new RustFunction(this, "fnCreate", {
      manifestPath: path.join(__dirname, "../../Cargo.toml"),
      functionName: "createBookHandler",
      binaryName: "create",
      environment: { TABLE_NAME: dynamoTable.tableName },
    });

    const fnGet = new RustFunction(this, "fnGet", {
      manifestPath: path.join(__dirname, "../../Cargo.toml"),
      functionName: "getBookHandler",
      binaryName: "get",
      environment: { TABLE_NAME: dynamoTable.tableName },
    });

    const fnList = new RustFunction(this, "fnList", {
      manifestPath: path.join(__dirname, "../../Cargo.toml"),
      functionName: "listBookHandler",
      binaryName: "list",
      environment: { TABLE_NAME: dynamoTable.tableName },
    });

    const fnDelete = new RustFunction(this, "fnDelete", {
      manifestPath: path.join(__dirname, "../../Cargo.toml"),
      functionName: "deleteBookHandler",
      binaryName: "delete",
      environment: { TABLE_NAME: dynamoTable.tableName },
    });

    dynamoTable.grantReadWriteData(fnCreate);
    dynamoTable.grantReadWriteData(fnGet);
    dynamoTable.grantReadWriteData(fnList);
    dynamoTable.grantReadWriteData(fnDelete);

    const createBookIntegration = new aws_apigw.LambdaIntegration(fnCreate);
    const getBookIntegration = new aws_apigw.LambdaIntegration(fnGet);
    const listBookIntegration = new aws_apigw.LambdaIntegration(fnList);
    const deleteBookIntegration = new aws_apigw.LambdaIntegration(fnDelete);
    // Create an API Gateway resource for each of the CRUD operations
    const api = new aws_apigw.RestApi(this, "bookApi", {
      restApiName: "Book API Service",
    });
    const books = api.root.addResource("books");
    books.addMethod("GET", listBookIntegration);
    const book = books.addResource("{isbn}");

    const bookModel = new aws_apigw.Model(this, "model-validator", {
      restApi: api,
      contentType: "application/json",
      description: "To validate the request body",
      modelName: "bookModel",
      schema: {
        type: aws_apigw.JsonSchemaType.OBJECT,
        required: ["isbn", "title", "authors"],
        properties: {
          isbn: {
            type: aws_apigw.JsonSchemaType.OBJECT,
            properties: {
              Isbn13: { type: aws_apigw.JsonSchemaType.STRING },
            },
          },
          title: { type: aws_apigw.JsonSchemaType.STRING },
          authors: { type: aws_apigw.JsonSchemaType.ARRAY },
          number_of_pages: { type: aws_apigw.JsonSchemaType.INTEGER },
          countries: { type: aws_apigw.JsonSchemaType.ARRAY },
          release_date: { type: aws_apigw.JsonSchemaType.STRING },
        },
      },
    });

    book.addMethod("GET", getBookIntegration);
    book.addMethod("PUT", createBookIntegration, {
      requestValidator: new aws_apigw.RequestValidator(this, "body-validator", {
        restApi: api,
        requestValidatorName: "body-validator",
        validateRequestBody: true,
      }),
      requestModels: {
        "application/json": bookModel,
      },
    });
    book.addMethod("DELETE", deleteBookIntegration);

    // create cloudformation output for the API Gateway URL
    new CfnOutput(this, "API URL", {
      value: api.url!,
    });

    addCorsOptions(book);

    // Add CORS to restrict cross-origin HTTP requests
    function addCorsOptions(apiResource: aws_apigw.IResource) {
      apiResource.addMethod(
        "OPTIONS",
        new aws_apigw.MockIntegration({
          integrationResponses: [
            {
              statusCode: "200",
              responseParameters: {
                "method.response.header.Access-Control-Allow-Headers":
                  "'Content-Type,X-Amz-Date,Authorization,X-Api-Key,X-Amz-Security-Token,X-Amz-User-Agent'",
                "method.response.header.Access-Control-Allow-Origin": "'*'",
                "method.response.header.Access-Control-Allow-Credentials":
                  "'false'",
                "method.response.header.Access-Control-Allow-Methods":
                  "'OPTIONS,GET,PUT,POST,DELETE'",
              },
            },
          ],
          passthroughBehavior: aws_apigw.PassthroughBehavior.NEVER,
          requestTemplates: {
            "application/json": '{"statusCode": 200}',
          },
        }),
        {
          methodResponses: [
            {
              statusCode: "200",
              responseParameters: {
                "method.response.header.Access-Control-Allow-Headers": true,
                "method.response.header.Access-Control-Allow-Methods": true,
                "method.response.header.Access-Control-Allow-Credentials": true,
                "method.response.header.Access-Control-Allow-Origin": true,
              },
            },
          ],
        }
      );
    }
  }
}
