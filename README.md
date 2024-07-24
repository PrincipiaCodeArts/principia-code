# software-cad
Implementation of the software cad, a tool that will make software engineering possible.


## Draft
- I want a tool that helps us to give a new and useful definition for software engineering. Currently, we do not have a good definition.
- This project will help us applying metrics to the software engineering process.
If we create an environment that integrates everything related with the software engineering process, it will be easier to generate enough
data to reason about a good new definition for software engineering. With that, we will be able to create a better software engineering practice.
First, we need data to reason about and a good environment to create experiments that will help us defining the new paths to a good engineering.
- This platform will help anyone developing high quality software. We are going to investigate the better metrics and practices to reach a good
engineering process.
### Characteristics of this platform
- The versioning will be done by each element, not by repository.
- This system will allow automatic "commit" and "pull request" given that each change will be done in a specific semantic element of the software.
- Semantic elements are: functions, classes, interfaces, methods, attributes, files, modules (composed by files, exposing external API that allows other modules to
communicate with them), architectural components (composed by modules, and exposing an external API that allows other architectural components to
communicate with it), global variables, enums, types, structs, traits (in rust), databases, tables, schemas, files, external APIs (for example, if we
do not have access to the repository of this API, we can model only the API signatures and mock it if necessary), code templates (pieces of code that
allow only editting of a part of the code, the common parts may be edited only for all the instances of the template), architectural components (queues,
APIs, databases, load balancers, security services, caching servers, servers, logging elements, etc) etc.
More suggestions from ChatGPT:
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
- Any semantic element will have its own version control. The default behavior of the system will be using the most updated version of each semantic
element for a given project. For example, if I maintain a package and offers the API A1, any semantic element used will have its lastest version in use
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
multiple implementations, documentation, example codes, owners (those who are responsible for most of the changes, for example), history data (all the versioning story
with every one that was responsible for changes, responsible for test failures, all the signature changes, etc ), a list of users that are able to access it, a list of semantic elements
that reference this element, a list of semantic elements that are referenced by this element, unique signature for each major version (its type, file, name, etc), debugging stream out
(it is a facility that allows the user to easily instrument this element to generate easy debugging information. For example, it will be very easy to select a variable in this element
to generate a stream of data that can be monitored or even create templates that can be easily used in the future. For example, a template that generates an easy story telling about the
execution of this element, even a visualization. It is very important to notice that no debugging instrumentation will be allowed in the production version of this element),
logging stream out (the same as debugging, but for logging. It will be allowed to be used in production code, but it will never change the flow state of the original element), easy instrumentation
for performance tests, easy instrumentation for networking server testing, failure instrumentation, etc.
   
