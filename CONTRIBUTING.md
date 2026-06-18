# Contributing to Aurora

Thanks for deciding to contribute to Aurora
These are the following guidelines for contributing to Aurora and it's docs, tools, website. You can feel free to purpose changes in a pull request!

#### Table of Contents

* [Code of Conduct](#code-of-conduct)
* [Acknowledgements](#acknowledgements)
* [What Should I Know Before I Get Started?](#what-should-i-know-before-i-get-started)

  * [Design Decisions](#design-decisions)
* [How Can I Contribute?](#how-can-i-contribute)

  * [Reporting Bugs](#reporting-bugs)
  * [Suggesting Enhancements](#suggesting-enhancements)
  * [Your First Code Contribution](#your-first-code-contribution)
  * [Pull Requests](#pull-requests)
* [Style Guides](#style-guides)

  * [Git Commit Messages](#git-commit-messages)
  * [Rust Style Guide](#rust-style-guide)
* [Additional Notes](#additional-notes)

  * [Issue and Pull Request Labels](#issue-and-pull-request-labels)
  * [Type Labels](#type-labels)
  * [Contribution Labels](#contribution-labels)
  * [Status Labels](#status-labels)
  * [Area Labels](#area-labels)
  * [Pull Request Labels](#pull-request-labels)
* [Acknowledgements](#acknowledgements)

## Code of Conduct

This project and everyone participating in it is governed by the [Aurora Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to [ahum.codes@gmail.com](mailto:ahum.codes@gmail.com).

## What should I know before I get started?
You first need to have familiarity with Aurora and it's goals, Rust, Hyprland, Lua. If you are considering to improve our GUIs you also need to have a familiarity with `GTK4-rs`


### Design Decisions
For improving Rust code, please follow [`Rust Style Guide`](https://doc.rust-lang.org/style-guide/)

## How Can I Contribute?

### Reporting Bugs

This section guides you through submitting a bug report for Aurora. Following these guidelines helps maintainers and the community understand your report :pencil:, reproduce the behavior :computer: :computer:, and find related reports :mag_right:.

Before creating bug reports, please check [this list](#before-submitting-a-bug-report) as you might find out that you don't need to create one. When you are creating a bug report, please [include as many details as possible](#how-do-i-submit-a-good-bug-report). Fill out [the required template](https://github.com/TheAhumMaitra/Aurora/.github/blob/master/.github/ISSUE_TEMPLATE/bug_report.md), the information it asks for helps us resolve issues faster.

> **Note:** If you find a **Closed** issue that seems like it is the same thing that you're experiencing, open a new issue and include a link to the original issue in the body of your new one.

#### Before Submitting A Bug Report
* **Check the [existing issues](https://github.com/TheAhumMaitra/Aurora/issues) and the [discussions](https://github.com/TheAhumMaitra/Aurora/discussions)** for a list of common questions and problems.

#### How Do I Submit A (Good) Bug Report?

Bugs are tracked as [GitHub issues](https://guides.github.com/features/issues/). Create an issue on that repository and provide the following information by filling in [the template](https://github.com/TheAhumMaitra/Aurora/.github/blob/master/.github/ISSUE_TEMPLATE/bug_report.md).

Explain the problem and include additional details to help maintainers reproduce the problem:

* **Use a clear and descriptive title** for the issue to identify the problem.
* **Describe the exact steps which reproduce the problem** in as many details as possible. For example, start by explaining how you started Aurora, e.g. which command exactly you used in the terminal, or how you started Aurora otherwise. When listing steps, **don't just say what you did, but explain how you did it**. For example, if you moved the cursor to the end of a line, explain if you used the mouse, or a keyboard shortcut or an Aurora command, and if so which one?
* **Provide specific examples to demonstrate the steps**. Include links to files or GitHub projects, or copy/pasteable snippets, which you use in those examples. If you're providing snippets in the issue, use [Markdown code blocks](https://help.github.com/articles/markdown-basics/#multiple-lines).
* **Describe the behavior you observed after following the steps** and point out what exactly is the problem with that behavior.
* **Explain which behavior you expected to see instead and why.**
* **Include screenshots, videos and animated GIFs** which show you following the described steps and clearly demonstrate the problem. If you use the keyboard while following the steps. You can use [this tool](https://www.cockos.com/licecap/) to record GIFs on macOS and Windows, and [this tool](https://github.com/colinkeenan/silentcast) or [this tool](https://github.com/GNOME/byzanz) on Linux.
* **If you're reporting that Aurora crashed**, please include logs and what specifically you did and how it crashed. IT might be a package issue or Hyprland issues
* **If the problem is related to performance or memory**, Include your system details
* **If the problem wasn't triggered by a specific action**, describe what you were doing before the problem happened and share more information using the guidelines below.

Provide more context by answering these questions:

* **Did the problem start happening recently** (e.g. after updating to a new version of Aurora) or was this always a problem?
* If the problem started happening recently, **can you reproduce the problem in an older version of Aurora?** What's the most recent version in which the problem doesn't happen? You can download older versions of Aurora from [the releases page](https://github.com/TheAhumMaitra/Aurora/releases).
* **Can you reliably reproduce the issue?** If not, provide details about how often the problem happens and under which conditions it normally happens.
* If the problem is related to working with files (e.g. opening and editing files), **does the problem happen for all files and projects or only some?** Does the problem happen only when working with local or remote files (e.g. on network drives), with files of a specific type (e.g. only JavaScript or Python files), with large files or files with very long lines, or with files in a specific encoding? Is there anything else special about the files you are using?

Include details about your configuration and environment:

* **Which version of Auora are you using?** You can get the exact version by running `aurora version` in your terminal
* **What's the name and version of the OS you're using**?
* **Are you running Aurora and Hyprland in a virtual machine?** If so, which VM software are you using and which operating systems and versions are used for the host and the guest?
* **Are you using all necessary and recommended packages** You can see [`all recomanded pacakges and nesseary pacakges in docs`](https://aurorawiki.vercel.app/docs/getting-started/installation/#recommended-packages)
* **Which keyboard layout and locale are you using?** Are you using a US layout and locale or some other layout, locale?

### Suggesting Enhancements

This section guides you through submitting an enhancement suggestion for Aurora, including completely new features and minor improvements to existing functionality. Following these guidelines helps maintainers and the community understand your suggestion :pencil: and find related suggestions :mag_right:.

Before creating enhancement suggestions, please check [this list](#before-submitting-an-enhancement-suggestion) as you might find out that you don't need to create one. When you are creating an enhancement suggestion, please [include as many details as possible](#how-do-i-submit-a-good-enhancement-suggestion). Fill in [the template](https://github.com/TheAhumMaitra/Aurora/blob/master/.github/ISSUE_TEMPLATE/feature_request.md), including the steps that you imagine you would take if the feature you're requesting existed.

#### Before Submitting An Enhancement Suggestion
* **Check if you're using the latest version of Aurora** You can check via running `aurora version`
* **Check if there's already a pull request about this**

#### How Do I Submit A (Good) Enhancement Suggestion?

Enhancement suggestions are tracked as [GitHub issues](https://guides.github.com/features/issues/). Determine your enhancement suggestion is related to, create an issue on that repository and provide the following information:

* **Use a clear and descriptive title** for the issue to identify the suggestion.
* **Provide a step-by-step description of the suggested enhancement** in as many details as possible.
* **Provide specific examples to demonstrate the steps**. Include copy/pasteable snippets which you use in those examples, as [Markdown code blocks](https://help.github.com/articles/markdown-basics/#multiple-lines).
* **Describe the current behavior** and **explain which behavior you expected to see instead** and why.
* **Include screenshots and animated GIFs** which help you demonstrate the steps or point out the part of Aurora which the suggestion is related to. You can use [this tool](https://www.cockos.com/licecap/) to record GIFs on macOS and Windows, and [this tool](https://github.com/colinkeenan/silentcast) or [this tool](https://github.com/GNOME/byzanz) on Linux.
* **Explain why this enhancement would be useful to most Aurora users**
* **Specify which version of Aurora you're using.** You can get the exact version by running `aurora version` in your terminal
* **Specify the name and version of the OS you're using.**

### Your First Code Contribution

Unsure where to begin contributing to Aurora? You can start by looking through these `beginner` and `help-wanted` labeled issues:

* [Beginner issues][beginner] - issues which should only require a few lines of code, and a test or two.
* [Help wanted issues][help-wanted] - issues which should be a bit more involved than `beginner` issues.

Both issue lists are sorted by total number of comments. While not perfect, number of comments is a reasonable proxy for impact a given change will have. 

### Pull Requests

The process described here has several goals:

- Maintain Aurora's quality
- Fix problems that are important to users
- Engage the community in working toward the best possible Aurora
- Enable a sustainable system for Aurora's maintainers to review contributions

Please follow these steps to have your contribution considered by the maintainers:

1. Follow all instructions in [the template](PULL_REQUEST_TEMPLATE.md)
2. Follow the [styleguides](#styleguides)
3. Make sure you have added tests if needed

While the prerequisites above must be satisfied prior to having your pull request reviewed, the reviewer(s) may ask you to complete additional design work, tests, or other changes before your pull request can be ultimately accepted.

## Styleguides

### Git Commit Messages

* Use the present tense ("Add feature" not "Added feature")
* Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
* Limit the first line to 72 characters or less
* Reference issues and pull requests liberally after the first line
* When only changing documentation, include `[docs-only]` in the commit title
* Consider starting the commit message with an applicable name:
    * :pkg(add): when adding a pacakge 
    * :pkg(rem): when removing a package
    * :pkg(up-all): when upgrading all packages
    * :pkg(up): when upgrading one pacakge
    * :pkg(ups): when upgrading many pacakges
    * :yipee: when you are adding changes in CONTRIBUTING.md
    * :sys_imv:  when improving performance
    * :water:  when plugging memory leaks (Mostly for code written in Lua)
    * :readme: when improving readme
    * :fix: when fixing a bug
    * :fire: when removing code or files, packages that are not needed
    * :cli: when fixing the CLI or adding features in CLI
    * :kawaii: when adding tests
    * :feat: when adding new features

### Rust Styleguide
Please follow [`Rust styleguide`](https://doc.rust-lang.org/style-guide/)

## Additional Notes

### AI Tool Policy

Reviewing a pull request takes a lot of time, but utilizing AI techniques to create an illogical but convincing-looking one is quite simple. These guidelines are in place because it is unjust for other contributors and reviewers to have to spend so much time on this:

1. You, not the AI, are in charge of reviewing and testing all LLM-generated content before submitting it.
2. Except for translations, avoid using AI to react to review comments.

If you don't, we'll close the Pull Request with an `ai-slop` label.

### Issue and Pull Request Labels

Aurora uses labels to organize issues, feature requests, and pull requests. These labels help contributors quickly understand the status, priority, and purpose of an issue or PR.

### Type Labels

| Label | Description |
|---------|-------------|
| `bug` | Confirmed bug or unexpected behavior. |
| `enhancement` | Improvement to an existing feature without major functionality changes. |
| `feature-request` | Proposal for a new feature. |
| `documentation` | Documentation-related work. |
| `question` | General questions or support requests. |
| `discussion` | Open discussion or design conversation. |

### Contribution Labels

| Label | Description |
|---------|-------------|
| `good-first-issue` | Suitable for first-time contributors. |
| `help-wanted` | Community contributions are welcome. |
| `beginner-friendly` | Requires minimal project knowledge. |

### Status Labels

| Label | Description |
|---------|-------------|
| `needs-information` | More information is required before work can continue. |
| `needs-reproduction` | A reported issue that still needs to be reproduced. |
| `blocked` | Waiting on another issue, dependency, or decision. |
| `duplicate` | Already reported elsewhere. |
| `invalid` | Not considered a valid issue. |
| `wontfix` | Will not be addressed at this time. |
| `ai-slop` | AI-generated content without testing, not followed [`AI Tool Policy`](#ai-tool-policy) |

### Area Labels

| Label | Description |
|---------|-------------|
| `rust` | Rust-related code. |
| `hyprland` | Hyprland configuration or integration. |
| `lua` | Lua scripts and modules. |
| `gtk4` | GTK4 user interface components. |
| `cli` | Command-line interface functionality. |
| `installer` | Installation and setup experience. |
| `performance` | Performance-related improvements. |
| `security` | Security-related reports or improvements. |

### Priority Labels

| Label | Description |
|---------|-------------|
| `priority:critical` | Requires immediate attention. |
| `priority:high` | Important issue affecting many users. |
| `priority:medium` | Standard priority work. |
| `priority:low` | Nice-to-have improvements. |

### Release Labels

| Label | Description |
|---------|-------------|
| `breaking-change` | Introduces breaking changes. |
| `release-blocker` | Must be resolved before the next release. |

### Pull Request Labels

| Label | Description |
|---------|-------------|
| `work-in-progress` | Still under active development. |
| `needs-review` | Ready for maintainer review. |
| `under-review` | Currently being reviewed. |
| `requires-changes` | Additional changes are required before merging. |
| `needs-testing` | Requires manual testing. |
| `ready-to-merge` | Approved and ready for merge. |

## Acknowledgements

This document is substantially adapted from the Atom project's CONTRIBUTING.md.

Copyright (c) GitHub Inc. and Atom contributors.

The original document was distributed under the MIT License.
