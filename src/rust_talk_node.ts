enum StatusCode {
  OK = 200,
}

type Response = {
  statusCode: StatusCode;
  message: string;
};

const functionHandler = async (): Promise<Response> => {
  return new Promise((resolve) => {
    let message: Response = {
      message: "Hello Rust Talk from Node!",
      statusCode: StatusCode.OK,
    };

    resolve(message);
  });
};

export async function main() {
  return await functionHandler();
}
