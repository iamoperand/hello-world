// ├─ src
// │  ├─ lib.rs -> registering modules
// │  ├─ entrypoint.rs -> entrypoint to the program
// │  ├─ instruction.rs -> program API, (de)serializing instruction data
// │  ├─ processor.rs -> program logic
// │  ├─ state.rs -> program objects, (de)serializing state
// │  ├─ error.rs -> program specific errors

// The flow of a program using this structure looks like this:
// - Someone calls the entrypoint
// - The entrypoint forwards the arguments to the processor
// - The processor asks instruction.rs to decode the instruction_data argument from the entrypoint function.
// - Using the decoded data, the processor will now decide which processing function to use to process the request.
// - The processor may use state.rs to encode state into or decode the state of an account which has been passed into the entrypoint.

pub mod entrypoint;
pub mod processor;
pub mod state;
