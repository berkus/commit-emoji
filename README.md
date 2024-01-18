# commit-emoji

Adorn your conventional commits with emoji.

It will add emoji automatically as a commit-msg git hook, so you only need to specify conventional commit's type and/or specify if a change is breaking (adding an emoji for breaking changes is optional).

Almost the entire tool's source code is duplicated from awesomely written [lazycc](https://gitlab.com/ogarcia/lazycc) by Óscar García Amor. Thank you for providing this solid foundation! As a result, it is also licensed under GPLv3.

## Configuration

commit-emoji contains a built-in default configuration based on my [gist](https://gist.github.com/berkus/5ce2cdf5dd74909bcd4faf6cb7d0ae18), so if you're fine with the selected emoji, you don't have to configure anything.

Custom configuration file can be placed in `.commit-emoji.toml` in the root of a git repository to provide
per-repository settings or in `~/.config/commit-emoji/config.toml` to provide global per-user settings.

The values in the config file will override behavior of the default config, not replace it.

Supported configuration options are visible in the provided configuration file in the root of this repo.

## Operation

* Run `cargo install commit-emoji` to install the tool.

* Run `commit-emoji -i` from inside a git repo to install the git hook.

* Run `commit-emoji -u` from a git repo to remove the hook. Removing the hook is safe - it will check the hook file to be the exact match of what it installed an warn you if you have any modifications in it.
