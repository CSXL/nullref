'use strict';

/**
 * A wrapper for the WebSocket class detailed by [Mozilla's Web API documentation]{@link https://developer.mozilla.org/en-US/docs/Web/API/WebSocket}.
 * @class Connection
 * @extends WebSocket
 */
class Connection extends WebSocket {
  /**
   * Create a connection with a Websocket server.
   * @constructor
   * @param {string} host - The hostname to connect to
   * (e.g. 'localhost', '127.0.0.1', or 'example.com')
   * @param {number} port - The port to connect to
   * (e.g. 8080, 80, or 443)
   * @return {Connection}
   */
  constructor(host, port) {
    super(`ws://${host}:${port}`);
    return this;
  }

  /**
   * Add an event listener for the 'open' event.
   * @param {function} fn
   * @fires Connection#open
   * @see {@link https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onopen}
  */
  onOpen(fn) {
    this.addEventListener('open', fn);
  }

  /**
   * Add an event listener for the 'close' event.
   * @param {function} fn
   * @fires Connection#close
   * @see {@link https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onclose}
  */
  onClose(fn) {
    this.addEventListener('close', fn);
  }

  /**
   * Add an event listener for the 'error' event.
   * @param {function} fn
   * @fires Connection#error
   * @see {@link https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onerror}
  */
  onError(fn) {
    this.addEventListener('error', fn);
  }

  /**
   * Add an event listener for the 'message' event.
   * @param {function} fn
   * @fires Connection#message
   * @see {@link https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onmessage}
  */
  onMessage(fn) {
    this.addEventListener('message', fn);
  }

  /**
   * Send a message to the server.
   * @param {string} message
   * @see {@link https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send}
  */
  send(message) {
    super.send(message);
  }
}

module.exports = Connection;
