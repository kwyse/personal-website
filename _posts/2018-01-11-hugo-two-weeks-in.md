---
layout: post
title: "Hugo, Two Weeks In"
date: 2018-01-11T20:23:33Z
category: softdev
---

> *2019/01*: Since this publication, this blog has migrated from Hugo to
Jekyll. Whilst this does not make the techniques described here invalid, it
does mean that what you currently see does not match them.

My [last post]({% post_url 2017-12-28-hail-hugo %}) about Hugo focused on
getting it set up. It finished with a few ideas on improvements. Maybe you've
noticed a few implemented since then =]

## One-liner improvements

Some of these were very simple to add. The theme I'm using,
[Minimo](https://minimo.netlify.com/), supports adding a reading time to posts
with one addition to `config.toml`.

```toml
showReadingTime = true
```

It's a small addition yet adds noted value for the reader.

The next change instead provides insight for the sight maintainer:

```toml
googleAnalytics = "[your Google Analytics tracking ID]"
```

Once your site starts getting some traffic, [Google
Analytics](https://analytics.google.com/) can help you create more targeted
content.

I've also enabled my RSS feed to display the full contents of an article
rather than just the summary. You can do that by overriding the default
`rss.xml` file (add your own `layouts/_default/rss.xml` and it will take
precedence) and changing

{% raw %}
```xml
<description>{{ .Summary | html }}</description>
```
{% endraw %}

to

{% raw %}
```xml
<description>{{ .Content | html }}</description>
```
{% endraw %}

Kudos to Brian Wisti for his
[post](https://randomgeekery.org/2017/09/15/full-content-hugo-feeds/) on this
technique.

The final quick addition is taxonomies. I've started a series called _This Week
I Discovered_ where, each week, I plan to discuss something cool I found in the
last seven days. These posts should naturally be grouped together. Hugo calls
such a grouping a
[*taxonomy*](https://gohugo.io/content-management/taxonomies/). Two taxonomies
are provided for us: *categories* and *tags*, but we can easily add our own as
well, such as a *series* taxonomy.

Enabling a taxonomy requires the name of it to be included in at least one
post's front matter:

```yaml
series: ["This Week I Discovered"]
```

## Somewhat effortful improvements

Adding a comments section could have been easy. Hugo supports
[Disqus](https://disqus.com/) comments out of the box. In your `config.toml`,
simply add:

```toml
disqusShortname = "[your Disqus short name]"
```

I ended up going this route, but there are alternatives.
[Staticman](https://staticman.net/), for example, doesn't require the comments
to be stored by a third-party. It's much more appropriate for a static-content
site.  Staticman listens for incoming `POST` requests that such a site would
generate and then creates a pull request to the repository with the comment
contents. It can be configured to merge the PRs immediately or delegate to the
repository owner, which provides comment moderation. It's a really nice idea.

The problem is that it the comments form does not look that great in Minimo. I
spent some time tweaking it but form design is not my forte. I'd like to move to
Staticman in the future if a designer will alleviate me of that burden ;)
Suffice to say, the comments section took some time to investigate and play
around with.

The biggest pain point was TLS, and it could have been so easy! The lesson here
is to always read the documentation. Each day, I would spend a bit of time
trying to find out why my site was so slow to initially load. The first
incarnation of this site didn't have this problem, and was hosted on the same
DigitalOcean setup, so I ruled that out. It seemed like a DNS lookup issue,
because performance was good once the site was loaded, but I couldn't tell if
this was on Namecheap's side or something I had misconfigured with Nanobox
given it was my first time using it.

The investigation led me discovering that [cURL can provide
timings](https://blog.josephscott.org/2011/10/14/timing-details-with-curl/)!
There's also an [awesome little
script](https://github.com/mat/dotfiles/blob/master/bin/curlt) to save you from
having to type out the long version. It confirmed immediately that the issue was
indeed DNS lookup. I took a closer look at my Namecheap DNS configuration and
found that I was redirecting traffic to the TLS version of the site. Nginx,
running inside my Nanobox, wasn't playing nice with this setup.  Then I took
another look at the
[README](https://github.com/nanobox-io/nanobox-engine-static/blob/master/README.md)
for the Nanobox static site engine and saw that forcing HTTPS was supported at
this level!

```yaml
force_https: true
```

Well I felt silly, but now the whole site is TLS enabled and loading within a
few hundred milliseconds. Much better than the ~75 seconds it was previously
taking!

The final task, and arguably the one with the biggest benefit, was to set up
automated deployments when pushing to GitHub. If you have a Travis CI account
and continuous integration enabled for the repository, simply add a
`.travis.yml` file with the following:

```yaml
sudo: required
install: sudo bash -c "$(curl -fsSL https://s3.amazonaws.com/tools.nanobox.io/bootstrap/ci.sh)"

script:
  - nanobox remote add [your Nanobox project name]
  - nanobox deploy
```

Travis CI configuration files can be customised much more than this, but it
suffices for the simple use case. Now the site will automatically deploy itself
whenever pushing!

## Endless possibilities

One can never be truly done with a project like this. Outside of creating
content, there's always ways to improve the site itself. I think the main points
are covered now, but hopefully this is only the start of side projects for this
side project.
