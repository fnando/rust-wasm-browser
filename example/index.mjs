import init, { greet } from "../pkg/hello_js";

async function main() {
  await init();

  const output = document.querySelector("#output");

  document.querySelector("button").onclick = () => {
    const name = document.querySelector("input").value.trim() || "Stranger";
    output.textContent = greet(name);
  };
}

main();
