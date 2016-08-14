title: My development setup
author: Krishan Wyse
date: 14 August, 2016
path: my-development-setup
summary: An overview of my development setup and the tools I use

Let's briefly talk about operating systems. The first time I used Linux was in late 2012. One of my courses at university required us to use some Unix shell software. Prior to this, I had used Windows almost exclusively and OS X a bit here and there. I hadn't delved into using a shell before. I was content with the Windows way of doing things as that is all I had known.

Fast forward a year, Linux had become my platform of choice and the shell a key component of workflow. I liked the speed and precision of the shell and how I was forced to understand the intricacies of the build processes I was previously protected from. During that university assignment, I stuck with [Ubuntu](http://www.ubuntu.com/). It was simple to install and didn't have any dangerous pitfalls. I tried out a few other distros during this time, including [Manjaro](https://manjaro.github.io/), Crunchbang (no longer actively developed), [Mint](https://www.linuxmint.com/) and [Fedora](https://getfedora.org/). Soon I heard about [Arch](https://www.archlinux.org/), and that it was a blazingly fast, hyper-optimised and minimalist distro that only the elite hackers use. I had to try it out. On about my fourth or fifth installation attempt, it actually booted. I was so happy. I was about to join that crowd.

Fast forward another year, Arch had become my primary OS. I began hearing about an even more hardcore distro named [Gentoo](https://www.gentoo.org/). The same process repeated, and now I use Arch on my desktop and Gentoo on my laptop. I admire facets of both. I prefer how Arch's package manager distributes binaries, as it makes installation of software much quicker. The extra optimisation Gentoo offers by forcing you to compile all software locally is not something I've found practical. But Gentoo supports far more official packages and its [overlay system](https://wiki.gentoo.org/wiki/Overlay) means I've always found the package I was looking. Arch does have the [AUR](https://wiki.archlinux.org/index.php/Arch_User_Repository), though I've had experiences with AUR packages not installing without config changes. Arch's wiki is in a class of its own in terms of breadth of content, but I've found Gentoo's to have more accurate information and better presentation.

The point is that they are both great distros and perfect for software development. They both contain the latest versions of libraries and all common development tools. I am very content using either. So with that, let's talk about some actual tools.

# The i3 window manager

I use [i3](https://i3wm.org/) and I wouldn't trade it for anything else. i3 is a **tiling** window manager, as opposed **stacking**, which is how Windows and OS X work. Tiling window managers will arrange windows on a screen for you, typically in a grid formation. This means you can see all windows in one view. Windows has a dual-view snapping feature which is similar. Think of that, but with the window manager intelligently laying out new windows and adjusting the layout when a window is closed.

Combine this notion with workspaces, of which you can have up to 10 with i3, and you never need to alt-tab again. I separate my workspaces based on task, and typically use four. First is my terminals, where flexibility is unbounded and where I do any ad hoc tasks. The second is my browser. The third is Emacs and the forth is usually another GUI-based program I'm working with at the time, often GIMP. I then have six more workspaces available should I need to open more windows that logically don't belong in one of those categories. My i3 config specifies a single keystroke to move windows between workspaces, so if I ever want to rewrite something I've found in my browser in Emacs, I can just move Emacs over to the browser workspace. The layout algorithm will take care to place Emacs in a suitable location. It makes manual window management seldom necessary, though there are instances where it's necessary. I use GIMP in i3's floating mode, which is just like stacking, because I've found GIMP's docks are unwieldy to manage when tiled.

i3 is configurable beyond setting key bindings. You can specify default programs and even save layouts! Layouts are stored as JSON and this makes it trivial to keep a tailored workspace layout between sessions. The configuration file itself is plain text and well documented. The same can be said for the i3 docs, which has a very nice [user guide](https://i3wm.org/docs/userguide.html).

There are a few other tools that complement i3 very well. For my task bar, I use [i3-blocks](https://github.com/vivien/i3blocks), which uses a nice INI-like format to specify "blocks" on your task bar, which are simply the outputs of commands like `date`, `amixer` and the like. For an application launcher, I use [Rofi](https://davedavenport.github.io/rofi/). Rofi is akin to OS X's application launcher, but orders of magnitude more performant.

There are many other tiling windows managers out there. Ones I've tried include [awesome](https://awesome.naquadah.org/) and [xmonad](http://xmonad.org/). I have nothing against these two, though I found their documentation lacking compared to i3, and that was what swung me over. Despite that, both are very popular and respected.

# URxvt and zsh

I use [URxvt](https://wiki.archlinux.org/index.php/rxvt-unicode) (also known as rxvt-unicode) as a terminal emulator. It supports unicode and 256 terminal colours, whilst still being lightweight. The only other terminal emulators I've tried are those that come default with distros, but these often lack 256-colour support or are heavier than URxvt. I have yet to find one that strikes this balance so well.

For the shell itself, I look no further than [zsh](https://wiki.archlinux.org/index.php/zsh). zsh has [significantly more features](http://fendrich.se/blog/2012/09/28/no/) than bash, but what does it for me is the smarter auto-completion, globbing support, case-insensitive spell matching, filtered command history... the list goes on.

I've customised my shell prompt to show extra information when inside a Git repository. The branch name, commit number delta between the local and remote branches, and whether or not there are untracked, unstaged or staged files present is all displayed in the prompt. This was inspired by [this post](http://stevelosh.com/blog/2010/02/my-extravagant-zsh-prompt/) by Steve Losh, though I've adjusted it to suit my needs.

# Emacs and vim

I started to learn vim with that university assignment. It still seemed pretty arcane until I read [Practical Vim](https://pragprog.com/book/dnvim2/practical-vim-second-edition) by Drew Neil. The book is organised as a series of tips that gradually move from essential to specialised, but all of them are useful. I owe most of my proficiency with vim to this book.

Vim kept my happy for a long time, but it became very slow for me when I added plugins in attempt to make it almost IDE-like. I began looking for an alternative. I had heard of Emacs, but when I went through the tutorial, I found the key bindings just too jarring, especially compared to vim. The turnaround came when I discovered [`evil-mode`](https://bitbucket.org/lyro/evil/wiki/Home). It is the best Vi-key bindings overlay I've used. I was able to get along with it fine out of the box after one key binding change for `Ctrl-U` so that the buffer scrolls half of a page up, like in vim, instead of being the Emacs count prefix key binding.

Emacs has a lot of advantages. Its package management is built-in and its package ecosystem is [more extensive](https://melpa.org/#/). For me, the performance is better too, despite now having even more plugins active than I did with vim. I still use vim for one-off tasks, though often I launch it only from terminal sessions. I keep Emacs in a dedicated window and almost always have it open with the project I am currently working on. They both use near enough the same key bindings with `evil-mode`, so interchange is seamless.

I've stripped the vim plugins I use down to the bare minimum, but these five are particularly essential for me:

* [`vim-commentary`](https://github.com/tpope/vim-commentary) - commenting and uncommenting code
* [`vim-surround`](https://github.com/tpope/vim-surround) - changing surrounding parenthesis/brackings/braces/quotes/etc.
* [`vim-unimpaired`](https://github.com/tpope/vim-unimpaired) - useful extra key bindings, most are in pairs
* [`vim-textobj-user`](https://github.com/kana/vim-textobj-user) - define custom text objects
* [`vim-textobj-entire`](https://github.com/kana/vim-textobj-entire) - text object for the entire buffer

Emacs plugins are a little harder to narrow down because many are modularised frameworks that depend on other plugins. Still, these are some standouts:

* [`use-package`](https://github.com/jwiegley/use-package) - isolates package configuration in your Emacs config file
* [`helm`](https://emacs-helm.github.io/helm/) - autocompletion back-end
* [`company-mode`](http://company-mode.github.io/) - autocompletion front-end
* [`flycheck`](http://www.flycheck.org/en/latest/) - syntax checking
* [`projectile`](http://batsov.com/projectile/) - project navigation and management
* [`magit`](https://magit.vc/) - Git interface
* [`evil-mode`](https://bitbucket.org/lyro/evil/wiki/Home) - Vi-like sanity

I find this setup works quite well. After all, it's best [if you can work with both](http://sachachua.com/blog/2013/05/how-to-learn-emacs-a-hand-drawn-one-pager-for-beginners/).

# My personal configuration files

You can find my configs for these tools and more in my [dotfiles repo](https://github.com/kwyse/dotfiles) on GitHub.

Hopefully this post offered some intrigue that caused you to Google something, or perhaps introduce you to a plugin you weren't aware of. There are so many tools out there and I regularly find new things to help me improve my workflow all the time. With a little effort in scoping them out, coding can be a lot more fun and productive.

Thanks for reading!
