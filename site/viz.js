
const POSTGREST = "http://localhost:3000"

function format_date(date, offset) {
    let new_date = structuredClone(date);
    new_date.setHours(date.getHours() + offset);
    let formatted = new_date.toISOString();
    formatted = formatted.replace(/\.\d\d\dZ/, "Z")
    return formatted
}

async function fetch_data() {
    let date = new Date();
    date.setMinutes(0);
    date.setSeconds(0);
    let times = [...Array(24).keys()].map(offset => format_date(date, offset));
    // TODO loop for all later
    let current_date = times[0];
    let url = `${POSTGREST}/forecasts?weather_time=in.(${times[0]})`;
    let response = await fetch(url, {});
    let data = await response.json();
    viz_data(data)
}

function get_trace(data, source) {
    let filtered = data.filter(el => el.source == source);
    filtered = filtered.sort((a, b) => a.forecast_time > b.forecast_time ? 1 : -1)
    let trace = {
        x: filtered.map(el => el.forecast_time),
        y: filtered.map(el => el.temperature),
        type: "scatter",
    }
    return trace
}

function viz_data(data) {
    console.log(data);
    let weatherapi_trace = get_trace(data, "weatherapi");
    console.log(weatherapi_trace);
    var layout = {
        title: "TODO current hour",
    };
    Plotly.newPlot("viz", [weatherapi_trace, ], layout);
}

document.addEventListener('DOMContentLoaded', (e) => {fetch_data()});
