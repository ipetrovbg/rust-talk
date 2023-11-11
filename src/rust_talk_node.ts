enum StatusCode {
  OK = 200,
}

type ResponseModel = {
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
  let message: ResponseModel = { message: "Hello Rust Talk from Node!" };
  let response = new Response(message, StatusCode.OK);

  return response;
};

export async function main() {
  return await functionHandler();
}
