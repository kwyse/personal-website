---
title: "This Week I Discovered: Vim Minimalism"
date: 2018-01-14T12:41:39Z
draft: true
---

Last month marked four years since I started my
[dotfiles](https://github.com/kwyse/dotfiles) repository on GitHub. Now it's in
a state of disrepair! I had no idea that I hadn't made a single commit to it
throughout 2017. I did though, I just didn't push them. There are local changes
on every computer that I work with. Some were the results of experiments I later
forgot about and others were quickly-needed hacks that I didn't find time to
refactor and stabilise. Now it's time to clean it up.

I came across this video which explains some lesser-known Vim tips. You can
achieve close the functionality of a lot of the most popular plugins using just
a few lines of Vimscript.

{{< youtube XA2WjJbmmoM >}}

---

The first tip is adding `**` to the search path when using Vim's `find` command.
This gives behaviour similar to a fuzzy finder but with the built-in, kind-of
awkward-for-long-lists wildmenu. I think this has to potential to be used
instead of a fuzzy finder for small code bases that have short-ish file names.
If you're working with a monolith, maybe not. Fuzzy finding with the first
letter of each camel-cased word in a class name can be really helpful for
monoliths.

The second tip is explaining Vim's built in *ctags* integration for
jump-to-definition functionality. Code completion is covered separately in the
video but ctags supports this as well. Unfortunately it seems like most effort
in this space is now being dedicated towards [language server
protocols](https://microsoft.github.io/language-server-protocol/). The [Rust
Language Server](https://github.com/rust-lang-nursery/rls) is an implementation
of the protocol and has
[support](https://www.ncameron.org/blog/what-the-rls-can-do/) from the core
team.

Build integration is explained with Vim's `makeprg` configuration variable. This
is the external program called when invoking `make` from inside Vim. For simple
cases it's perfectly sufficient. The video goes on further to explain how to
integrate marked errors with Vim's quickfix list so that you can navigate among
them easily. In general, this needs to be done on a per-language basis, so I
agree with the speaker that plugins are useful here.

Personally, I've never seen the need for snippets. Maybe I'm just using the
wrong languages. =]

That leaves file browsing. The speaker talks about *netrw*, Vim's built-in file
browser. Unfortunately, netrw isn't the most stable piece of software.
[This](https://www.reddit.com/r/vim/comments/22ztqp/why_does_nerdtree_exist_whats_wrong_with_netrw/?st=jcetivby&sh=95ada33e)
Reddit thread talks about some of the alternatives.
[NERDTree](https://github.com/scrooloose/nerdtree) is the most frequently
recommended and the most popular "project draw"-type plugin. Such plugins are an
ongoing point of contention within the community, with those that want to make
Vim IDE-like and those that say this is unidiomatic and that Vim should stick to
traditional Unix philosophy: do one thing and do it well. For Vim, that's
editing text. As such, simpler alternatives have been created like
[FileBeagle](https://github.com/jeetsukumaran/vim-filebeagle) and
[dirvish](https://github.com/justinmk/vim-dirvish).

## Deciding what Vim should be

Proselytising online about idiomatic Vim configurations is all well and good for
seeing different opinions and helping you decide which camp you sit in. Vim,
along with its plugin ecosystem, is completely open and that means you can
configure out however you please.

For me, I want Vim to be *fast*. Thinking back on my usage in the last four
years, I'm generally comfortable falling back into the terminal. *Very* few of
the plugins I had installed I used to their full potential. And yet, seeing that
delay when opening Vim over and over again *would* agitate me.

Realising that removes the need for many plugins. Most of us probably use Git
everyday. Tim Pope's [fugitive.vim](https://github.com/tpope/vim-fugitive) tops
many "must-have Vim plugins" lists. The problem is that the time you spend
interacting with Git is not that much compared to the time spent reading and
writing code. I interact with Git a few times per day on average, maybe ten
times, but I'm editing code far more frequently than that. Enabling it inside
Vim seems unnecessary. Sure it might be helpful to see which branch you're on,
but do you change branches that frequently? It could be useful to see the
modifications in the sidebar like
[vim-gitgutter](https://github.com/airblade/vim-gitgutter) offers, but how often
do you actually make decisions based on that?

Instead, I think it makes more sense to use a dedicated tool like
[tig](https://jonas.github.io/tig/). You can think of this like the mode-based
philosophy of Vim, but more meta =] We were in "editing" mode and now we're in
"review" mode, where we check our changes before committing them. This is the
workflow I was already using without realising it, so it's a natural fit.

This can be taken further. During the orientation phase when learning a new
codebase, exploring with a file browser is useful. Once you're familiar with the
codebase though, you'll often just want to jump to specific files. You have a
mental map of the codebase in your head. Why include a file browser then?
Instead, we can again use an external tool like
[ranger](https://ranger.github.io/)!

It's likely there are other phases in our workflows that we can delegate to a
specialised tool. Both tig and ranger have Vim integrations available and
delegating to them when they are needed feels natural. It keeps Vim snappy and
focussed on what it's good at.

## Less plugins, more configuration

Plugins are designed to be applicable to as many people as possible. They will
support use cases you may never need. Hence I am also trying to instead take
ideas from the plugins I like and configure them inside Vim myself. It's a great
way to learn Vimscript, offers me tailored control on the behaviour, and make
sure every time I benchmark Vim's startup time I know why it's behaving the way
it is.
