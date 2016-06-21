# webypass

*Bypass today's web heaviness and browse properly with [w3m][w3m].*

The web is fucked. Pages are just so *heavy*, so *uselessly dynamic*, it's sickening. Not only
is it sickening, but it also makes the web inaccessible to less powerful devices.

As an aesthetical experiment, I decided that my main computer would be a Raspberry Pi. All in all,
it works rather well and runs rather fast... except for [Midori][midori], which is dog slow.
Considering the complexity of rendering a modern web page, it isn't surprising, but it's sad.

There are lightweight browsers such as [w3m][w3m], but the web is now so reliant on modern CSS and
JS that most pages are simply and utterly broken.

I really don't want to load your 3mb assets to render your crappy article, I just want the content.

That's what this application does: it's a proxy server that spews pure content with proper
semantics that is easily processed by a ligtweight browser.

## Status

Very early. Will most likely panic all the time. Websites for which there's a filter:

* [Reddit](www.reddit.com): List of links for all pages that have some.

## How it works

I don't precisely know yet, but it's mostly going to be website-specific filters. Many modern
websites today hide their content behind APIs that are accessed through JS. We're going to use
the same APIs to fetch the content directly and bypass the JS crap.

### But won't you have to re-implement everything?

Not really. Most of the complexity of modern websites is because they're visually complex and
very interactive. If our goal is to properly present it in w3m, this simplifies our task a lot.

But then, I don't know yet. I want to try.

## Usage

After you've compiled with cargo, simply run `webypass` without argument. It's going to listen
on `127.0.0.1:8080`.

Then, in your browser, open `http://localhost:8080/?url=https://www.reddit.com`. You're going
to get your web page returned. If it's a supported website, the content is going to be mangled
so that it's actually usable in a text-based browser.

If it's not supported, the content will be returned as-is.

Any other request type will yield a `404`.

### What about proxying?

Proxying, that is, setting the "proxy" settings of your browser to `http://localhost:8080`, can't
work with HTTPS. This type of proxying can't properly do a TLS handshake, with reason: the goal
of SSL is to ensure that there's no eavesdropping and/or content alteration. Unfortunately, that's
precisely what this application does.

So no, it's got to be that way.

### Cookies

Because webypass' URL is `http://localhost`, it's not going to receive proper cookies from your
browser. However, when the website responds, it's going to respond with proper `Set-Cookie`. For
this reason, webypass **intercepts** all cookies, keep them in memory, and re-send them with your
proxied request the next time to hit that website.

For now, that's the only practical way I've found to do things like being logged in a website.

For now, the cookie cache is only in memory. It's never written to persistent storage.

### Privacy

Webypass has access to all the contents of your HTTP requests and it stores all your cookies. For
this reason, you should only run it **locally**. You don't want to share your instance of webypass
with anyone else.

[w3m]: http://w3m.sourceforge.net/
[midori]: http://midori-browser.org/
[hyper]: https://github.com/hyperium/hyper
