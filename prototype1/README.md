# Prototype 1
## Use cases
This prototype will implement the hello world for the software cad project.
Initially, it will be a very simple project which is able to handle very simple
use cases, namely, only a main function (the first semantic element), simple
functions (the second semantic element) and constants (the third semantic element).

- Main function use case: The user creates a main function with the possibility
to print "Hello world" from it. The user has access to a simple control panel that
allows creating the resources. The possible resources that he will be able to create
are: main function, simple function, constant. After creating the main function, 
the user will execute it, seeing the printed message in his screen.

- The user can select a fast context action to create a new resource from the editing
of another resource.
    - Examples:
        1. I want to create a function from the main. I type a name that is not
        recognized in the current scope. The text editor should suggest context 
        actions for me. 
        1:type "add(1, 2)" -> 2: text editor suggests context actions.
        1. Another possibility is that I can use fast binding keys to create a new resource.
        For example, suppose that I want to create a function to use in some scope.
        I can have the following path: Press: CTRL+Shift+F1 and a new text field appears
        for me to type the name of the function. After that, the name of the function will
        be automatically inserted in the place where my cursor is. In the future, 
        the compiler must be able to analyze the context to suggest an initial signature
        for my function.

- Every resource created by the user must store its history metadata in a git like
way. The user can see all that history in an easy way.

- The user can edit a resource in multiple ways. The first one is using the TUI
options. But not only that way. It will be possible to edit also the source code
of the resource, if it makes sense. 
    - Examples:
        1. I can edit the body source code of a Main resource using a text editor.
        1. I can edit the value of a constant using a simplified version of an editor.
        It can be an embedded editor inside a text field, for example.

## Requirements

- [X] **A basic TUI for the home**: 
    - The user will be able to select one of the following options in the home:
        1. Execute a resource: Choose a Main to execute (pre-compile, compile, opens a shell, and execute)
        1. Create a resource
            1. Create a Program
            1. Create a Main
            1. Create a Function
            1. Create a Constant.
        1. Manage a resource
            - Redirects to the page that allows managing the resources
1. **Create the basic resources**:
    - [ ] Main: the main resource represents the main() function of rust. It contains multiple components:
        - Components:
            - id
            - name
            - Metadata:
                - Author
                - Version
                - Timestamp
                - Documentation
                    - Description
                    - Examples
            - Signature:
                - return type
            - History of bodies: version control for this component. Contains 
            multiple bodies (linked list), which are detailed below:
                - body: 
                    - Rust source code
                    - Committed message
                    - Timestamp: number of seconds from Epoch 
                    - Commit author
                    - Illustration: [src1, "initial commit", 123123, "A1"]->[src2, "change 1", 12312323, "A2"]->[src3, "commit 3", 1231235345, "A1"]
    - [ ] Function: represents any fn element that could be created on a body of a
    rust file. Its components are:
        - Components:
            - id
            <!-- this is only for user experience purpose. Instead of identifying a programming element by its "pretty name", the id could be used.  -->
            - name 
            - Metadata:
                - Author
                - Version
                - Timestamp
                - Documentation
                    - Description
                    - Examples
                - previous version: Id of the old version of this function. if a
                function "changes" its signature, it actually creates a new one 
                with different ID. The user is then prompted to choose a new
                name for the new/previous function 
            - Signature: the name is not part of the signature because it does not
            matter in the pre-compilation time.
                - return type
                - arguments: list of arguments
                - implicit stateful arguments: global variables, (others?)
            - History of bodies: version control for this component. Contains 
            multiple bodies (linked list), which are detailed below:
                - body: 
                    - Rust source code
                    - Committed message
                    - Timestamp: number of seconds from Epoch 
                    - Commit author
                    - Illustration: [src1, "initial commit", 123123, "A1"]->[src2, "change 1", 12312323, "A2"]->[src3, "commit 3", 1231235345, "A1"]

    - [ ] Constant: represents any const element that could be created on a body of a
    rust file. Its components are:
        - Components:
            - id
            <!-- this is only for user experience purpose. Instead of identifying a programming element by its "pretty name", the id could be used.  -->
            - name 
            - Metadata:
                - Author
                - Version
                - Timestamp
                - Documentation
                    - Description
                    - Examples
                - previous version: Id of the old version of this constant. 
            - Signature: the name is not part of the signature because it does not
            matter in the pre-compilation time.
                - type
            - History of values: version control for this component. Contains 
            multiple values (linked list), which are detailed below:
                - value: 
                    - Rust expression that returns this value and is a valid const
                    - Committed message
                    - Timestamp: number of seconds from Epoch 
                    - Commit author
                    - Illustration: ["Hello World", "initial commit", 123123, "A1"]->["Hello", "change 1", 12312323, "A2"]->["Hello\n", "commit 3", 1231235345, "A1"]


## Lessons learnt

