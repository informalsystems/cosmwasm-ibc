# Contribution guidelines

Thank you for your interest in contributing to cosmwasm-ibc!🎉

The rest of this document outlines the best practices for contributing to this
repository:

- [Decision Making](#decision-making) - process for agreeing to changes
- [Issues](#issues) - what makes a good issue
- [Pull Requests](#pull-requests) - what makes a good pull request
- [Forking](#forking) - fork the repo to make pull requests
- [Changelog](#changelog) - changes must be recorded in the changelog
- [Releases](#releases) - how to release new version of the crates

## Decision Making

When contributing to the project, the following process leads to the best chance
of landing the changes in `main`.

All new contributions should start with a GitHub issue which captures the
problem you're trying to solve. Starting off with an issue allows for early
feedback.

When the problem and the proposed solution are well understood, changes should
start with a [draft pull
request](https://github.blog/2019-02-14-introducing-draft-pull-requests/)
against the branch `main`. The draft status signals that work is underway. When
the work is ready for feedback, hitting "Ready for Review" will signal to the
maintainers to take a look.

Implementation trajectories should aim to proceed where possible as a series of
smaller incremental changes, in the form of small PRs that can be merged
quickly. This helps manage the load for reviewers and reduces the likelihood
that PRs will sit open for long periods of time.

## Issues

We welcome bug reports, feature requests, and other contributions to our
project. To open an issue, please follow these guidelines:

1. **Search existing issues**: Before opening a new issue, please search
   existing issues to ensure that is not a duplicates.

2. **Provide a clear and descriptive title**: This helps others understand the
   nature of the issue at a glance.

3. **Provide detailed information**: In the issue description, clearly state the
   **purpose** of the issue include as much information as possible, such as:
    - Steps to reproduce the issue
    - Expected behavior
    - Actual behavior
    - The version of the operating system and the software you are using
    - Error messages or logs (if applicable)

This assist us prioritize and categorize your issue more effectively and help
others and reviewers understand the type and severity of the issue.

## Pull Requests

If you have write access to the cosmwasm-ibc repo, you can directly branch off
of `main`. This makes it easier for project maintainers to directly make changes
to your branch should the need arise. Otherwise, check the [Forking](#forking)
section for instructions.

Branch names should be prefixed with the author's name followed by a short
description of the feature, eg. `name/feature-x`.

Pull requests are made against `main` and are squash-merged into it. Each PR should:

- make reference to an issue outlining the context
- update any relevant documentation and include tests
- add a corresponding entry in the `.changelog` directory using `unclog`,
  see the [Changelog](#changelog) section for more details.

Additionally, in order to make PRs as easy to review as possible, each PR should:

- Be focused on implementing _*one*_ piece of logic from end-to-end. It must be
  very clear what the purpose of the PR is from looking at the PR's title,
  description, and/or linked issue(s). It should also be very clear what value
  the changes incorporated in the PR aim to deliver. A single PR that does
  multiple things, without a clear articulation of the problem it attempts to
  solve, will very likely be rejected.
- Be small, ideally no more than 500 lines of code changes. While this is a
  guideline and not a hard rule, in general, larger changes should being
  structured as a series of PRs, each building off of the previous ones; these
  PRs should also be tracked in a tracking issue. If a single PR absolutely has
  to be larger, it _must_ be structured such that it can be reviewed
  commit-by-commit, with each commit doing a single logical thing, accompanied
  with a good description of what it aims to achieve in the git commit message.
  Poorly structured PRs will likely be rejected on the grounds of being too much
  of a burden for the core maintainers to review; you will be asked to
  restructure the PR in accordance with the guidelines laid out here. This does
  not necessarily apply to documentation-related changes or automatically
  generated code (e.g. generated from Protobuf definitions). But automatically
  generated code changes should occur within separate commits, so they are
  easily distinguishable from manual code changes.

## Responsibilities of a PR Reviewer

If you're tagged as the reviewer of a PR, you are responsible for shepherding it
through to completion. This includes fixing issues with the PR and taking the
lead on decisions that need to be resolved in order to get the PR merged.

If you're tagged as a reviewer on a PR that affects a part of the code base that
you are unfamiliar with, you can hand it off to someone (with their consent)
who is more appropriate to shepherd the PR through to completion.

## Forking

If you do not have write access to the repository, your contribution should be
made through a fork on GitHub. Fork the repository, contribute to your fork
(either in the `main` branch of the fork or in a separate branch), and then
make a pull request back upstream.

When forking, add your fork's URL as a new git remote in your local copy of the
repo. For instance, to create a fork and work on a branch of it:

- Create the fork on GitHub, using the fork button.
- `cd` to the original clone of the repo on your machine
- `git remote rename origin upstream`
- `git remote add origin git@github.com:<location of fork>`

Now `origin` refers to your fork and `upstream` refers to the original version.
Now `git push -u origin main` to update the fork, and make pull requests against
the original repo.

To pull in updates from the origin repo, run `git fetch upstream` followed by
`git rebase upstream/main` (or whatever branch you're working in).

## Changelog

Every non-trivial PR must update the [CHANGELOG](CHANGELOG.md). This is
accomplished indirectly by adding entries to the `.changelog` folder in
[`unclog`](https://github.com/informalsystems/unclog) format using the `unclog`
CLI tool. `CHANGELOG.md` will be built by whomever is responsible for performing
a release just prior to release - this is to avoid changelog conflicts prior to
releases.

### Install `unclog`

```bash
cargo install unclog
```

### Examples

Add a `.changelog` entry to signal that a bug was fixed, without mentioning any component.

```bash
unclog add -i update-unclog-instructions -s bug-fixes -n 1634 -m "Update CONTRIBUTING.md for latest version of unclog" --editor vim
```

Add a .changelog entry under the `FEATURES` section in CHANGELOG.md.

```bash
unclog add -s features --id a-new-feature --issue-no 1235 -m "msg about this new-feature" --editor vim
```

### Preview unreleased changes

```bash
unclog build -u
```

The Changelog is _*not*_ a record of what Pull Requests were merged; the commit
history already shows that. The Changelog is a notice to users about how their
expectations of the software should be modified. It is part of the UX of a
release and is a _*critical*_ user facing integration point. The Changelog must
be clean, inviting, and readable, with concise, meaningful entries. Entries must
be semantically meaningful to users. If a change takes multiple Pull Requests to
complete, it should likely have only a single entry in the Changelog describing
the net effect to the user. Instead of linking PRs directly, we instead prefer
to log issues, which tend to be higher-level, hence more relevant for users.

When writing Changelog entries, ensure they are targeting users of the software,
not fellow developers. Developers have much more context and care about more
things than users do. Changelogs are for users.

Changelog structure is modeled after [Tendermint
Core](https://github.com/tendermint/tendermint/blob/master/CHANGELOG.md) and
[Hashicorp Consul](http://github.com/hashicorp/consul/tree/master/CHANGELOG.md).
See those changelogs for examples.

We currently split changes for a given release between these four sections:
Breaking Changes, Features, Improvements, and Bug Fixes.

Entries in the changelog should initially be logged in the _Unreleased_ section,
which represents a "staging area" for accumulating all the changes throughout a
release (see [Pull Requests](#pull-requests) below). With each release, the
entries then move from this section into their permanent place under a specific
release number in Changelog.

Changelog entries should be formatted as follows:

```md
- Some description about the change ([#xxx](https://github.com/informalsystems/cosmwasm-ibc/issues/xxx)) (optional @contributor)
```

Here `xxx` is the issue number, and `contributor` is the author/s of the change.

It's also acceptable for `xxx` to refer to the relevant pull request, but issue
numbers are preferred. Note this means issues (or pull-requests) should be
opened first so the changelog can then be updated with the corresponding number.

Changelog entries should be ordered alphabetically numerically according to
their issue/PR number.

Changes with multiple classifications should be doubly included (eg. a bug fix
that is also a breaking change should be recorded under both).

Breaking changes are further subdivided according to the APIs/users they impact.
Any change that effects multiple APIs/users should be recorded multiply - for
instance, a change to some core protocol data structure might need to be
reflected both as breaking the core protocol but also breaking any APIs where
core data structures are exposed.

## Releases

Our release process is as follows:

1. Bump the version in `Cargo.toml`.
2. Update the [changelog](#changelog) to reflect and summarize all changes in
   the release. This involves:
   1. Running `unclog build -u` and copy pasting the output at the top
      of the `CHANGELOG.md` file, making sure to update the header with
      the new version.
   2. Running `unclog release vX.Y.Z --editor <editor>` to create a
      summary of all of the changes in this release.
   3. Committing the updated `CHANGELOG.md` file and `.changelog` directory to the repo.
3. Push this to a branch `release/vX.Y.Z` according to the version number of
   the anticipated release (e.g. `release/v0.18.0`) and open a **draft PR**.
4. Run `cargo doc --all-features --open` locally to double-check that all the
   documentation compiles and is up-to-date and coherent. Fix any potential
   issues here and push them to the release PR.
5. Mark the PR as **Ready for Review** and incorporate feedback on the release.
6. Once approved, merge the PR.
7. Checkout the `main` and pull it with `git checkout main && git pull origin main`.
   Then create a signed tag and push it to GitHub: `git tag -s -a vX.Y.Z && git push origin vX.Y.Z`
   In the tag message, write the version and the link to the corresponding section of the changelog.
8. If any problem arises, submit a new PR, get it merged to `main` and try again.
   The reason for not releasing straight from the release branch, and therefore losing the
   ability to fix publishing-related problems as they arise, is that we would like the embedded
   metadata of the published crates, namely the Git commit at which the release was done,
   to match the Git commit on the `main` branch which will be tagged.
   [See this article][crates.io-security] for a more in-depth explanation.
   **Note:** This step requires the appropriate privileges to push crates to [crates.io].
9. Once the tag is pushed, wait for the CI bot to create a GitHub release, then
    update the release description to

    ```md
    [📖 CHANGELOG](https://github.com/informalsystems/cosmwasm-ibc/blob/master/CHANGELOG.md#vXYZ)`
    ```

All done! 🎉

[crates.io]: https://crates.io
[crates.io-security]: https://codeandbitters.com/published-crate-analysis/
