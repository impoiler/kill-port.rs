
# kill-port

A simple command-line utility written in Rust to kill processes running on a specific port on macOS.

## Problem it Solves

When developing web applications, sometimes ports remain occupied even after stopping the application, especially with React/Node.js development servers. This utility provides a quick way to free up a port by killing any process that's using it.

## Installation

### Prerequisites
- Rust (if you're building from source)
- macOS (this utility uses macOS-specific commands)
- `lsof` command (usually pre-installed on macOS)

### Building from Source

1. Clone this repository:
```bash
git clone https://github.com/impoiler/kill-port.rs.git

cd kill-port
```

2. Build the binary:
```bash
rustc kill-port.rs
```

3. Make it globally available (optional):
```bash
sudo mv kill-port /usr/local/bin/
```

## Usage

Basic usage:
```bash
kill-port <port_number>
```

Example:
```bash
kill-port 3000  # Kills process running on port 3000
```

### Common Use Cases

Kill a React development server:
```bash
kill-port 3000
```

Kill a Next.js development server:
```bash
kill-port 3000
```

Kill a Node.js server:
```bash
kill-port 8080
```

## How it Works

1. Takes a port number as a command-line argument
2. Uses `lsof` to find processes using that port
3. Forcefully terminates the processes using `kill -9`

## Error Handling

The utility handles several error cases:
- Invalid port number
- No process found on specified port
- Failed kill attempts
- Missing command-line arguments

## Notes

- Uses `kill -9` which forcefully terminates processes
- Requires appropriate permissions to kill processes
- Works only on macOS (due to `lsof` command usage)

## Development

The source code is written in Rust and consists of a single file (`kill-port.rs`). It uses standard library features and system commands.

### Code Structure
- Argument parsing
- Port validation
- Process discovery using `lsof`
- Process termination using `kill`

## Contributing

Feel free to submit issues and enhancement requests!

## License

MIT License

## Author

[Your Name]

## Acknowledgments

Thanks to the Rust community and tools like `lsof` that make this utility possible.

## Troubleshooting

### Common Issues

1. **Permission Denied**
   ```bash
   sudo kill-port <port_number>
   ```

2. **Command Not Found**
   - Ensure the binary is in your PATH
   - Check if you've moved it to `/usr/local/bin/`

3. **Port Still in Use**
   - Try running the command with sudo
   - Ensure no other process has immediately taken over the port

## Future Improvements

- [ ] Add support for other operating systems
- [ ] Add a "gentle" kill option (without -9)
- [ ] Add support for killing processes by name
- [ ] Add verbose mode for debugging