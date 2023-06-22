<!-- create a binary application mingrep a comand line tool in rust -->

# make minigrep accept two cmd arguments

- file path
- string to search for

<p> eg cargo run -- searchstring filename.txt</P>

<P> Reading the arguements from the command line we will be using std::env::args provided by rust.</p>

## getting the commandline arguments

- organizing our code to increase modularity.
<p> following rust rules during code spliting. logic is put in the lib.rs and the main.rs runs the logic. this is a better approach since we can test the program and all of its logic in one place then we are able to execute it in the main.rs in a more concise way.</p>

### error handling

- we used result i prefer result but there came an operator ? that returns an error value from the current function for the caller to handle

- oops i was wrong and right i changed to use iflet contruct.

### splitting code into a lib crate

- create a lib.rs in src folder
- create a module to write tests we use TDD to develop the application
- in the process we learn about the is_ok method that is used on the result enum. Now, the method returns a boolean if the Ok is true and false if it finds the Err.
