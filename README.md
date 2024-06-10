# API Benchmarking

This repository contains a comparative analysis of building an API using three different programming languages: Node.js, PHP, and Rust. The purpose of this project is to evaluate the performance and speed of each language when it comes to handling concurrent requests and serving content from a database.

## Overview

The Headless CMS API is designed to retrieve content from a PostgreSQL database and serve it as JSON responses. The API endpoints are implemented in Node.js, PHP, and Rust, allowing for a direct comparison of their performance characteristics.

Each implementation follows a similar structure and uses the following technologies:

- Node.js: Express.js, pg (PostgreSQL client)
- PHP: Built-in web server, pg (PostgreSQL client)
- Rust: Actix Web, Deadpool PostgreSQL, Tokio PostgreSQL

## API Endpoint

The API exposes a single endpoint:

- `GET /api/data`: Retrieves all content from the database and returns it as a JSON response.

## Performance Testing

To assess the performance of each language implementation, the repository includes scripts and instructions for conducting benchmarking tests using Apache Bench (ab) or Siege. The tests measure the response times, throughput, and concurrency handling capabilities of each API.

The benchmarking process involves sending a specified number of requests with varying concurrency levels to each API endpoint and recording the results. The collected data is then analyzed and visualized to compare the performance metrics of Node.js, PHP, and Rust.

## Getting Started

To run the APIs locally and perform benchmarking tests, follow these steps:

1. Clone the repository: `git clone https://github.com/JordanRobo/API_Speed_Tests.git`
2. Set up a PostgreSQL database and update the database configuration in each language's implementation.
3. Install the necessary dependencies for Node.js, PHP, and Rust.
4. Start the API servers for each language.
5. Run the benchmarking tests using the provided scripts and tools.
6. Analyze the results and compare the performance of Node.js, PHP, and Rust.

Detailed instructions for each step can be found in the individual language directories.

## Results

The benchmarking results, including graphs and tables comparing the performance metrics of Node.js, PHP, and Rust, are presented in the `results` directory. The findings showcase the strengths and weaknesses of each language in terms of handling concurrent requests and serving content efficiently.

## Conclusion

This project demonstrates the capabilities of Node.js, PHP, and Rust in building a Headless CMS API. By evaluating their performance through benchmarking tests, developers can make informed decisions when choosing a language and framework for their Headless CMS projects. The results highlight the advantages of Rust's performance, concurrency handling, and safety features, making it a compelling choice for building high-performance and scalable Headless CMS backends.

Feel free to explore the code, run the tests, and contribute to the project. Your feedback and suggestions are welcome!

## License

This project is licensed under the [MIT License](LICENSE).
