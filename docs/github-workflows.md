# GitHub Workflows

A guide to the four workflows used in the `test-repo` learning exercise, with the terminology for each.

- **Repository:** `https://github.com/data-probe-1/test-repo`
- **Owner / maintainer:** `data-probe-1`
- **Contributor:** `rohingosling` (on a second computer)

## Overview

`rohingosling` has no write access to `data-probe-1/test-repo`. That makes this the classic **open-source contribution model**, also called the **fork and pull request workflow**.

The other option is to add `rohingosling` as a **collaborator**, who then works on **branches** directly in the repo. The fork workflow is the better one to learn first, because it covers more of the important ideas.

```
 data-probe-1/test-repo  ──── Fork ────►  rohingosling/test-repo
        ▲                                         │
        │                                       Clone
   Pull Request                                   ▼
        │                                  Local repository
        └──────── Push branch ◄──────── (feature branch commits)
```

## Workflow 1: Create and Push the Project (`data-probe-1`)

1. **Create the remote repository** on GitHub. This is already done: `test-repo`.
2. **Initialize a local repository** with `git init`. With Rust, `cargo new` does this for you.
3. **Stage** the files. This puts them in the **staging area**, also called the **index**.
4. **Commit** the staged files. A commit is a snapshot with a **commit message** and a **SHA hash**.
5. **Add a remote** named `origin`. By convention, `origin` is the name for the main remote.
6. **Push** to GitHub. The `-u` flag sets up **upstream tracking** between local `main` and `origin/main`.

```bash
cargo new hello-world
cd hello-world
git add .
git commit -m "Initial commit: Hello World in Rust"
git remote add origin https://github.com/data-probe-1/test-repo.git
git push -u origin main
```

**Terms:** repository (repo), local vs remote, working tree, staging area, commit, branch (`main` is the **default branch**), remote, `origin`, push, upstream.

## Workflow 2: Contribute Changes (`rohingosling`, other computer)

1. **Fork:** on GitHub, `rohingosling` clicks **Fork**. This creates `rohingosling/test-repo`, a server-side copy they own.
2. **Clone** the fork to the other computer. The fork becomes that computer's `origin`.
3. **Add the original repo as `upstream`**.
4. **Create a feature branch.** Never work directly on `main`.
5. **Edit, stage and commit** as in Workflow 1.
6. **Push the branch to the fork.**
7. **Open a pull request (PR)** on GitHub. Pick `rohingosling:add-greeting` as the **head (compare) branch** and `data-probe-1:main` as the **base branch**. Add a title and a description.

```bash
git clone https://github.com/rohingosling/test-repo.git
cd test-repo
git remote add upstream https://github.com/data-probe-1/test-repo.git
git checkout -b add-greeting

# ... edit files ...

git add .
git commit -m "Add personalised greeting"
git push -u origin add-greeting
```

> "Sending it back" isn't a push to the original repo. It's a **request** for the owner to **pull** (merge) your branch, which is where the name "pull request" comes from.

**Terms:** fork, clone, `upstream` vs `origin`, feature branch, pull request, base and head branches, contributor.

## Workflow 3: Review and Accept (`data-probe-1`)

1. Open the PR's **Files changed** tab and look at the **diff**.
2. Optionally test it locally with `gh pr checkout <number>`.
3. Leave **review comments**. These can be inline on specific lines, and **suggested changes** can be applied with one click.
4. **Submit the review** with one of three verdicts:
   - **Comment**: general feedback only.
   - **Approve**: ready to merge.
   - **Request changes**: blocks the PR until it's fixed.
5. **Merge the PR**, choosing one of three **merge strategies**:
   - **Create a merge commit**: keeps the full history.
   - **Squash and merge**: combines the PR into one commit.
   - **Rebase and merge**: puts the commits on top of `main` in a straight line.
6. The head branch can then be deleted.

Optional extras: **branch protection rules** (for example, "require a PR review before merging"), **CI checks** with **GitHub Actions**, and **CODEOWNERS**.

## Workflow 4: Reject, Then Try Again

### Rejecting (`data-probe-1`)

- **Request changes**: the normal "not yet". The PR stays open and `rohingosling` gets a notification with the review comments.
- **Close the PR without merging**: a final "no". Leave a comment explaining why. A closed PR can be **reopened**.

### Trying again (`rohingosling`)

1. Make more commits on the same branch and push. **The open PR updates automatically**, so there's no need to open a new one.
2. Reply to the comments and mark them **Resolve conversation**.
3. Click **Re-request review**.
4. If `main` changed in the meantime, **sync the fork** (see below). The GitHub **Sync fork** button does the same thing for `main`.
5. Fix any **merge conflicts** the sync turns up.

```bash
git fetch upstream
git rebase upstream/main          # or: git merge upstream/main
git push --force-with-lease       # a plain git push is enough after a merge
```

If the PR was closed rather than sent back for changes, make a new branch, or rework the old one, and open a **new PR**.

**Terms:** review, request changes, approve, re-request review, resolve conversation, close vs merge, sync fork, fetch, rebase, merge conflict, force-push.

## Practical Note for Two Accounts

On the second computer, sign in as `rohingosling` and set the git identity so the commits are credited to `rohingosling`:

```bash
gh auth login
git config user.name  "rohingosling"
git config user.email "<rohingosling account email>"
```

Then the PR, the review and the notifications all show up between two separate accounts, which is what you want for this exercise.
