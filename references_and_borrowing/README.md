##  References

* A reference stores the memory address of a value
* Borrowing means creating a reference
* References enable the reuse of data with moving ownership

##  Immutable References

* Reference are immutable my default
* An immutable reference does not have permission to modify the original value at the memory address
* A value can have any number of immutable references. There is no risk
* Immutable references implement the copy trait. Rust will create a full copy in situations where one is needed (variable assignment, function parameters, variable inside array, etc)

##  Mutable References

* An mutable reference has permission to modify the original value at the memory address
* A value can only have one mutable reference at a time
* Mutable references do not implement the copy trait. Ownership will move on variable reassigment
* The compiler understands the references's lifetime, which is the time it is being utilized in the program. A lifetime can end before the function's scope

##  Dangling References

* A dangling references is a pointer to a memory addres that has been deallocated
* Dangling references create bugs and unpredictable behaviors in other programming languages
* The Rust compiler prevents dangling references. A reference is guaranteed to point to valid data
* The reference (the original data) must outlive the reference  