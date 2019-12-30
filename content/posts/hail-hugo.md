+++
title = "Hail, Hugo!"
author = ["Krishan Wyse"]
publishDate = 2017-12-28T00:00:00+00:00
tags = ["blogging"]
categories = ["blogging"]
draft = false
+++

One of my goals this holiday season was to relaunch my blog. My previous attempt
failed because there was too much friction in actually _blogging_. It was an
[Iron](http://ironframework.io/)-backed Rust web app using PostgreSQL and [Diesel](http://diesel.rs/) on a DigitalOcean host
behind NGINX. It was my first time using most of these technologies for anything
serious. I learnt a lot about them in the course of building that blog but the
fifth blog post never saw the light of day.  Deployment was too manual and
enhancements were a time sync. Ultimately it became a burden.

Enter static site generators. I know I'm late to the party, but at least I now
comprehend the benefit ;) [Hugo](https://gohugo.io/) is one such static site generator that seems
[fairly popular](https://github.com/gohugoio/hugo/stargazers). And I can see why: it is _so_ easy to get something up and
running quickly.  Here I'll show you how to go from running that initial
scaffolding command to hitting [insert your domain name] and seeing it fully
deployed.

One thing of note is exactly how much Hugo balances convention (or opinion)
against configuration. Hugo touts configuration as one of its strengths and this
was an initial draw for me. I try to avoid heavily magic-laden frameworks like
the plague. Sometimes that can go too far, when the desire to build everything
from scratch is strong. So in an effort to find balance, I want to start with
the absolute bare minimum and build up from there. We should not add anything
that doesn't have an explicit and understood purpose. Thus far, Hugo has
accommodated that, so let's begin.


## Building the basics {#building-the-basics}

Hugo already has a great [quick start](https://gohugo.io/getting-started/quick-start/) guide that we'll follow to begin with.

First install Hugo with your package manager of choice.

```sh
# OS X buddies
$ brew install hugo

# Arch compatriots
$ sudo pacman --sync hugo

# Gentoo and friends
$ sudo layman --add go-overlay
$ sudo emerge --ask www-apps/hugo
```

Let's create a skeleton site and add a theme. You can browse available themes
[here](https://themes.gohugo.io/). I'll be using the
[Minimo](https://minimo.netlify.com/) theme.

```sh
$ hugo new site wild-baguette $ cd wild-baguette $ git init $ git
submodule add https://github.com/MunifTanjim/minimo themes/minimo
```

This is _all_ you need for a basic site without any content (yet). Let's pause
here and check out the folder structure.

```sh
$ tree -I minimo # ignore the theme directory
.
├── archetypes
│   └── default.md
├── config.toml
├── content
├── data
├── layouts
├── static
└── themes
```

Excluding the theme, there's only two files! We'll cover `archtype=s in a moment
when we add some content, but for now let's open up =config.toml`. You will
likely need to add options specific to the theme you chose. Most importantly,
actually set the theme with the `theme` key! You can find out relevant options
for the Minimo theme on [this commit](https://github.com/kwyse/personal-website/blob/b00c1f66a4a30f260347a8507d479f0c9fde36f9/config.toml) I made for this site. Minimo has its own
comprehensive configuration [example](https://themes.gohugo.io/theme/minimo/docs/example-config-toml/), but the only required key is the
`recentPostsLength`. Forgetting to add this key will give you an error upon
starting the server.

Fire up the Hugo server and navigate to the URL displayed in the output.  You
should see something beautiful. If not, ensure your configuration is correct.

```sh
$ hugo server
```


## Adding content and customising {#adding-content-and-customising}

What good are wild baguettes if you can't find them? Being good citizens, we'll
share our knowledge on their habitats in our first blog post.

```sh
$ hugo new posts/where_to_find_them.md
```

The `posts` directory will be nested under `contents` in the project root. Fill
this file with whatever pleases you. The important thing is the front
matter. That's the bit in the `---` block at the top of the file. By default,
Hugo will use the `title` for the post's title (say what?) and the name of the
file for its URL path. It's important that `title` is set.

If you're wondering what dictates that front matter, that's the archetype! Hugo
allows you to [define your own archetypes](https://gohugo.io/content-management/archetypes/) to streamline adding the content you
wish to provide. The default archetype suffices for a simple blog.

Nesting the new post under the `posts` directory will also nest it in the URL.
With that in mind, it makes sense to have a `posts` page on the site. We need to
create a new file for that.

```sh
$ echo '---\ntitle: Posts\n---' > content/posts/_index.md
```

Each [section](https://gohugo.io/content-management/sections/), or distinct part of our site, can contain a `_index.md` file that
represents the section itself rather than its children, like posts do.  The
value we give to `title` here will be what's shown on the section page and in
the navigation bar. One more change: add `sectionPagesMenu = "main"` to
`config.toml`. The value you pass here should be the name of your menu, which
could be different for your chosen theme. For Minimo, it is `main`.

Start the server again and see our new navigation menu and post rendered. The
`-D` flag here indicates we want to render draft posts in addition to regular
ones.

```sh
$ hugo server -D
```

Let's personalise the theme a bit. Minimo gives you a very easy way to override
the CSS with the `customCSS` key in `config.toml`. You can read about a specific
use case [here](https://discourse.gohugo.io/t/minimo-css-customization/7173/4), where the accent colour is changed.

Changing layout content is down to Hugo, so it should be independent of the
theme. Say we want to add an additional line to the footer of our site. Simply
add files to the `layout` directory. They must be the same name as the ones your
theme uses. For Minimo, this meant [adding a footer partial](https://github.com/kwyse/personal-website/blob/41e3702fa15589739e22f64870acb9c19e9a7322/layouts/partials/footer/attribution.html) for the new content,
as well as [overriding the existing footer](https://github.com/kwyse/personal-website/blob/41e3702fa15589739e22f64870acb9c19e9a7322/layouts/partials/footer.html). You can read more about Hugo's
solution for customising your theme [here](https://gohugo.io/themes/customizing/).

Thus far, the only thing that tripped me up was Minimo requiring that one
`recentPostsLength` key to have a value. Hugo itself has been completely
transparent in what it's doing. If you look at the folder structure of our
project, you'll see every file we've added has a known purpose. If we want to
make adjustments to common configuration options, `config.toml` is a good place
to start. If we want to override specific parts of our theme, we just provide
the override in the `layouts` directory. The organisation of our site content
will correspond to the folder structure inside the `content` directory. So far,
it's all very intuitive!


## Deploying {#deploying}

I promised we'd deploy this so the whole world would know where to hunt for wild
baguettes. Hugo has a [good guide](https://gohugo.io/hosting-and-deployment/deployment-with-nanobox/) on [Nanobox](https://nanobox.io/) deployment. I hadn't heard of
Nanobox before so I did some digging. Essentially they provide a managed Docker
container for you. They are not a cloud provider and rely on you having an
account with a service such as AWS or DigitalOcean.  Once you link the accounts,
Nanobox will take care of deployments to the hosts of the cloud
provider. Nanobox itself is completely free on the basic plan. Given one of the
goals of this project was to streamline the process of actually _blogging_, this
sounded like the perfect solution.

I'll be using DigitalOcean here because I already had an account. Follow the
[Nanobox guide](https://docs.nanobox.io/providers/hosting-accounts/digitalocean/) to generate a DigitalOcean key and give Nanobox read/write access
to your account. Then launch the new app.

You'll then need to [install](https://docs.nanobox.io/install/) the Nanobox CLI locally. Nanobox needs two files for
us to tell it how to run our Hugo project: [`boxfile.yml`](https://github.com/kwyse/personal-website/blob/65791863bff9abfd4c6e430ca38d601c90d9b61c/boxfile.yml), which tells Nanobox
what commands to run, and [`install.sh`](https://github.com/kwyse/personal-website/blob/65791863bff9abfd4c6e430ca38d601c90d9b61c/install.sh), which is called by `boxfile.yml` and
actually installs Hugo inside the container. Add these files to your project,
then tell Nanobox to deploy!

```sh
$ nanobox remote add <nanobox-app-name> $ nanobox deploy
```

Visit your Nanobox portal and find a link that takes you to the deployed site!
Too easy!

The next logical step is to set up a domain name for your site. I use [NameCheap](https://www.namecheap.com/)
and have been very happy with their service. The prices are competitive and they
provide a very intuitive dashboard for adding DNS records.


## Next steps {#next-steps}

There's **loads** of places we could go from here. If you want your site to get
even marginal traffic, definitely get a domain name for it.  You'll probably
then want to look into adding TLS to your site. Even if you only serve static
content, it's always a [good idea](https://security.stackexchange.com/questions/142496/which-security-measures-make-sense-for-a-static-web-site) to enable it. After that, you can check out the
depths of [Hugo's documentation](https://gohugo.io/documentation/). We've barely scratched the surface of what it's
capable of.

This will probably be the path that I take. I'm impressed with how easy the
whole experience was, primarily due to the solid documentation and usability of
Hugo. Hopefully you'll see more content on here soon!
