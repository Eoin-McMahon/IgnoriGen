# Requirements for IgnoriGen CLI tool

* Must be able to pass a langauge as a command line argument
* Must be able to pass multiple languages and have each of their gitignores appended to one another
* Must store the gitignore in the running directory or a specified directory through a --destination argument 
* Must have a help flag which lists all available templates
* Must show suggestions if an invalid language was passed (i.e Did you mean "flutter"?)
* Must be symlinked so it can be used in any directory

* Should have a caching option to save templates.

example

```bash
$ ignorigen flutter dart
$ ignorigen --lang flutter dart --dest ../src/
```