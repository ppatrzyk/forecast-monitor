
const KSQL = "http://localhost:8088/query";

async function viz() {
    let query = JSON.stringify(
        {
            "ksql": "SELECT * FROM forecasts LIMIT 10;",
            "streamsProperties": {
                "ksql.streams.auto.offset.reset": "earliest"
            }
        }
    )
    opts = {
        method: "POST",
        headers: {
            "Accept": "application/vnd.ksql.v1+json",
            "Content-Type": "application/json",
        },
        body: query,
    }
    let response = await fetch(KSQL, opts);
    let data = await response.json();
    console.log(data);
}

document.addEventListener('DOMContentLoaded', (e) => {viz()});