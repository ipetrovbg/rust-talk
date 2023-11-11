enum StatusCode {
  OK = 200,
}

type ResponseModel = {
  message: string;
};

class Response {
  statusCode: StatusCode;
  body: string;

  constructor(payload: ResponseModel, statusCode: StatusCode) {
    this.statusCode = statusCode;
    this.body = JSON.stringify(payload);
  }
}

const functionHandler = async (): Promise<Response> => {
  return new Promise((resolve) => {
    let message: ResponseModel = { message: "Hello Rust Talk from Node!" };
    let response = new Response(message, StatusCode.OK);

    resolve(response);
  });
};

export async function main() {
  return await functionHandler();
}
