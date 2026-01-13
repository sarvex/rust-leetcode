// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     /// Creates a new `ListNode` with the given value and no next node.
//     pub fn new(val: i32) -> Self {
//         Self { val, next: None }
//     }
// }

impl Solution {
    /// Swaps every two adjacent nodes in a linked list.
    ///
    /// # Intuition
    /// For each pair of nodes, swap their positions by adjusting pointers.
    /// Recursively process the rest of the list after each swap.
    ///
    /// # Approach
    /// Use recursion to swap pairs from the end of the list backward:
    /// 1. Base case: if fewer than two nodes remain, return as-is
    /// 2. Take the first two nodes and recursively swap the remainder
    /// 3. Point the first node to the result of the recursive call
    /// 4. Point the second node to the first node
    /// 5. Return the second node as the new head of this pair
    ///
    /// # Complexity
    /// - Time: O(n) where n is the number of nodes
    /// - Space: O(n) due to recursion stack
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut first) => match first.next.take() {
                None => Some(first),
                Some(mut second) => {
                    first.next = Self::swap_pairs(second.next.take());
                    second.next = Some(first);
                    Some(second)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: &[i32]) -> Option<Box<ListNode>> {
        vec.iter()
            .rev()
            .fold(None, |next, &val| {
                let mut node = Box::new(ListNode::new(val));
                node.next = next;
                Some(node)
            })
    }

    fn from_list(list: Option<Box<ListNode>>) -> Vec<i32> {
        std::iter::successors(list.as_ref(), |node| node.next.as_ref())
            .map(|node| node.val)
            .collect()
    }

    #[test]
    fn test_swap_pairs_even_length() {
        let head = to_list(&[1, 2, 3, 4]);
        let result = Solution::swap_pairs(head);
        assert_eq!(from_list(result), vec![2, 1, 4, 3]);
    }

    #[test]
    fn test_swap_pairs_odd_length() {
        let head = to_list(&[1, 2, 3]);
        let result = Solution::swap_pairs(head);
        assert_eq!(from_list(result), vec![2, 1, 3]);
    }

    #[test]
    fn test_swap_pairs_single_element() {
        let head = to_list(&[1]);
        let result = Solution::swap_pairs(head);
        assert_eq!(from_list(result), vec![1]);
    }

    #[test]
    fn test_swap_pairs_empty() {
        let head = to_list(&[]);
        let result = Solution::swap_pairs(head);
        assert_eq!(from_list(result), Vec::<i32>::new());
    }

    #[test]
    fn test_swap_pairs_two_elements() {
        let head = to_list(&[1, 2]);
        let result = Solution::swap_pairs(head);
        assert_eq!(from_list(result), vec![2, 1]);
    }
}

==============================================================================
snacks:                                                            16 ⚠️  7 ❌

Snacks ~
- ✅ OK setup called

Snacks.bigfile ~
- ⚠️ WARNING setup {disabled}

Snacks.dashboard ~
- ⚠️ WARNING setup {disabled}

Snacks.explorer ~
- ✅ OK setup {enabled}
- ✅ OK 'trash' `# Un-recognized argument --version at index 1`
- ✅ OK 'gio' `2.86.3`
- ✅ OK System trash command found

Snacks.image ~
- ⚠️ WARNING setup {disabled}
- ✅ OK 'kitty' `kitty 0.45.0 created by Kovid Goyal`
- ✅ OK 'wezterm' `wezterm 20240203-110809-5046fc22`
- ❌ ERROR None of the tools found: 'magick', 'convert'
- ❌ ERROR `magick` is required to convert images. Only PNG files will be displayed.
- ✅ OK Terminal Dimensions:
  - {size}: `2700` x `1800` pixels
  - {scale}: `1.12`
  - {cell}: `9` x `18` pixels
- ✅ OK Available Treesitter languages:
    `css`, `html`, `javascript`, `latex`, `markdown_inline`, `markdown`, `scss`, `svelte`, `tsx`, `typst`, `vue`
- ⚠️ WARNING Missing Treesitter languages:
    `norg`
- ⚠️ WARNING Image rendering in docs with missing treesitter parsers won't work
- ❌ ERROR Tool not found: 'gs'
- ⚠️ WARNING `gs` is required to render PDF files
- ❌ ERROR None of the tools found: 'tectonic', 'pdflatex'
- ⚠️ WARNING `tectonic` or `pdflatex` is required to render LaTeX math expressions
- ❌ ERROR Tool not found: 'mmdc'
- ⚠️ WARNING `mmdc` is required to render Mermaid diagrams
- ❌ ERROR your terminal does not support the kitty graphics protocol
- supported terminals: `kitty`, `wezterm`, `ghostty`

Snacks.input ~
- ⚠️ WARNING setup {disabled}

Snacks.lazygit ~
- ✅ OK {lazygit} installed

Snacks.notifier ~
- ⚠️ WARNING setup {disabled}
- ❌ ERROR is not ready

Snacks.picker ~
- ⚠️ WARNING setup {disabled}
- ⚠️ WARNING `vim.ui.select` for `Snacks.picker` is not enabled
- ✅ OK Available Treesitter languages:
    `regex`
- ✅ OK 'git' `git version 2.50.1 (Apple Git-155)`
- ✅ OK 'rg' `ripgrep 15.1.0`
- ✅ OK `Snacks.picker.grep()` is available
- ✅ OK 'fd' `fd 10.3.0`
- ✅ OK `Snacks.picker.files()` is available
- ✅ OK `Snacks.picker.explorer()` is available
- ✅ OK `SQLite3` is available

Snacks.quickfile ~
- ⚠️ WARNING setup {disabled}

Snacks.scope ~
- ✅ OK setup {enabled}

Snacks.scroll ~
- ⚠️ WARNING setup {disabled}

Snacks.statuscolumn ~
- ⚠️ WARNING setup {disabled}

Snacks.terminal ~
- ✅ OK shell configured
  - `vim.o.shell`: /bin/zsh
  - `parsed`: { "/bin/zsh" }

Snacks.toggle ~
- ⚠️ WARNING {which-key} is not installed

Snacks.words ~
- ✅ OK setup {enabled}

==============================================================================
vim.deprecated:                                                           1 ⚠️

 ~
- ⚠️ WARNING vim.validate is deprecated. Feature will be removed in Nvim 1.0
  - ADVICE:
    - use vim.validate(name, value, validator, optional_or_msg) instead.
    - stack traceback:
        /Users/sarvex/.vscode-oss/extensions/asvetliakov.vscode-neovim-1.18.24-universal/runtime/lua/vscode/api.lua:235
        /Users/sarvex/.vscode-oss/extensions/asvetliakov.vscode-neovim-1.18.24-universal/runtime/lua/vscode/sync-options.lua:119
        /Users/sarvex/.vscode-oss/extensions/asvetliakov.vscode-neovim-1.18.24-universal/runtime/lua/vscode.lua:12
        [C]:-1
        lua:1
    - stack traceback:
        /Users/sarvex/.vscode-oss/extensions/asvetliakov.vscode-neovim-1.18.24-universal/runtime/lua/vscode/api.lua:235
        /Users/sarvex/.vscode-oss/extensions/asvetliakov.vscode-neovim-1.18.24-universal/runtime/lua/vscode/sync-options.lua:120
        /Users/sarvex/.vscode-oss/extensions/asvetliakov.vscode-neovim-1.18.24-universal/runtime/lua/vscode.lua:12
        [C]:-1
        lua:1

==============================================================================
vim.health:                                                               1 ❌

Configuration ~
- ✅ OK no issues found

Runtime ~
- ✅ OK $VIMRUNTIME: /opt/homebrew/Cellar/neovim/0.11.5_1/share/nvim/runtime

Performance ~
- ✅ OK Build type: Release

Remote Plugins ~
- ✅ OK Up to date

terminal ~
- ❌ ERROR command failed: infocmp -L
  infocmp: environment variable TERM not set


External Tools ~
- ✅ OK ripgrep 15.1.0 (/opt/homebrew/bin/rg)

==============================================================================
vim.lsp:                                                                    ✅

- LSP log level : WARN
- Log path: /Users/sarvex/.local/state/nvim/lsp.log
- Log size: 0 KB

vim.lsp: Active Clients ~
- No active clients

vim.lsp: Enabled Configurations ~

vim.lsp: File Watcher ~
- file watching "(workspace/didChangeWatchedFiles)" disabled on all clients

vim.lsp: Position Encodings ~
- No active clients

==============================================================================
vim.provider:                                                             6 ⚠️

Clipboard (optional) ~
- ✅ OK Clipboard tool found: pbcopy

Node.js provider (optional) ~
- Node.js: v25.2.1

- ⚠️ WARNING Missing "neovim" npm (or yarn, pnpm) package.
  - ADVICE:
    - Run in shell: npm install -g neovim
    - Run in shell (if you use yarn): yarn global add neovim
    - Run in shell (if you use pnpm): pnpm install -g neovim
    - You may disable this provider (and warning) by adding `let g:loaded_node_provider = 0` to your init.vim

Perl provider (optional) ~
- ⚠️ WARNING "Neovim::Ext" cpan module is not installed
  - ADVICE:
    - See :help |provider-perl| for more information.
    - You can disable this provider (and warning) by adding `let g:loaded_perl_provider = 0` to your init.vim
- ⚠️ WARNING No usable perl executable found

Python 3 provider (optional) ~
- ⚠️ WARNING No Python executable found that can `import neovim`. Using the first available executable for diagnostics.
- ⚠️ WARNING Could not load Python :
  /usr/bin/python3 does not have the "neovim" module.
  /opt/homebrew/bin/python3.13 does not have the "neovim" module.
  /opt/homebrew/bin/python3.12 does not have the "neovim" module.
  python3.11 not found in search path or not executable.
  python3.10 not found in search path or not executable.
  python3.9 not found in search path or not executable.
  python not found in search path or not executable.
  - ADVICE:
    - See :help |provider-python| for more information.
    - You can disable this provider (and warning) by adding `let g:loaded_python3_provider = 0` to your init.vim
- Executable: Not found

Python virtualenv ~
- ✅ OK no $VIRTUAL_ENV

Ruby provider (optional) ~
- Ruby: ruby 2.6.10p210 (2022-04-12 revision 67958) [universal.arm64e-darwin25]
- ⚠️ WARNING `neovim-ruby-host` not found.
  - ADVICE:
    - Run `gem install neovim` to ensure the neovim RubyGem is installed.
    - Run `gem environment` to ensure the gem bin directory is in $PATH.
    - If you are using rvm/rbenv/chruby, try "rehashing".
    - See :help |g:ruby_host_prog| for non-standard gem installations.
    - You can disable this provider (and warning) by adding `let g:loaded_ruby_provider = 0` to your init.vim

==============================================================================
vim.treesitter:                                                             ✅

Treesitter features ~
- Treesitter ABI support: min 13, max 15
- WASM parser support: false

Treesitter parsers ~
- ✅ OK Parser: angular                   ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/angular.so
- ✅ OK Parser: astro                     ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/astro.so
- ✅ OK Parser: bash                      ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/bash.so
- ✅ OK Parser: bibtex                    ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/bibtex.so
- ✅ OK Parser: c                         ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/c.so
- ✅ OK Parser: c                    (not loaded), path: /opt/homebrew/Cellar/neovim/0.11.5_1/lib/nvim/parser/c.so
- ✅ OK Parser: c_sharp                   ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/c_sharp.so
- ✅ OK Parser: clojure                   ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/clojure.so
- ✅ OK Parser: cmake                     ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/cmake.so
- ✅ OK Parser: cpp                       ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/cpp.so
- ✅ OK Parser: css                       ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/css.so
- ✅ OK Parser: dart                      ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/dart.so
- ✅ OK Parser: diff                      ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/diff.so
- ✅ OK Parser: dockerfile                ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/dockerfile.so
- ✅ OK Parser: dtd                       ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/dtd.so
- ✅ OK Parser: eex                       ABI: 13, path: /Users/sarvex/.local/share/nvim/site/parser/eex.so
- ✅ OK Parser: elixir                    ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/elixir.so
- ✅ OK Parser: elm                       ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/elm.so
- ✅ OK Parser: erlang                    ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/erlang.so
- ✅ OK Parser: fish                      ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/fish.so
- ✅ OK Parser: fsharp                    ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/fsharp.so
- ✅ OK Parser: git_config                ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/git_config.so
- ✅ OK Parser: git_rebase                ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/git_rebase.so
- ✅ OK Parser: gitattributes             ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/gitattributes.so
- ✅ OK Parser: gitcommit                 ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/gitcommit.so
- ✅ OK Parser: gitignore                 ABI: 13, path: /Users/sarvex/.local/share/nvim/site/parser/gitignore.so
- ✅ OK Parser: gleam                     ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/gleam.so
- ✅ OK Parser: glimmer                   ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/glimmer.so
- ✅ OK Parser: glimmer_javascript        ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/glimmer_javascript.so
- ✅ OK Parser: glimmer_typescript        ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/glimmer_typescript.so
- ✅ OK Parser: go                        ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/go.so
- ✅ OK Parser: gomod                     ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/gomod.so
- ✅ OK Parser: gosum                     ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/gosum.so
- ✅ OK Parser: gowork                    ABI: 13, path: /Users/sarvex/.local/share/nvim/site/parser/gowork.so
- ✅ OK Parser: haskell                   ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/haskell.so
- ✅ OK Parser: hcl                       ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/hcl.so
- ✅ OK Parser: heex                      ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/heex.so
- ✅ OK Parser: helm                      ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/helm.so
- ✅ OK Parser: html                      ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/html.so
- ✅ OK Parser: java                      ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/java.so
- ✅ OK Parser: javascript                ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/javascript.so
- ✅ OK Parser: jsdoc                     ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/jsdoc.so
- ✅ OK Parser: json                      ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/json.so
- ✅ OK Parser: json5                     ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/json5.so
- ✅ OK Parser: jsonc                     ABI: 13, path: /Users/sarvex/.local/share/nvim/site/parser/jsonc.so
- ✅ OK Parser: julia                     ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/julia.so
- ✅ OK Parser: kotlin                    ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/kotlin.so
- ✅ OK Parser: latex                     ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/latex.so
- ✅ OK Parser: lua                       ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/lua.so
- ✅ OK Parser: lua                  (not loaded), path: /opt/homebrew/Cellar/neovim/0.11.5_1/lib/nvim/parser/lua.so
- ✅ OK Parser: luadoc                    ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/luadoc.so
- ✅ OK Parser: luap                      ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/luap.so
- ✅ OK Parser: markdown                  ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/markdown.so
- ✅ OK Parser: markdown             (not loaded), path: /opt/homebrew/Cellar/neovim/0.11.5_1/lib/nvim/parser/markdown.so
- ✅ OK Parser: markdown_inline           ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/markdown_inline.so
- ✅ OK Parser: markdown_inline      (not loaded), path: /opt/homebrew/Cellar/neovim/0.11.5_1/lib/nvim/parser/markdown_inline.so
- ✅ OK Parser: ninja                     ABI: 13, path: /Users/sarvex/.local/share/nvim/site/parser/ninja.so
- ✅ OK Parser: nix                       ABI: 13, path: /Users/sarvex/.local/share/nvim/site/parser/nix.so
- ✅ OK Parser: nu                        ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/nu.so
- ✅ OK Parser: ocaml                     ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/ocaml.so
- ✅ OK Parser: php                       ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/php.so
- ✅ OK Parser: php_only                  ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/php_only.so
- ✅ OK Parser: printf                    ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/printf.so
- ✅ OK Parser: prisma                    ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/prisma.so
- ✅ OK Parser: python                    ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/python.so
- ✅ OK Parser: query                     ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/query.so
- ✅ OK Parser: query                (not loaded), path: /opt/homebrew/Cellar/neovim/0.11.5_1/lib/nvim/parser/query.so
- ✅ OK Parser: r                         ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/r.so
- ✅ OK Parser: regex                     ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/regex.so
- ✅ OK Parser: rego                      ABI: 13, path: /Users/sarvex/.local/share/nvim/site/parser/rego.so
- ✅ OK Parser: rnoweb                    ABI: 13, path: /Users/sarvex/.local/share/nvim/site/parser/rnoweb.so
- ✅ OK Parser: ron                       ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/ron.so
- ✅ OK Parser: rst                       ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/rst.so
- ✅ OK Parser: ruby                      ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/ruby.so
- ✅ OK Parser: rust                      ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/rust.so
- ✅ OK Parser: scala                     ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/scala.so
- ✅ OK Parser: scss                      ABI: 13, path: /Users/sarvex/.local/share/nvim/site/parser/scss.so
- ✅ OK Parser: solidity                  ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/solidity.so
- ✅ OK Parser: sql                       ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/sql.so
- ✅ OK Parser: svelte                    ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/svelte.so
- ✅ OK Parser: terraform                 ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/terraform.so
- ✅ OK Parser: thrift                    ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/thrift.so
- ✅ OK Parser: toml                      ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/toml.so
- ✅ OK Parser: tsx                       ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/tsx.so
- ✅ OK Parser: twig                      ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/twig.so
- ✅ OK Parser: typescript                ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/typescript.so
- ✅ OK Parser: typst                     ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/typst.so
- ✅ OK Parser: vim                       ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/vim.so
- ✅ OK Parser: vim                  (not loaded), path: /opt/homebrew/Cellar/neovim/0.11.5_1/lib/nvim/parser/vim.so
- ✅ OK Parser: vimdoc                    ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/vimdoc.so
- ✅ OK Parser: vimdoc               (not loaded), path: /opt/homebrew/Cellar/neovim/0.11.5_1/lib/nvim/parser/vimdoc.so
- ✅ OK Parser: vue                       ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/vue.so
- ✅ OK Parser: xml                       ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/xml.so
- ✅ OK Parser: yaml                      ABI: 14, path: /Users/sarvex/.local/share/nvim/site/parser/yaml.so
- ✅ OK Parser: zig                       ABI: 15, path: /Users/sarvex/.local/share/nvim/site/parser/zig.so
