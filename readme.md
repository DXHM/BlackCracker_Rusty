# Black Cracker Rusty

## Project Description

Black Cracker Rusty is a password cracking framework built with Rust. It integrates various password cracking functionalities, including dictionary attack, weak password detection, and hash cracking.

## Features

- **Dictionary Attack Mode**: Attempts to crack a target password by iterating through passwords in a dictionary file.
- **Weak Password Check Mode**: Checks if a target password is weak and provides a password strength assessment.
- **Hash Cracking Mode**: Attempts to crack password hashes and recover the original passwords.

## Installation

1. Clone the project to your local machine:

   ```bash
   git clone https://github.com/dxhm/Blackcracker_rusty.git
   ```

2. Navigate to the project directory:

   ```bash
   cd Blackcracker_rusty
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

4. Run the project:

   ```bash
   cargo run -- <mode> <target>
   ```

   - `<mode>`: Select the mode to run, which can be `dictionary`, `weak_password`, or `hash_cracker`.
   - `<target>`: The target password or hash value.

## Usage Examples

- Dictionary Attack Mode:

  ```bash
  blackcracker_rusty dictionary password123
  ```

- Weak Password Check Mode:

  ```bash
  blackcracker_rusty weak_password user1
  ```

- Hash Cracking Mode:

  ```bash
  blackcracker_rusty hash_cracker 5f4dcc3b5aa765d61d8327deb882cf99
  ```

## Dependencies



## Contributing



## License



---
