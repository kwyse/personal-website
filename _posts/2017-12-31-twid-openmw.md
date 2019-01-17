---
layout: post
title: "This Week I Discovered: OpenMW"
date: 2017-12-31T12:11:10Z
categories: ["Gaming"]
series: ["This Week I Discovered"]
---

Two weeks back I installed Gentoo with the intention to use it as my primary
desktop. Naturally I need games to play. My go-to benchmark for the state of
Linux gaming has always been the [Wine Application
Database](https://appdb.winehq.org/), so I checked it for a game I knew I could
sink countless hours into.

Well it turns out the
[results](https://appdb.winehq.org/objectManager.php?sClass=application&iId=3150)
for Oblivion are not too promising, especially for Steam. Maybe Morrowind fairs
better?
[Yes](https://appdb.winehq.org/objectManager.php?sClass=application&iId=1015),
but there are still issues. But the description holds a gem: a Linux-native
alternative called [OpenMW](https://openmw.org/en/).

If you want a video summary, check out [the FAQ
video](https://www.youtube.com/watch?v=g2PKBD0D9Gw). It's quite dated but still
shown prominently on the official site and gives a good overview of the
project.

Essentially, OpenMW is an open source reimplementation of the game engine for
Morrowind. It's not a different game because it still relies on assets from the
original. You can't *play* Morrowind through it without actually owning a copy
of Morrowind, or at the very least having access to the game's data files.

# Trying it out

A TES game running natively on Linux? Sign me up. It even has an official Gentoo
[ebuild](https://packages.gentoo.org/packages/games-engines/openmw)! Getting it
running was remarkably simple. The package includes a wizard that will guide you
through retrieving the data files from the original game. I have a physical copy
but Steam and GOG versions are also supported. It launched without a hitch.

Playing it was a different story... Straight of the boat, I was hitting between
four and ten FPS. Of course it wouldn't be that easy! I couldn't find anything
Gentoo-specific but the general consensus online was to make sure that your GPU
drivers were up to date. I had only installed my OS in the last two weeks so I
knew that couldn't have been the issue.

Turns out it was! OpenMW doesn't seem to play nice with the
[`nouveau`](https://wiki.gentoo.org/wiki/Nouveau) drivers. These are open source
NVIDIA drivers that are installed by default when running through the Gentoo
handbook. Gentoo has a
[guide](https://wiki.gentoo.org/wiki/NVidia/nvidia-drivers) on replacing the
`nouveau` drivers with the proprietary `nvidia` ones.

After rebuilding my kernel and a few package builds (and rebuilds) later, I had
booted into a new `nvidia`-driven X session. Everything seemed to be working
except for when I did the test the guide recommends: running `glxinfo`, which
kept failing. This command is meant to say that direct rendering is enabled.
Guess that means it's not enabled? Fortunately there's a troubleshooting section
for that issue. You need to disable the `Direct Rendering Manager` in the
kernel. After doing that and rebuilding the kernel, which took quite a while
considering only one option was disabled, a lot of warnings came up. The advice
online was to ensure that the `Direct Rendering Manager` was enabled... :(

Despite that, I pushed on and tried OpenMW again. Lo and behold 280 FPS! That
will do.

# New toys

Everything seems to be running smoothly despite those warnings, but time will
tell how long that lasts. At least it always teaches you something ;)

OpenMW is still not at v1 yet but it's a very mature project. It's also not the
only project of this nature. Some others include:

* [OpenRCT2](https://openrct2.org/) - for RollerCoaster Tycoon 2
* [OpenTTD](https://www.openttd.org/en/) - for Transport Tycoon Deluxe
* [openage](http://openage.sft.mx/) - for Age of Empires
* [OpenRA](http://www.openra.net/) - for Command & Conquer

Many years of my childhood were spent with RCT and RCT2 so I couldn't help but
try out OpenRCT2. Like OpenMW, it has an official Gentoo
[package](https://packages.gentoo.org/packages/games-simulation/openrct2) and a
[getting started](https://openrct2.website/getting-started/) guide that will set
you up. Ten minutes later I was back in Magic Mountain, but with a day and night
cycle! As a bonus, there's an
[option](https://github.com/OpenRCT2/OpenRCT2/wiki/Loading-RCT1-scenarios-and-data)
to point to RCT data files and play scenarios from the original game.

The sentiment behind these projects is beautiful. Not only do they allow us to
preserve these wonderful games and keep them playable for everyone, but they
open them up to younger players by adding features that are simply expected
today, like widescreen support. Given their timeless gameplay, I expect that
they will always have a community of players, and that is a very good thing.
