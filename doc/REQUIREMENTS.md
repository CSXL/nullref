# Requirements
## High-Level Features
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
  - Encrypts messages with intended reciever's public key (Assymetric Encryption)
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
  - Client IPs should ONLY be visible to the proxy server they are connected to
  - Messages will ONLY be visible in plaintext to the intended recepient
  - Encryption and decryption should be handled by the clients
