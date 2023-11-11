import {
  DynamoDBClient,
  QueryCommand,
  QueryInput,
} from "@aws-sdk/client-dynamodb";
import { unmarshall } from "@aws-sdk/util-dynamodb";

type User = {
  userId: string;
  userRole: string;
  userName: string;
  userEmail: string;
};

enum StatusCode {
  OK = 200,
}

type ResponseModel = User[];

class Response {
  statusCode: StatusCode;
  body: string;

  constructor(response: ResponseModel, statusCode: StatusCode) {
    this.statusCode = statusCode;
    this.body = JSON.stringify(response);
  }
}

const client = new DynamoDBClient({
  region: "eu-central-1",
});

const functionHandler = async (): Promise<Response> => {
  const queryInput: QueryInput = {
    TableName: "Users",
    KeyConditionExpression: "userRole = :userRole",
    ExpressionAttributeNames: {
      userRole: "userRole",
    },
    ExpressionAttributeValues: {
      ":userRole": {
        S: "admin",
      },
    },
  };

  const queryResponse = await client.send(new QueryCommand(queryInput));

  const admins: ResponseModel = (queryResponse.Items ?? []).map(
    (item) => unmarshall(item) as User,
  );

  let response = new Response(admins, StatusCode.OK);

  return response;
};

export async function main() {
  return await functionHandler();
}
