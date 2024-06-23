import { echo_data } from "./echo_data";
import { load_data } from "./load_data";

async function main() {
  const data = await load_data();
  const echo = await echo_data(data);
  const version_span = document.querySelector("#ver") as HTMLSpanElement;
  if (!version_span) {
    throw Error("no version element");
  }
  version_span.innerText = echo.release;
}
main();
