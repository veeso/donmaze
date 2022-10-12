# Contributing

Before contributing to this repository, please first discuss the change you wish to make via issue of this repository before making a change.
Please note we have a [code of conduct](CODE_OF_CONDUCT.md), please follow it in all your interactions with the project.

- [Contributing](#contributing)
  - [Open an issue](#open-an-issue)
    - [Questions](#questions)
    - [Bug reports](#bug-reports)
    - [Copy error](#copy-error)
    - [Feature requests](#feature-requests)
  - [Pull Request Process](#pull-request-process)
    - [Software guidelines](#software-guidelines)
  - [Developer contributions guide](#developer-contributions-guide)

---

## Open an issue

Open an issue when:

- You have questions or concerns regarding the project or the application itself.
- You have a bug to report.
- You have a copy error to report. (Error in documentation; bad translation or sentence)
- You have a feature or a suggestion to improve donmaze to submit.

### Questions

If you have a question open an issue using the `Question` template.
By default your question should already be labeled with the `question` label, if you need help with your installation, please also add the `help wanted` label.
Check the issue is always assigned to `veeso`.

### Bug reports

If you want to report an issue or a bug you've encountered while using donmaze, open an issue using the `Bug report` template.
The `Bug` label should already be set and the issue should already be assigned to `veeso`.
Don't set other labels to your issue, not even priority.

When you open a bug try to be the most precise as possible in describing your issue. I'm not saying you should always be that precise, since sometimes it's very easy for maintainers to understand what you're talking about. Just try to be reasonable to understand sometimes we might not know what you're talking about or we just don't have the technical knowledge you might think.
Please always provide the environment you're working on and consider that we don't provide any support for older version of donmaze, at least for those not classified as LTS (if we'll ever have them).
If you can, provide the log file or the snippet involving your issue. You can find in the [user manual](docs/man.md) the location of the log file. Please, if you can, enable the **debug mode**, before submitting the log, in order to provide us with a better overview of the problem.
Last but not least: the template I've written must be used. Full stop.

Maintainers will may add additional labels to your issue:

- **duplicate**: the issue is duplicated; the reference to the related issue will be added to your description. Your issue will be closed.
- **priority**: this must be fixed asap
- **sorcery**: it is not possible to find out what's causing your bug, nor is reproducible on our test environments.
- **wontfix**: your bug has a very high ratio between the difficulty to fix it and the probability to encounter it, or it just isn't a bug, but a feature.

### Copy error

If you want to report a copy error, create an issue using the `copy` template.
The `Documentation` label should already be set and the issue should already be assigned to `veeso`.

If you want to fix the copy by yourself you can fork the project and open a PR, otherwise I will fix it by myself.
The copy issue is accepted **also if you're not a C1/C2 speaker**, but a speaker of that level in case the language is different from Italian/English is preferred.
Please fullfil the form on the bottom of the template if you want.

### Feature requests

Whenever you have a good idea which chould improve the project, it is a good idea to submit it to the project owner.
The first thing you should do though, is not starting to write the code, but is to become concern about how donmaze works, what kind
of contribution I appreciate and what kind of contribution I won't consider.
Said so, follow these steps:

- Read the contributing guidelines, entirely
- Think on whether your idea would fit in the project mission and guidelines or not
- Think about the impact your idea would have on the project
- Open an issue using the `feature request` template describing with accuracy your suggestion
- Wait for the maintainer feedback on your idea

It is very important to follow these steps, since it will prevent you from working on a feature that will be rejected and trust me, none of us wants to deal with this situation.

Always mind that your suggestion, may be rejected: I'll always provide a feedback on the reasons that brought me to reject your feature, just try not to get mad about that.

---

## Pull Request Process

Let's make it simple and clear:

1. Open a PR with an **appropriate label** (e.g. bug, enhancement, ...).
2. Write a **properly documentation** for your software compliant with **rustdoc** standard.
3. Write tests for your code. This doesn't apply necessarily for implementation regarding the user-interface module (`ui/activities`) and (if a test server is not available) for file transfers.
4. Check your code with `cargo clippy`.
5. Check if the CI for your commits reports three-green.
6. Report changes to the PR you opened, writing a report of what you changed and what you have introduced.
7. Update the `CHANGELOG.md` file with details of changes to the application. In changelog report changes under a chapter called `PR{PULL_REQUEST_NUMBER}` (e.g. PR12).
8. Assign a maintainer to the reviewers.
9. Wait for a maintainer to fullfil the acceptance tests
10. Wait for a maintainer to complete the acceptance tests
11. Request maintainers to merge your changes.

### Software guidelines

In addition to the process described for the PRs, I've also decided to introduce a list of guidelines to follow when writing the code, that should be followed:

1. **Let's stop the NPM apocalypse**: personally I'm against the abuse of dependencies we make in software projects and I think that NodeJS has opened the way to this drama (and has already gone too far). Nowadays nobody cares about adding hundreds of dependencies to their projects. Don't misunderstand me: I think that package managers are cool, but I'm totally against the abuse we're making of them. I think when we work on a project, we should try to use the minor quantity of dependencies as possible, especially because it's not hard to see how many libraries are getting abandoned right now, causing compatibility issues after a while. So please, when working on donmaze, try not to add useless dependencies.
2. **No C-bindings**: personally I think that Rust still relies too much on C. And that's bad, really bad. Many libraries in Rust are just wrappers to C libraries, which is a huge problem, especially considering this is a multiplatform project. Everytime you add a C-binding to your project, you're forcing your users to install additional libraries to their systems. Sometimes these libraries are already installed on their systems (as happens for libssh2 or openssl in this case), but sometimes not. So if you really have to add a dependency to this project, please AVOID completely adding C-bounded libraries.
3. **Test units matter**: Whenever you implement something new to this project, always implement test units which cover the most cases as possible.
4. **Comments are useful**: Many people say that the code should be that simple to talk by itself about what it does, and comments should then be useless. I personally don't agree. I'm not saying they're wrong, but I'm just saying that this approach has, in my personal opinion, many aspects which are underrated:
   1. What's obvious for me, might not be for the others.
   2. Our capacity to work on a code depends mostly on **time and experience**, not on complexity: I'm not denying complexity matter, but the most decisive factor when working on code is the experience we've acquired working on it and the time we've spent. As the author of the project, I know the project like the back of my hands, but if I didn't work on it for a year, then I would probably have some problems in working on it again as the same speed as before. And do you know what's really time-saving in these cases? Comments.

## Developer contributions guide

You can view the developer guide [here](docs/developer.md).

---

Thank you for any contribution!  
Christian Visintin
