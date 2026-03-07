Practicing Rust - from the easiest to the hardest.


### Phase 1: Syntax, Ownership, and Muscle Memory

**1. Temperature Converter**
Create a command-line utility that prompts the user to enter a temperature and its unit (Celsius or Fahrenheit). Read the input, parse the string into a floating-point number, perform the correct mathematical conversion, and print the result. This will get you comfortable with basic I/O and shadowing.

**2. Number Guessing Game**
Build a program that generates a random number between 1 and 100 using the `rand` crate. Prompt the user to guess the number in a `loop`. Use a `match` expression to compare the guess to the secret number, printing "Too high!", "Too low!", or "You win!" and breaking the loop when they guess correctly.

**3. Nth Fibonacci Generator**
Write a function that takes an integer `n` and returns the `n`th number in the Fibonacci sequence. Try implementing it twice: once using recursion and once using a simple loop (iteration). This will test your understanding of basic integer types and control flow without hitting performance bottlenecks.

**4. CLI Calculator**
Create a program that takes a simple mathematical expression from standard input (e.g., "5 + 3" or "10 / 2"). Define an `enum` representing the four basic operations (Add, Subtract, Multiply, Divide). Parse the input, map the operator to your enum, and use a `match` statement to execute the operation and print the result.

**5. Pig Latin Converter**
Write a script that converts a given string of text into Pig Latin. If a word starts with a consonant, move the first consonant to the end of the word and add "ay" (e.g., "first" becomes "irst-fay"). If a word starts with a vowel, just add "hay" to the end (e.g., "apple" becomes "apple-hay"). This forces you to deal with Rust's strict UTF-8 string handling.

**6. Mean, Median, & Mode Finder**
Given a hardcoded list of integers (a `Vec<i32>`), write functions to calculate the mean (average, requiring a cast to `f64`), the median (middle value, requiring you to sort the vector first), and the mode (most frequent value, requiring a `HashMap` to track occurrences).

**7. Shape Area Calculator**
Define three `struct`s: `Circle`, `Rectangle`, and `Triangle`, each with the necessary dimensional fields. Define a trait called `Area` with a method `calculate_area(&self) -> f64`. Implement this trait for all three structs, create an instance of each, and print their areas.

**8. Company Directory**
Build a text interface to manage employees and departments. The user should be able to type commands like "Add Sally to Engineering" or "Add Amir to Sales". Store this data using a `HashMap<String, Vec<String>>`. Allow the user to request a list of all people in a specific department, or an alphabetically sorted list of all people in the company.

**9. Simple Todo List**
Create a persistent task manager. Define a `Task` struct with a description and a boolean `completed` state. Store these in a `Vec<Task>`. Run a continuous loop where the user can choose to add a task, view all tasks, mark a task as complete by its index, or quit the application.

**10. Custom Error Types**
Write a program that simulates reading a configuration file. Create a custom enum called `ConfigError` with variants for `FileNotFound`, `PermissionDenied`, and `ParseError`. Implement the `std::error::Error` trait for it. Write a function returning `Result<Config, ConfigError>` that purposefully triggers one of these errors, and use the `?` operator to propagate it.

---

### Phase 2: Idiomatic Rust and the Standard Library

**11. Minigrep**
Recreate the classic `grep` tool (as outlined in the Rust book). Your program should accept a search string and a filename as command-line arguments. Read the file, search for the string line-by-line, and print matching lines. Implement an environment variable (`IGNORE_CASE=1`) to toggle case-insensitive searching.

**12. JSON Parser/Formatter**
Using the `serde` and `serde_json` crates, write a program that reads a messy, unformatted JSON file from your disk. Define Rust `struct`s that accurately represent the JSON schema, deserialize the file into your structs, and then serialize it back to a new file, but pretty-printed with proper indentation.

**13. Conway's Game of Life**
Implement the famous cellular automaton rules on a 2D grid. Represent the grid using a `Vec<Vec<bool>>` (or a flat 1D vector with math for indexing). Calculate the next generation based on the number of living neighbors, and print the grid to the terminal in a loop, clearing the screen between "frames".

**14. LRU Cache**
Build a Least Recently Used (LRU) cache. You will need a `HashMap` for fast key-value lookups, but you also need to track the order of access to know which item to evict when the cache reaches its capacity limit. Doing this purely in safe Rust without standard doubly-linked lists is a great test of managing references and state.

**15. Custom Smart Pointer**
Create your own basic version of `Box<T>`, let's call it `MyBox<T>`. It should wrap a value. Implement the `Deref` trait so you can use the `*` operator on `MyBox` just like a regular reference. Then, implement the `Drop` trait to print a custom message to the console exactly when the pointer goes out of scope, proving you understand memory cleanup.

**16. Brainfuck Interpreter**
Write an interpreter for the esoteric language Brainfuck. The language only has 8 simple commands (like `>`, `<`, `+`, `-`, `[`, `]`). You will need to create an array (or `Vec<u8>`) representing the memory tape, a data pointer, and a state machine to handle the nested loop brackets correctly.

**17. Markdown to HTML Converter**
Create a script that reads a simple Markdown file and outputs standard HTML. Focus on parsing basic elements: convert lines starting with `#` to `<h1>` tags, text wrapped in `**` to `<strong>` tags, and standard text blocks into `<p>` tags. This will test your string slicing and pattern-matching abilities.

**18. Custom Iterators**
Create a custom data structure—for example, a `RingBuffer` or a simple binary tree. Implement the `Iterator` trait for it so that you can use your custom structure directly in a `for` loop (e.g., `for item in my_tree { ... }`). You will need to define the `type Item` and implement the `next()` method.

**19. Basic Key-Value Store**
Build a highly simplified, persistent database. When a user sets a key-value pair, append that command to an on-disk log file. When the database starts up, it should read the log file from disk and replay all the commands into an in-memory `BTreeMap`. This introduces you to file-backed persistence.

**20. Multi-threaded Web Scraper**
Create a program that takes a list of URLs. Using `std::thread`, spawn a separate thread for each URL. Use the `reqwest` crate (in blocking mode) to download the HTML for each site concurrently. Extract the `<title>` tag from each page and send the results back to the main thread using an `mpsc` channel.

---

### Phase 3: Advanced Architectures and the Ecosystem

**21. Thread Pool Implementation**
Instead of spawning a new thread for every task (which can crash a system under heavy load), build a Thread Pool. Create a struct that pre-allocates a fixed number of worker threads. Use an `Arc<Mutex<mpsc::Receiver<Job>>>` to allow the worker threads to safely share a queue of closures (Jobs) waiting to be executed.

**22. Async Weather Fetcher**
Dive into asynchronous Rust. Use the `tokio` runtime and the asynchronous version of `reqwest`. Connect to a public weather API (like Open-Meteo) and fetch the current weather for five different global cities simultaneously. Use `tokio::join!` or `future::join_all` to wait for all the asynchronous network requests to finish before printing the results.

**23. Declarative Macro (`macro_rules!`)**
Rust lacks a built-in way to easily initialize a `HashMap` with data, similar to the `vec![]` macro. Write a declarative macro called `hashmap!` that allows you to write `let map = hashmap!("a" => 1, "b" => 2);`. This will teach you macro syntax, repetition patterns, and AST matching.

**24. Procedural Macro**
Write a custom `#[derive]` macro. Create a separate crate of type `proc-macro`. Write a macro called `#[derive(Describe)]` that can be attached to any `struct`. It should automatically generate an `impl` block for that struct containing a method `fn describe(&self)` that prints the exact names of the fields the struct possesses at compile time.

**25. TCP Chat Server & Client**
Build two separate programs. First, a server using `std::net::TcpListener` that accepts incoming connections, spins up a thread for each, and broadcasts any received messages to all other connected clients. Second, a client program that connects to the server and handles reading from the server and writing user input simultaneously.

**26. Doubly Linked List**
It is famously difficult to write a safe, performant Doubly Linked List in Rust due to multiple mutable references pointing to the same nodes. Attempt to write one. You will have to step out of safe Rust and use `unsafe` blocks, raw pointers (`*mut T`), and the `NonNull` wrapper. This will teach you exactly why the borrow checker acts the way it does.

**27. Terminal User Interface (TUI)**
Use the `ratatui` crate (and `crossterm` for backend) to build a rich, interactive terminal application. Build a dashboard that displays different "widgets" (boxes, charts, lists). Implement an event loop that listens for keyboard presses (like arrow keys) to let the user navigate between different UI panes without the terminal scrolling.

**28. Async Web Server from Scratch**
Do not use a high-level framework like Axum or Actix. Using only `tokio::net::TcpListener` and `tokio::io`, build an asynchronous web server. You will need to manually parse the incoming byte stream into HTTP GET/POST requests, route the path to a specific async handler function, and format raw HTTP response strings (with headers and status codes) to send back.

**29. Blockchain Simulation**
Create a simulation of a basic blockchain. Define a `Block` struct containing an index, timestamp, data, the hash of the previous block, and its own hash. Implement a "proof of work" system where you increment a nonce until the block's hash starts with a certain number of zeroes. This requires strict cryptographic hashing (using the `sha2` crate) and complex state management.

**30. CHIP-8 Emulator**
The ultimate capstone. Build an emulator for the 1970s CHIP-8 system. You will need to model the CPU (registers, program counter, index register), 4KB of RAM, a 64x32 monochrome display, and a hex keyboard. You will read binary ROM files, decode the opcodes bit-by-bit, and execute the correct instructions. Connect the display buffer to a windowing crate like `minifb` to actually play games like Pong or Space Invaders.
