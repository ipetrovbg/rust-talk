type Response = {
  statusCode: number;
  message: string;
};

const function_handler = async () => {
  let message = {
    message: "Hello Rust Talk from Node!",
    statusCode: 200,
  } as Response;

  return message;
};

export async function main() {
  await function_handler();
}
