# Process Simulator

This Rust application simulates the creation of background processes based on names listed in a `processes.txt` file. It's designed to mimic the presence of these processes in the system's task manager without performing any actual operations.

## Project Structure

- `src/main.rs`: Contains the main program that reads process names from `processes.txt` and spawns dummy processes.
- `src/dummy.rs`: A dummy program that does nothing but run indefinitely to simulate a background process.

## Requirements

- Rust Programming Language
- Cargo (Rust's build system and package manager)

## Setup

1. **Clone the Repository**
   
Clone this repository to your local machine using:
   
`git clone <repository-url>`


2. **Navigate to the Project Directory**

Change into the project directory:

cd <project-directory>


3. **Build the Project**

Build the project with Cargo:

`cargo build --release`

This will compile both the main program and the dummy program, placing the executables in `target/release`.

## Usage

1. **Prepare the Process List**

Create a `processes.txt` file in the project root containing the names of the processes you want to simulate, one per line. For example:

```
SysInfoCap.exe
NetworkCap.exe
DiagsCap.exe
```

2. **Run the Main Program**

Execute the main program using Cargo:

`cargo run --release`

This will read each line from `processes.txt` and spawn a corresponding dummy process that will appear in the system's task manager.

## Important Notes

- The dummy processes do nothing and simply sleep indefinitely. They can be terminated from the task manager.
- This program does not require administrative privileges unless it needs to simulate processes that would normally require such privileges.
- Ensure that your antivirus software allows the execution of these dummy programs, as it might mistake them for suspicious activity.

## Contributing

Contributions to this project are welcome. Please fork the repository, make your changes, and submit a pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
Notes
