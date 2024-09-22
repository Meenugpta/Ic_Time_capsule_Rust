# Time Capsule Smart Contract

## Overview

The Time Capsule Smart Contract is a decentralized application (dApp) built on the Internet Computer Protocol (ICP). It allows users to create digital time capsules, storing messages that can only be accessed after a specified future date.

## Features

- Create time capsules with customizable unlock times
- Retrieve time capsules once they've reached their unlock time
- List all currently available (unlocked) time capsules

## Prerequisites

Before you begin, ensure you have the following installed:
- [dfx](https://internetcomputer.org/docs/current/developer-docs/build/install-upgrade-remove) (version 0.23.0 or later)
- [Rust](https://www.rust-lang.org/tools/install) (version 1.81.0 or later)
- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (version 1.81.0 or later)

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/Meenugpta/time_capsule_project.git
   cd time_capsule_project
   ```

2. Install the required dependencies:
   ```
   npm install
   ```

3. Start the local Internet Computer replica:
   ```
   dfx start --background
   ```

4. Deploy the canister:
   ```
   dfx deploy
   ```

## Usage

### Creating a Time Capsule

To create a new time capsule:

```
dfx canister call time_capsule_backend create_time_capsule '("Owner Name", "Your message here", <unlock_timestamp>)'
```

Replace `<unlock_timestamp>` with the Unix timestamp when you want the capsule to be unlocked.

### Retrieving a Time Capsule

To retrieve a time capsule (if it's unlocked):

```
dfx canister call time_capsule_backend get_time_capsule '(<capsule_id>)'
```

Replace `<capsule_id>` with the ID of the time capsule you want to retrieve.

### Listing Available Capsules

To list all currently unlocked capsules:

```
dfx canister call time_capsule_backend list_available_capsules
```

## Project Structure

- `src/time_capsule_backend/`: Contains the Rust source code for the canister
- `src/time_capsule_backend/time_capsule_backend.did`: Candid interface file
- `dfx.json`: Project configuration file

## Development

To make changes to the smart contract:

1. Modify the Rust code in `src/time_capsule_backend/src/lib.rs`
2. Update the Candid interface in `src/time_capsule_backend/time_capsule_backend.did` if necessary
3. Rebuild and redeploy the canister:
   ```
   dfx build
   dfx canister install time_capsule_backend --mode upgrade
   ```

## Testing

To run the test suite:

```
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Internet Computer Documentation
- Rust Programming Language
- DFINITY Foundation
