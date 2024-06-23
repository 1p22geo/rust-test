export async function echo_data(data: Data) {
  let data_res = await fetch("/api/data", {
    method: "POST",
    body: JSON.stringify(data),
  });
  let data_json = await data_res.json();

  return data_json as Data;
}
