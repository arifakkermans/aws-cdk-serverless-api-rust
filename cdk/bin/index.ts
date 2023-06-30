import * as cdk from 'aws-cdk-lib';
import { SimpleBookStack } from '../lib/lambda-api-gateway-stack';

const envEU  = { account: '658544069824', region: 'eu-west-1' };
const app = new cdk.App();

new SimpleBookStack(app, 'SimpleBookStack', { env: envEU });