"""
Handles runtime
"""
import asyncio
import typing
import json
from json.decoder import JSONDecodeError
import base64
from binascii import Error as BinasciiError
import logging
import websockets

logging.basicConfig(
    format='%(asctime)s %(message)s',
    level=logging.INFO,
)

class Message(typing.TypedDict):
    LABEL: str
    TO: bytes
    FROM: bytes
    CONTENT: bytes

class Connection(typing.TypedDict):
    PUBLIC_KEY: bytes
    WEBSOCKET: websockets.server.WebSocketServerProtocol
    OUTBOUND_MESSAGES: asyncio.Queue
    INBOUND_MESSAGES: asyncio.Queue

connections = dict[bytes, Connection] # pylint: disable=invalid-name

async def dict_to_bytes(_dict: dict) -> bytes:
    obj = json.dumps(_dict)
    obj = obj.encode('ascii')
    obj = base64.b64encode(obj)
    return obj

async def dict_from_bytes(_bytes: bytes) -> dict:
    json_string = base64.b64decode(_bytes)
    obj = json.loads(json_string)
    return obj

async def producer_handler(connection: Connection):
    while True:
        message = await connection['OUTBOUND_MESSAGES'].get()
        try:
            assert isinstance(message, Message), 'Message must be in Message format'
        except AssertionError as error:
            logging.error(error)
            continue
        try: # Base64 encoding for reliable sending
            message = dict_to_bytes(message)
        except (TypeError, BinasciiError) as serialization_error:
            logging.error(serialization_error)
            await connection['OUTBOUND_MESSAGES'].task_done()
            continue


async def consumer_handler(connection: Connection):
    async for message in connection['WEBSOCKET']:
        try:
            assert isinstance(message, typing.ByteString), 'Message must be bytes'
        except AssertionError as error:
            logging.error(error)
            continue # 
        try: # Serializes and loads message
            message = dict_from_bytes(message)
        except (JSONDecodeError, BinasciiError) as error:
            logging.error(error)
            continue # TODO: Send error message to client
        send_to = message.get('TO')
        send_from = message.get('FROM')
        if send_to is None:
            continue # TODO: Send error message to client

async def ws_handler(websocket: websockets.server.WebSocketServerProtocol):
    pass

# Temporary for testing
async def main():
    async with websockets.server.serve(ws_handler, 'localhost', 8765):
        await asyncio.Future()

if __name__ == '__main__':
    asyncio.run(main())
