/*
Objective: Create a full duplex connection between client and server.
 */
"use strict";
// Connect to server
const HOST = "127.0.0.1";
const PORT = 8080;

const server = new WebSocket("ws://" + HOST + ":" + PORT.toString());

server.addEventListener('open', function (event) {
    server.send('Discovered.');
});

server.addEventListener('message', function (event) {
    console.log('Message from server ', event.data);
});

server.addEventListener('close', function (event_) {
    console.log('Connection closed');
});

const button = document.getElementById('button')
button.addEventListener('click', function () {
    server.send('Discovered.')
})