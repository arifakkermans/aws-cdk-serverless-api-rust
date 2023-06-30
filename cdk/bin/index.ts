import * as cdk from 'aws-cdk-lib';
import { SimpleBookStack } from '../lib/lambda-api-gateway-stack';

const app = new cdk.App();

new SimpleBookStack(app, 'SimpleBookStack');