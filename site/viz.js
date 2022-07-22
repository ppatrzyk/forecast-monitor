
const POSTGREST = "http://localhost:3000"

function format_date(date, offset) {
    let new_date = structuredClone(date);
    new_date.setHours(date.getHours() + offset);
    let formatted = new_date.toISOString().replace(/\.\d\d\dZ/, "Z");
    return formatted
}

async function fetch_data() {
    let date = new Date();
    date.setMinutes(0);
    date.setSeconds(0);
    let times = [...Array(24).keys()].map(offset => format_date(date, offset));
    let url = `${POSTGREST}/forecasts?weather_time=in.(${times.join(",")})`;
    let response = await fetch(url, {});
    let data = await response.json();
    viz_data(data, times)
}

function get_trace(data, source, time) {
    let filtered = data.filter(el => el.source == source & el.weather_time == time);
    filtered = filtered.sort((a, b) => a.forecast_time > b.forecast_time ? 1 : -1)
    let trace = {
        x: filtered.map(el => el.forecast_time),
        y: filtered.map(el => el.temperature),
        name: source,
        type: "scatter",
    }
    return trace
}

function viz_data(data, times) {
    times.forEach(time => {
        let traces = [get_trace(data, "weatherapi", time), get_trace(data, "open-meteo.com", time), get_trace(data, "tomorrow.io", time), ]
        var layout = {title: time, displayModeBar: false};
        plot_div = document.createElement('div');
        plot_div.setAttribute("id", time);
        document.getElementsByTagName("main")[0].appendChild(plot_div)
        Plotly.newPlot(time, traces, layout);
    });
}

document.addEventListener('DOMContentLoaded', (e) => {fetch_data()});
