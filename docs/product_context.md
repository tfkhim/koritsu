# Product context

Big merges are much more complicated and prone to errors than multiple small
merges. Therefore, many people give the advice to integrate your changes often
with the changes of other developers. With trunk based development there is a
Git workflow that forces you to do exactly that.

The required level of automation around trunk based development also leads to
very good **change load time**, **deployment frequency** and **recovery time**
metrics.

Another advantage is the linear commit history, which is very simple to
understand.

But there are also some drawbacks. There is no safty net that prevents broken
commits to be pushed to the main branch. With broken I mean commits that do not
compile or there are linter errors. Those commits block other developers from
getting their changes to production.

A possible solution is to push your commit first to a very short lived feature
branch. The continuous integration pipeline then runs on this feature branch.
If all checks are green the branch is automatically merged into the main branch.
Other developers had already a similar idea and named the approach
[Koritsu](https://debitoor.com/blog/trunk-based-development-how-we-fixed-it-with-koritsu).

## How does it work

If a developer is satisfied with their changes they can push their commits onto
a ready branch. The branch name of this branch must start with `ready/`. The
continuous integration jobs must run on those branches.

The Koritsu Github application watches the ready branches and waits for the
continuous integration runs to finish. When the workflow run was successful the
application checks if a fast forward merge into the main branch is possible. In
this case it does one of the following things:

- If the branch contains only one additional commit the application performs a
  fast forward merge
- If the branch contains more than one commit the appliction always adds a merge
  commit
