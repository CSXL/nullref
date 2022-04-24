# Requirements
## High-Level Components
- Identity
  - Public-Key Identification
  - Public-Key Verification
  - Session-specific alias
- Client
  - Browser-based
  - As close to static HTML and vanilla JS as possible
  - Self-contained
  - Verify public-keys
  - Use asymmetric encryption (RSA) to encrypt messages
- Server
  - Simple, simple, simple
  - Verify public-keys
  - Handle connections and route messages to respective clients
- Security
  - Client IPs should ONLY be visible to the proxy server attatched to their public-key
  - Messages should ONLY be visibile in plaintext to the clients
  - Encryption and decryption should be handled by the clients