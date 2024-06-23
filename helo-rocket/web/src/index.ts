async function main() {
  let res = await fetch("/api/data")
  let json = await res.json()

  let data_res = await fetch("/api/data", {
    "method": "POST",
    "body": JSON.stringify(json)
  })
  let data_json = await data_res.json()

  const version_span = document.querySelector("#ver") as HTMLSpanElement
  if (!version_span) {
    throw Error("no version element")
  }
  version_span.innerText = data_json.release
}
main()
