# Software-cad
The Software CAD is an initiative that I took to improve software engineering.
I want, with this ecosystem, to empower the scientists and the engineers to
make them able to look at the field in a more rigorous way, with real metrics.

I think we are for a very long time doing a very poor software engineer process. 

Where is the science in the software engineering field? Everyone is only giving
opinions based on nothing more than a fragile software engineering experience.
We do not have good metrics, good tools, good user experience, etc. 

This ecosystem aims to integrate all the software engineering experience (from 
development to deployment and maintenance) in only one place. It also aims to give
a much better **user experience**. Why does the software engineering tools have such
bad user experience? A lot of tools, a lot of integration is needed, etc. With
this ecosystem, all the experience, from end-to-end, will be very well integrated,
with a focus on performance and user experience. Finally, we will evolve from 
the archaic activity of coding directly in the source code. This will raise one 
level of abstraction, adding the pre-compilation phase. The user will handle
elements that will have a little bit more of abstraction in comparison to archaic
source code.

The focus of this tool will be to first integrate very well the Trivium (Rust,
Dart, and Python) and also the LifeScheduler operating system. 

This ecosystem must provide value to all the following parts of the stream value:
product, development, QA, Infosec, Operations, etc

Components draft: 
    - TUI
    - Product (??)
        - Use cases
        - LifeScheduler (??)
    - Development
        - Granular version control: micro CI/CD, the signatures, documentations, and test suites are the important things, not the "version".
        - code review process
            - automated tools (can apply AI, for example)
            - manual peer review
            - pair programming
        - Micro and macro CI/CD
            - automated tests
        - Debugging tools
            - Performance profiling
            - bugs
        - Documentation
        - Architecture
            - Components/files/functions/code
        - Text editor
            - AI helpers
            - Analyzers
            - Formatters
        - Pre-compiler
        - Compiler (rustc, dartc, python interpreter)
        - Shell
        - Security
            - permissions (micro permission, etc), etc
        - Good abstraction for external elements like databases and external APIs.
    - Operations
        - Infrastructure
        - The continue of the micro/macro CI/CD: depending on the configuration, the natural version update can be passed from the development layer to the operations layer.
        - Monitoring and logging
            - Application Monitoring
            - Log
        - Security
            - Secrets
        - Containers
        - Databases
        - Servers
        - Deploy process
        - Configurations
        - Scalability and performance
            - Load balancers
            - caching
    - Version Control: that element will be pervasive. Not only in the level
    of files, but in the level of semantic unit elements, operation elements, 
    etc. This tool will also help collecting metrics during the process.

## First Steps

My first goal, as a proof of concept is to create a set of prototypes that will
make it possible to create a bootstrap of the project itself. In other words, 
I want to manage this project using itself! It will store the code, organize it,
build, and deploy where necessary. The first programming language that will be
supported is the Rust.

One of the philosophies of this project is its ability to compose with external
components. For example, obviously, it will use rustc, cargo etc as tools for Rust.
But it will do in such a way that it would be easy to change them to other versions.
They are simply loosely coupled components that are integrated to the project.

At first, I will start with a very simple prototype that manages a hello world 
Rust program. It will have a very simple TUI (Terminal User Interface), 
an development component, and a build component. It will handle a very simple
Rust program.


Components draft: 
    - TUI
    - Development
        - main function
        - Text editor
            - Analyzers
            - Formatters
        - Pre-compiler
        - Compiler (rustc)
        - Shell
    - Operations
        - Configurations
        - Build process
    - Version control

## Draft 
**CAVEATS**: This project is still experimental and the ideas below are not
well organized and not necessarily good ideas. The important aspect is to experiment
and to prototype with them to check what will be good or not. One project of this
extent should be done little by little, **discovering** the best path during the
process. Trying to do something "perfect" in the first run is not a good approach.

- I want a tool that helps us to give a new and useful definition for software engineering. 
- This project will help us applying metrics to the software engineering process.
If we create an environment that integrates everything related with the software engineering process, it will be easier to generate enough
data to reason about a good new definition for software engineering. With that, we will be able to create a better software engineering practice.
First, we need data to reason about and a good environment to create experiments that will help us defining the new paths to a good engineering.
- This platform will help anyone developing high quality software. We are going to investigate the better metrics and practices to reach a good
engineering process.
### Characteristics of this platform
- The versioning will be done by each element, not by repository (true versioning, or natural versioning).
- This system will allow automatic "commit" and "pull request" given that each change will be done in a specific semantic element of the software.
- Semantic elements are: functions, classes, interfaces, methods, attributes, files, modules (composed by files, exposing external API that allows other modules to
communicate with them), architectural components (composed by modules, and exposing an external API that allows other architectural components to
communicate with it), global variables, enums, types, structs, traits (in rust), databases, tables, schemas, files, external APIs (for example, if we do not have access to the repository of this API, we can model only the API signatures and mock it if necessary), code templates (pieces of code that allow only editting of a part of the code, the common parts may be edited only for all the instances of the template), architectural components (queues,
APIs, databases, load balancers, security services, caching servers, servers, logging elements, etc) etc.
Make all those things manageable abstractions, make it possible even to manipulate them programmatically. 
Below, there are some extra suggestions from chatGPT (brainstorm):
1. **Interfaces**: Define a contract that classes can implement, ensuring a consistent API.
2. **Abstract Classes**: Classes that cannot be instantiated directly and are meant to be subclassed.
3. **Protocols (in Swift)**: Similar to interfaces in other languages, defining a blueprint of methods, properties, and other requirements.
4. **Mixins**: Classes that offer methods to be used by other classes without being their parent class.
5. **Annotations (in Java)**: Provide metadata about the code which can be used by the compiler or at runtime.
6. **Decorators (in Python)**: Functions or classes that modify the behavior of other functions or classes.
7. **Middlewares**: In web development, software that provides common services and capabilities to applications outside of what's offered by the operating system.
8. **Components**: In frameworks like React, Vue, and Angular, reusable UI elements.
9. **Containers**: In dependency injection, containers hold and manage instances of classes.
10. **Services**: In service-oriented architecture, services are self-contained units of functionality.
11. **Endpoints**: Specific paths in an API where requests are sent.
12. **Handlers**: Functions or methods that handle specific events or requests.
13. **ViewModels**: In MVVM architecture, they act as an intermediary between the view and the model.
14. **Contexts**: In React, a way to pass data through the component tree without having to pass props down manually at every level.
15. **Stores**: In state management libraries like Redux or MobX, stores hold the state of the application.
16. **Observers**: Objects that watch for changes in state or events and respond accordingly.
17. **Subjects**: Objects that maintain a list of observers and notify them of any state changes.
18. **Tasks/Jobs**: Units of work that can be scheduled or queued for execution.
19. **Pipelines**: Series of steps or stages where data is processed in sequence.
20. **Strategies**: Behavioral design pattern that enables selecting an algorithm's implementation at runtime.
21. **Factories**: Design pattern used to create objects without specifying the exact class of object that will be created.
22. **Proxies**: Objects that control access to another object.
23. **Facades**: Design pattern that provides a simplified interface to a complex system.
24. **Adapters**: Design pattern that allows incompatible interfaces to work together.
25. **Commands**: Encapsulate a request as an object, thereby allowing for parameterization of clients with queues, requests, and operations.
26. **Events**: Signals that something has happened in the system.
27. **Event Listeners**: Functions or methods that wait for and respond to events.
28. **Event Emitters**: Objects that produce events.
29. **Hooks**: Functions that let you “hook into” React state and lifecycle features from function components.
30. **Policies**: Guidelines or rules for accessing resources in an application.
31. **Guards**: Functions or methods used to protect certain parts of an application, such as routes in a web application.
32. **Resolvers**: Functions or methods that resolve data before a route or component is activated.
33. **Serializers**: Convert objects to a format that can be easily stored or transmitted.
34. **Deserializers**: Convert data back into objects.
35. **Aggregates**: Group of related objects that are treated as a single unit.
36. **Repositories**: Design pattern that provides an abstraction for data access, separating the application logic from data access logic.
37. **Mappers**: Objects or functions that transform data from one format to another.
38. **Validators**: Ensure data meets certain criteria before it is processed or stored.
39. **Filters**: Process data to exclude certain elements or to include only specific elements.
40. **Policies**: Enforce rules and constraints in a system, often related to security or business logic.

These elements can be used in various combinations to structure and organize code, making it more modular, maintainable, and scalable.
- Any semantic element may have its own "version control". The default behavior of the system will be using the most updated version of each semantic element for a given project. For example, if I maintain a package and offers the API A1 , any semantic element used will have its lastest version in use
by default, unless configured explicitly to use an older version. If I update my API to A1', and send this version to the users, this will be the
version used, even if there are some semantic components that are not in their lastest version. Each major version of my semantic elements will be
composed by multiple versions of each of its internal or dependent components. For example, an external API will expose, let's say,
A1 = (F1[1.3.5], F2[12.3.4], F2'[11.6.2], F3[0.0.2]). In the previous example, the API exposed has two major versions of the same semantic elements, but
with different signatures. This is exposed by the change in the first number of their version (F2 is 12 and F2' is 11, meaning they are incompatible in their
signature). Other possibility would be having a wrapper in the inner implementation. A wrapper can be created to help in terms of compatibility. If a wrapper was used,
it would have the following version: A1 = (F1[1.3.5], F2[12.3.4], F2'[w.12.3.4], F3[0.0.2]). A wrapper has the same version of its internal implementation but with
'w' preffix. An API must not have more than one semantic element with the same signature being exposed. For example, the following would not be possible:
 A1 = (F1[1.3.5], F2[12.3.4], F2'[11.6.2], F2'[w.12.3.4], F3[0.0.2]) because there are two F2'.
- Each semantic element will be in the precompiled form. In other words, to be used, it will required a pre-compilation and a compilation within a semantic marginal context.
- A semantic marginal context is nothing more than a complete graph of elements with only one of them missing, with that being the target one. With the target one, we have
a complete semantic context that generates an executable software after compiled or interpreted.
- An elementary semantic element does not depend on any other semantic element other than those that are external (external API) or standard (from the standard of the language,
for example, basic types, etc). They have the benefit of being easier to test.
- Any semantic element may have the following components: associated test cases (with table test facilities, templates, mutant testing, testing metrics, etc), default mock
implementations (if no default mock, it may use its real version), wrappers,
multiple implementations, documentation, example codes, owners (those who are responsible for most of the changes, for example; they are know as blame in github), history data (all the versioning story
with every one that was responsible for changes, responsible for test failures, all the signature changes, etc ), a list of users that are able to access it, a list of semantic elements
that reference this element, a list of semantic elements that are referenced by this element, unique signature for each major version (its type, file, name, etc), debugging stream out
(it is a facility that allows the user to easily instrument this element to generate easy debugging information. For example, it will be very easy to select a variable in this element
to generate a stream of data that can be monitored or even create templates that can be easily used in the future. For example, a template that generates an easy story telling about the
execution of this element, even a visualization. It is very important to notice that no debugging instrumentation will be allowed in the production version of this element),
logging stream out (the same as debugging, but for logging. It will be allowed to be used in production code, but it will never change the flow state of the original element), easy instrumentation
for performance tests, easy instrumentation for networking server testing, failure instrumentation, etc.
- This platform will be integrated with life scheduler operating system.
- This platform will assist the engineer in common tasks of engineering, like refactoring
- What about a structured way to review code for each semantic element? For example, a function may have a checklist of items that the reviewer can make to help him in the process.
It could have items like: signature (return type, name, parameters, etc), documentation, body, tests, example, etc and it could have the possibility to generate items or specific questions
generated by AI. It will be possible to create different checklist templates for different use cases.]
- This platform will also make it easier for us to generate resources for deliberate practice. Examples of semantic elements would be algorithms and data structure problems, reading algorithm problems,
repository exploration (it is a functionality that uses the semantic elements of a repository to generate problems, quizzes, tests, etc. For example, it can create a reading code practice
problem with pieces of code from a semantic element with the help of the documentation and AI tools. It can create a coding problem from a semantic element using its tests as feedback and providing
the user with a prompt to write the code that passes the test. )
- The platform will have an easy TODO task system that will be easily integrated with life scheduler, allowing easy creation of tasks for the future without losing focus on the current task.
- Add possibility of template or section-like organization within a text source of an element. For example, suppose that I have a large source code for a function, I will be able to create
visual sections that can help me organize the content. They will not change the semantics of the element, but they will help organize it viasually.
- Add seamless integration with the archaic way of programming: source code. In other words,
one is able to integrate this system little by little.
- Another possible feature is adding testing checkpoints in code. For example, suppose that I have a function which I do not want to change its code or refactor it. But at the same time I want to test a very specific part of this function. I can add two checkpoints (start and end) to simulate the execution of only that part. I only need to provide the mocked input for start and the expected for the end. I may also add intermediate points with intermediate checks.
