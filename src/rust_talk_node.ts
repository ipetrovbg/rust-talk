export async function main() {
  return {
    statusCode: 200,
    body: JSON.stringify({
      message: "Hello Rust Talk from Node!",
    }),
  };
}
