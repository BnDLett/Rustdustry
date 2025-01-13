# Contributing
These are the rules that I have in place for contributing. You won't get flamed for not following them to the "t";
however, it is highly preferred that you abide by them as closely as possible. If you don't follow these rules, then
your pull request is less likely to be merged.

## Rules
1. Keep it simple. Developing for this doesn't need to be overly complex. Both in terms of adding new game objects
(such as adding new units/blocks) or just modifying the behavior of the base system itself.

2. Use a multi-paradigm approach. You don't need to stick to one specific paradigm all the time. Doing so is, in
essence, just writing bad code. Also, ensure that you're using the appropriate paradigm(s).

3. Composition over inheritance. Rust doesn't directly support inheritance, so you should use composition when possible.
If you're unsure as to what composition is, then refer to [this](https://stackify.com/oop-concepts-composition/).

4. Open an issue before you make a significant change. Significant changes (such as changing what a function's 
arguments) can be EXTREMELY dangerous.

5. Ensure that you appropriately bump the version number based on [Semantic Versioning 2.0.0](https://semver.org/). <br>
**note**: Since the major version is currently `0`, changes are fair game. The major version will be bumped once I, or
a majority of the maintainers of the project, decides that the project is ready for production.

6. Be descriptive in your extended commit messages. If you're using a jetbrains IDE, then you can use
[this plugin](https://plugins.jetbrains.com/plugin/20935-git-commit-message-format) to help with extended commit
messages.
