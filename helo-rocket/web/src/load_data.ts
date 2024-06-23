export async function load_data() {
  let res = await fetch("/api/data");
  let json = await res.json();

  return json as Data;
}
