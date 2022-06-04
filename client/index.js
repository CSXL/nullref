'use strict';
const HOST = '127.0.0.1';
const PORT = 8080;
const CLIENT_ID = Math.floor(Math.random() * 1000);

const server = new WebSocket('ws://' + HOST + ':' + PORT.toString());

server.addEventListener('open', function () {
    console.log('Established a connection with: ' + server.url)
    server.send('Client ID: ' + CLIENT_ID);
});

server.addEventListener('message', function (event) {
    console.log('Message from server ', event.data);
});

server.addEventListener('close', function () {
    console.log('Connection closed');
});

const button = document.getElementById('button')
button.addEventListener('click', function () {
    server.send('Discovered.')
})