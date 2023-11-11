enum StatusCode {
  OK = 200,
}

type Response = {
  statusCode: StatusCode;
  message: string;
};

const functionHandler = async (): Promise<Response> => {
  let message: Response = {
    message: "Hello Rust Talk from Node!",
    statusCode: StatusCode.OK,
  };

  return message;
};

export async function main() {
  return await functionHandler();
}
