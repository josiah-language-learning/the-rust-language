# Functions

`main` is a function that exists in every Rust executable.  `fn` is used to
declare functions. **_snake case_** is the Rust community's convention for
naming functions _AND_ variables.

Rust does not care where you define your functions; ordering is not important,
unlike C.

## Function Parameters

- Every function parameter requires a type signature.  _This decision was made
  in order to optimize rustc's ability to infer types._
  
## Function Bodies Contain Statements and Expressions

- Made-up of statements optionally concluded by an _"expression"_

### Statements vs Expressions

- **statement:** _an instruction that performs an action_
    - ```rust
      let y = 6;  // ends in a ;
      ```
    - Each and every function definition is a statement.
      ```rust
      fn halitosis(person: Person) {
          // Does some things to determine if 'person' has halitosis
      }
      ```
    - Statements do not return values _(I think this is in contrast to `Ruby`)_.
      The following errors:
      ```rust
      let x = (let y = 6);
      ```
      _See invalid-statement.rs & invalid-statement.compile.err and note that
      there seems to be experimental support for this syntax, presently. Perhaps
      this will change in the future to be more like they way C & Ruby are?_
- **expression:** _evaluate to a compute some value_
    - `5 + 6` is an expression because it evaluates to `11`.
    - Expressions may be part of a _statement_
    - Every function call and macro call is an expression (because every
      function and macro evaluates to some value).
    - Blocks may be used to create new expressions, for example:
      ```rust
      fn main() {
          let x = 5;
          
          // A block is used to set y = to the result of a complex expression
          let y = {
                let x = 3;
                x + 1  // ending with expression returns the expr result.
          };  // notice, a semicolon is needed here to close the 'let' statement
          
          println!("The value of y is: {}", y);  // => The value of y is 4
      }
      ```
