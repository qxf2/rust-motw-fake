# rust-fake-crate-examples
This repository contains a collection of Rust code examples demonstrating the use of the `fake` crate for generating fake data and various Rust language features. 

## Introduction
The fake crate is useful for creating mock data for different types of data without having to write a lot of code. For instance, if you wish to evaluate your program’s performance with different inputs or need illustrative examples, this crate allows you to generate authentic-looking data for said purposes. It uses the rand crate as a dependency and implements the Dummy and Fake traits for many common types, as well as providing the faker module with various helpers for generating fake values in different formats and locales. Additionally, it also provides macros and functions for creating collections and custom types with fake value. Examples showcasing all these features are provided in this repo.

## Installation

1. Make sure you have Rust installed. If not, you can install it using [rustup](https://rustup.rs/).

2. Clone this repository to your local machine:
   ```bash
   git clone https://github.com/qxf2/rust-fake-crate-examples.git
   ```

## Usage

1. Navigate to the project directory

```bash
cd rmotw_fake/
```

2. Run the required code example:
```bash
cargo run --bin <name_of_file>
```
Eg:
```bash
cargo run --bin fake_trait_usage
```

## Examples

The examples provided here use a social media app as a context, showcasing the generation of fake data that can help test the features of the app. Designed to be simple and get you started, these examples illustrate how to generate synthetic data using the various features provided by the fake crate.

Here is how to run one example and what the output looks like.

Example1: Basic Usage of Fake Trait: 

The fake crate in Rust offers a generic trait named Fake, allowing users to implement it for various types (Eg: string-based data, numeric data etc.). By implementing the Fake trait for a specific type, we can generate random or fake instances of that type. This example shows how we can do that.

```bash
rmotw_fake$ cargo run --bin fake_trait_usage
   Compiling rmotw_fake v0.1.0 
     Running `target/debug/fake_trait_usage`
Likes: 576
Shares: 69
Post: "zIHz14dHooiwpPh9w0DeWLTLiRkEvYtSdVKerbW5fm9h"
Caption: "tRiadlcjP"
Category: "normal"
```
