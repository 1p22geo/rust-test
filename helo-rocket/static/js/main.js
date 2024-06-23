async function main() {
  let res = await fetch("/api/data")
  let json = await res.json()

  let data_res = await fetch("/api/data", {
    "method": "POST",
    "body": JSON.stringify(json)
  })
  let data_json = await data_res.json()

  document.querySelector("#ver").innerText = data_json.release
}
main()
