# Specification
## Requirements
### High-Level Features
- Identity
  - Public-Key Identification
  - Session-based aliases
- Client
  - Browser-based \
  **Lightweight**
  - Fit into one HTML file
  - Zero external dependencies \
  **Near Zero-Trust**
  - Store messages locally
  - Encrypt messages with intended receiver's public key 
  (Asymmetric Encryption)
  - Verifies public-keys
- Server
  - Cloud-based \
  **Scalable**
  - Containerized into docker image
  - Deployable to kubernetes
  - Use websockets to communicate with clients \
  **Near Zero-Trust**
  - Verifies public-keys
  - Route encrypted messages to clients based off public-key 
- Notes
  - Client IPs should ONLY be visible to the proxy server they are 
  connected to
  - Messages will ONLY be visible in plaintext to the intended recipient
  - Encryption and decryption should be handled by the clients
### Feature Notes
**Identity** \
**Identities in a zero-trust system should be `recyclable`, `verifiable` 
and `un-traceable`.** This zero-trust model allows for security of client 
data over insecure networks and malicious middlemen. Unlike an IP, a 
public-key cannot be attributed to a clients network. This lack of 
attribution shields the client against DDoS attacks and being traced 
geographically based on their IP. Additionally, a public and private key 
pair is inexpensive to generate allowing for recycling of identity in 
the case of spam or targeting. Public-keys can be used to sign and 
encrypt messages, allowing for verification and un-traceability of 
senders.

**Client** \
**Clients should encrypt _ALL_ plaintext messages with the recipients 
public key.** If clients encrypt messages with the recipients public 
keys, only the intended recipient can read the contents of the message. 
If only the intended recipient can read a message, a server routing 
messages to a non-intended location will not compromise the integrity of 
said messages. **Clients should also handle the storage of all 
messages.** By clients handling messages, clients can rely on themselves
for message history and history depth.

**Server** \
**Servers should be the _ONLY_ component that stores client IPs.** The 
purpose of the server is to abstract away the IP addresses of senders 
with public keys. By limiting servers to this scope we can trust them as
little as possible. All the servers need to do is propagate messages 
across clients. The rest is sugar.

## Component Details
### Client
 - `HTML5` with `CSS3` in style tag and `JS (ES6)` in script tag (for 
 portability)
 - Use of websockets in accordance to the 
 [RCF 6455](https://datatracker.ietf.org/doc/html/rfc6455) specification
 using the browser's native
 [WebSocket API](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API)
 - `RSA` key-pair generation, signing, and encryption implemented with the [Web Crypto API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Crypto_API)
 - Storage of application state and messages using the browser's native
 [Web Storage API](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
 - _Zero_ External Dependencies
 
 ### Server
 - `Rust` `websocket` server. Chosen for its
[speed](https://www.researchgate.net/profile/Matt-Tomasetti-2/publication/348993267_An_Analysis_of_the_Performance_of_Websockets_in_Various_Programming_Languages_and_Libraries/links/601a30ba299bf1cc269cdd6f/An-Analysis-of-the-Performance-of-Websockets-in-Various-Programming-Languages-and-Libraries.pdf)
and popularity. And also, I just wanted to try it :p
 - Using `tokio` for the asynchronous runtime. Interesting discussion about Tokio
[here](https://www.reddit.com/r/rust/comments/u8uw3z/is_tokio_slow_or_is_it_just_being_compared/)
 - Using `tokio-tugstenite` for the websockets. Why? Because of 
[this discussion](https://www.reddit.com/r/rust/comments/goxm85/which_websocket_library_to_use/)
