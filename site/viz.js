
async function viz() {
    let response = await fetch(url, opts);
    let data = await response.json();
    console.log(data);
}

document.addEventListener('DOMContentLoaded', (e) => {viz()});