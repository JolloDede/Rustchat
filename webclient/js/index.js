
const form = document.getElementById("form");
const API_URL = 'http://127.0.0.1:10000';

form.addEventListener("submit", (event) =>{
    event.preventDefault();

    const formData = new FormData(form);
    const content = formData.get("content");

    console.log(content);

    fetch(API_URL, {
        method: "POST",
        body: content
    });
})

// let ws = new WebSocket('ws://127.0.0.1:10000', {transports: ['websocket']});

// ws.on('connect', function () {
//     console.log('connected!');
//     ws.emit('greet', { message: 'Hello Mr.Server!' });
// });

// ws.onopen = function() {
//     console.log("Hallo");
// }