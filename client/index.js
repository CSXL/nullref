'use strict';
class Server extends WebSocket {
  constructor(host, port) {
    super('ws://' + host + ':' + port.toString());
    return this;
  }
  onOpen(fn) {
    this.addEventListener('open', fn);
  }
  onClose(fn) {
    this.addEventListener('close', fn);
  }
  onError(fn) {
    this.addEventListener('error', fn);
  }
  onMessage(fn) {
    this.addEventListener('message', fn);
  }
  send(message) {
    super.send(message);
  }
}

const HOST = '127.0.0.1';
const PORT = 8080;
const CLIENT_ID = Math.floor(Math.random() * 1000);

const server = new Server(HOST, PORT);

server.onOpen(function() {
  console.log('Established a connection with: ' + server.url);
  server.send('Client ID: ' + CLIENT_ID);
});

server.onClose(function() {
  console.log('Connection closed.');
});

server.onError(function(error) {
    console.log('An error occurred: ', error);
});

server.onMessage(function(event) {
  console.log('Message from server: ', event.data);
});


const button = document.getElementById('button');
button.addEventListener('click', function() {
  server.send('Discovered.');
});
