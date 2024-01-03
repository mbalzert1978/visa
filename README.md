# Credit Card Validator

A Rust program to validate credit card numbers based on certain rules.

## Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [Usage](#usage)
- [Build and Run](#build-and-run)
- [License](#license)

## Introduction

This Rust program validates credit card numbers against specific rules to determine their validity. It checks for the correct format and performs sum checks to ensure compliance with common credit card validation rules.

## Features

- Validates credit card numbers based on specific rules.
- Displays whether a credit card number is valid or not.

## Usage

To use the Credit Card Validator, you need to provide a credit card number as a command-line argument. The program will then validate the provided credit card number and display the result.

Example:

```bash
cargo run -- "1234 5678 9012 3450"
```

## Build and Run
Ensure you have Rust installed. If not, follow the official Rust installation guide.

Clone the repository:
```bash
git clone https://github.com/mbalzert1978/visa.git
cd visa
```
Build the project:
```bash
cargo build --release
```
Run the program with a credit card number:
```bash
./visa "1234 5678 9012 3450"
```

## License
This project is licensed under the MIT License - see the [LICENSE](https://opensource.org/license/mit/) file for details.
