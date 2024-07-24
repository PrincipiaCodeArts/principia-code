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
- Semantic elements are: functions, classes, methods, attributes, files, modules (composed by files, exposing external API that allows other modules to
communicate with them), architectural components (composed by modules, and exposing an external API that allows other architectural components to
communicate with it), global variables, enums, types, structs, traits (in rust), etc.
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
- 
