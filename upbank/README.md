# UpBank Client

This library is a client for [UpBank's API](https://developer.up.com.au/). It
provides two methods of interacting with the API, the first and most simple is
by providing a client that maps directly to the UpBank API, in particular all
the objects map directly to those returned by the UpBank API. Then, on top of
that, it provides an abstraction layer that wraps things like currency up
nicely.