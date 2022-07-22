
const POSTGREST = "http://localhost:3000"

function format_date(date, offset) {
    let new_date = structuredClone(date);
    new_date.setHours(date.getHours() + offset);
    let formatted = new_date.toISOString();
    formatted = formatted.replace(/\.\d\d\dZ/, "Z")
    return formatted
}

async function viz() {
    let date = new Date();
    date.setMinutes(0);
    date.setSeconds(0);
    let times = [...Array(24).keys()].map(offset => format_date(date, offset));
    // TODO loop for all later
    let current_date = times[0];
    let url = `${POSTGREST}/forecasts?weather_time=in.(${times[0]})`;
    let response = await fetch(url, {});
    let data = await response.json();
    console.log(data);
}

document.addEventListener('DOMContentLoaded', (e) => {viz()});
