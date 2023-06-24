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

### writing standard error instead of standard output

<h6> stderr is used to output standard error</h6>
- eprintln! macro is used to print to the standard error stream.

- we read and write an output from the shell using > that means write to a specified file. best practices are to use stderr for err that my arise and stdout for standard output.

# try runnung the code

use cargo run -- [word to find] [file to read from (poem.txt)] > output.txt.

- note the change on output.txt

* a refactor to use iterators

- we use iterators instead of the expensive clone that makes allocates a new memory of an item in the heap.
