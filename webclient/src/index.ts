import { Socket } from "net";

// const form = document.getElementById("form");
// const API_URL = 'http://127.0.0.1:10000';


let client = new Socket();

// client.connect({port: 10000; adress: "127.0.0.1" }), function(){
//     console.log('TCP connection established with the server.');
// }

client.connect(10000, "127.0.0.1"), function(){
    console.log('TCP connection established with the server.');
}

// form.addEventListener("submit", (event) =>{
//     event.preventDefault();

//     const formData = new FormData(form);
//     const content = formData.get("content");

//     console.log(content);

//     fetch(API_URL, {
//         method: "POST",
//         body: content
//     });
// })

// let ws = new WebSocket('ws://127.0.0.1:10000', {transports: ['websocket']});

// ws.on('connect', function () {
//     console.log('connected!');
//     ws.emit('greet', { message: 'Hello Mr.Server!' });
// });

// ws.onopen = function() {
//     console.log("Hallo");
// }