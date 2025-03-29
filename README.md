# Toy Web Browser Render Engine

A lightweight implementation of a web browser rendering engine built for educational purposes. This project helps understand the fundamental concepts behind how web browsers parse HTML, build DOM trees, and render web pages.

## Overview

This project creates a simple rendering engine in Rust that can:
- Parse HTML documents
- Build a Document Object Model (DOM)
- Pretty-print the DOM tree structure
- (Future capability) Apply basic CSS styling
- (Future capability) Layout and render elements

## Getting Started

### Prerequisites

- Rust (stable version)
- Cargo (comes with Rust)

### Installation

```bash
# Clone this repository
git clone https://github.com/yourusername/toy-web-render.git

# Navigate to the project directory
cd toy-web-render

# Build the project
cargo build
```

### Usage

```bash
# Run the renderer with the example HTML file
cargo run
```

## Project Structure

```
toy-web-render/
├── src/                # Source code
│   ├── main.rs         # Main entry point
│   ├── dom.rs          # DOM implementation
│   └── css.rs          # CSS parsing (future)
├── example/            # Example HTML files for testing
│   └── test.html       # Test HTML file
├── Cargo.toml          # Rust package manifest
└── README.md           # This file
```

## Features

- **HTML Parser**: Converts HTML string into tokens and builds a DOM tree
- **DOM Implementation**: Simple representation of the Document Object Model
- **Pretty Printing**: Visualize the DOM tree structure in the console

## Exercises

1. Input an HTML file from the `example/` directory
2. Pretty print the node structure of the DOM
3. (Advanced) Implement basic CSS parsing
4. (Advanced) Add layout calculation for elements
5. (Advanced) Render simple boxes to represent elements

## How It Works

1. The HTML parser tokenizes the input HTML
2. It constructs a tree representation of the document (DOM)
3. The renderer traverses the DOM to produce output
4. (Future) Style calculation and layout will be added

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- This project is inspired by educational resources about browser rendering engines
- Thanks to all contributors who spend time to help improve this project
