# Black Cracker Rusty

For the Chinese version of the description, please refer to [中文版说明](/README_CN.md).

I have recently become obsessed with playing Rust and thought of creating a web or cryptography tool using Rust.

This project is still a work in progress. I'm creating a repository for it and will slowly work on improving it when I have time. 😁...

I welcome contributions from all the experts out there. Please give it a star. Feel free to share your ideas or raise issues for discussion. I hope to include your avatar in the acknowledgments.

Love and Peace

> This project is a password cracking framework written in Rust.

> It integrates various password cracking functionalities, including dictionary attacks, weak password detection, and hash cracking.

> etc...

## Repository

Github: [BlackCracker_Rusty](https://github.com/DXHM/BlackCracker_Rusty)

Release: [Release](https://github.com/dxhm/BlackCracker_Rusty/releases/latest)

## Functionality

- **Dictionary Attack Mode**: Attempts to crack a target password by iterating through a dictionary file.
- **Weak Password Detection Mode**: Evaluates the security of a target password by checking if it is a weak password.
- **Hash Cracking Mode**: Attempts to restore the original password by cracking its hash value.

## Installation

1. Clone the project to your local machine:

   ```bash
   git clone https://github.com/DXHM/BlackCracker_Rusty.git
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

## Usage

- `<mode>`: Select the mode you want to run, which can be `dictionary`, `weak_password`, or `hash_cracker`.
- `<target>`: The target password or hash value.

## Example

### Linux

- Dictionary Attack Mode:

  ```bash
  blackcracker_rusty dictionary password123
  ```

- Weak Password Detection Mode:

  ```bash
  blackcracker_rusty weak_password user1
  ```

- Hash Cracking Mode:

  ```bash
  blackcracker_rusty hash_cracker 5f4dcc3b5aa765d61d8327deb882cf99
  ```

### Windows

- Dictionary Attack Mode:

  ```bash
  blackcracker_rusty.exe dictionary password123
  ```

- Weak Password Detection Mode:

  ```bash
  blackcracker_rusty.exe weak_password user1
  ```

- Hash Cracking Mode:

  ```bash
  blackcracker_rusty.exe hash_cracker 5f4dcc3b5aa765d61d8327deb882cf99
  ```

## Requirements

- rust-crypto = "^0.2"
- embed-resource="^2.0"

## Contribution

[<img alt="AShujiao" src="https://avatars.githubusercontent.com/u/69539047?v=4" width="117">](https://github.com/dxhm)

## License

---