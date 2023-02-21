const MessageTypes = {
  NEW_CONNECTION: 'NEW_CONNECTION',
  DISCONNECT: 'DISCONNECT',
  MESSAGE: 'MESSAGE',
};

/**
 * Represents a message sent between clients.
 * @class Message
 * @property {string} type - The type of message.
 */
class Message {
  /**
   * Create a new message.
   * @constructor
   * @param {MessageTypes} type - The type of message.
   * @param {string} from - The sender of the message.
   * @param {string} to - The recipient of the message.
   * @param {string} content - The content of the message.
  */
  constructor(type, from, to, content) {
    this.type = type;
    this.from = from;
    this.to = to;
    this.content = content;
  }

  /**
   * Checks if the message is valid.
   * @return {boolean}
  */
  isValid() {
    // Check if the type is valid
    if (!Object.values(MessageTypes).includes(this.type)) {
      return false;
    }
    return this.type && this.from && this.to && this.content;
  }

  /**
   * Create a new message from a JSON string.
   * @param {string} json - The JSON string to parse.
   * @return {Message}
   * @throws {Error} If the JSON string is invalid.
  */
  fromJSON(json) {
    try {
      const obj = JSON.parse(json);
      return new Message(obj.type, obj.from, obj.to, obj.content);
    } catch (e) {
      throw new Error('Invalid JSON string');
    }
  }

  /**
   * Convert the message to a JSON string.
   * @return {string}
  */
  toJSON() {
    return JSON.stringify(this);
  }
}

module.exports = {
  MessageTypes,
  Message,
};

