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
  - Stores messages locally
  - Encrypts messages with intended reciever's public key 
  (Assymetric Encryption)
  - Verifies public-keys
- Server
  - Cloud-based \
  **Scaleable**
  - Containerized into docker image
  - Deployable to kubernetes
  - Use websockets to communicate with clients \
  **Near Zero-Trust**
  - Verifies public-keys
  - Route encrypted messages to clients based off public-key 
- Notes
  - Client IPs should ONLY be visible to the proxy server they are 
  connected to
  - Messages will ONLY be visible in plaintext to the intended recepient
  - Encryption and decryption should be handled by the clients
### Feature Detail
**Identity** \
**Identities in a zero-trust system should be `recyclable`, `verifiable` 
and `untracable`.** This zero-trust model allows for security of client 
data over insecure networks and malicious middlemen. Unlike an IP, a 
public-key cannot be attributed to a clients network. This lack of 
attribution sheilds the client against DDoS attacks and being traced 
geographically based on their IP. Additionally, a public and private key 
pair is inexpensive to generate allowing for recycling of identity in 
the case of spam or targeting. Public-keys can be used to sign and 
encrypt messages, allowing for verification and untracability of 
senders.

**Client** \
**Clients should encrypt ALL plaintext messages with the recepients 
public key.** If clients encrypt messages with the recepients public 
keys, only the intended recepient can read the contents of the message. 
If only the intended recepient can read a message, a server routing 
messages to a non-intended location will not compromise the integrity of 
said messages.

**Server**