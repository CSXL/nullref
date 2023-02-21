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
