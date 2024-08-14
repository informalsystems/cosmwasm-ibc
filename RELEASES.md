## Releases

The release process for cosmwasm-ibc involves uploading the `ibc-client-cw`
and `ibc-client-tendermint-cw` crates to crates.io. As it stands, the release
[workflow][release-workflow] performs this via the `cargo release` command
whenever a branch prepended with `release` is merged.

1. Open a new branch for the purposes of performing the release. The branch
   name needs to be prepended with `release`, along with the anticipated
   version, in the form `release/vX.Y.Z`.
2. Update the changelog to reflect and summarize all the changes that were
   merged since the previous release:
   1. Run `unclog build -u` and copy-paste the output to the top of the
      `CHANGELOG.md` file.
   2. Update the header at the top of the changelog file with the new
      anticipated version.
   3. Run `unclog release --editor <editor> vX.Y.Z` to create a summary
      of all the changes that are part of this release.
      1. Your specified text editor will open. Write a release summary
         and close your editor.
      2. Add this summary to the changelog as well.
   4. Commit the updated `CHANGELOG.md` and `.changelog` directory to
      the repo.
3. Push this branch and open a *draft PR*.
4. Bump the versions of all crates to the new version in their respective
   `Cargo.toml`s and in the root `Cargo.toml` as well, then push these
   changes to the release PR.
5. Mark the PR as **Ready for Review** and incorporate any feedback on the
   release. Once approved, merge the PR.
   1. The release [workflow][release-workflow] will run `cargo release --execute`
      command in a CI worker.
6. Checkout the `main` branch and pull it with
   `git checkout main && git pull origin main`.
7. Create a signed tag `git tag -s -a vX.Y.Z`. In the tag message, write the
   version and the link to the corresponding section of the changelog. Then push
   the tag to GitHub with `git push origin vX.Y.Z`.
8. If some crates were not successfully released, check the cause of the failure and
    act accordingly:
    1. In case of intermittent problems with the registry, try `cargo release`
       locally to publish any missing crates from this release. This step
       requires the appropriate privileges to push crates to [crates.io].
    2. If there is any new crate published locally, add
       [ibcbot] to its owners' list.
    3. In case problems arise from the source files, fix them, bump a new patch
       version (e.g. `v0.48.1`) and repeat the process with its corresponding
       new tag.
9. Once the tag is pushed, wait for the CI bot to create a GitHub release, then
    update the release description and append:
    `[📖CHANGELOG](https://github.com/cosmos/ibc-rs/blob/main/CHANGELOG.md#vXYZ)`

[release-workflow]: ./.github/workflows/release.yaml
[ibcbot]: https::crates.io/users/ibcbot

