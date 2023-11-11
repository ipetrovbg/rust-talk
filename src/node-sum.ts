enum StatusCode {
  OK = 200,
}

type ResponseModel = {
  sum: number;
  message: string;
};

class Response {
  statusCode: StatusCode;
  body: string;

  constructor(response: ResponseModel, statusCode: StatusCode) {
    this.statusCode = statusCode;
    this.body = JSON.stringify(response);
  }
}

const functionHandler = async (): Promise<Response> => {
  let sum = 0;
  for (let i = 0; i < 100_000_000; i++) {
    sum += i;
  }

  let message: ResponseModel = {
    message: `The sum of 0..100,000,000 is ${sum}`,
    sum,
  };
  let response = new Response(message, StatusCode.OK);

  return response;
};

export async function main() {
  return await functionHandler();
}
