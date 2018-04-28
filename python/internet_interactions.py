#!/usr/bin/env python

# Practice with urllib2.

import urllib2

URL = "http://www.espn.com"
TIMEOUT = 30 # Measured in seconds.

def test_connection():
    req = urllib2.Request(URL)
    req.add_header('Accept', 'json;version=2')
    req.add_header('Content-Type', 'application/json')
    response = urllib2.urlopen(req, timeout=TIMEOUT)

    if response is None:
        print "response is None."
        return

    # returns a string url
    response_url = response.geturl()

    # returns a dict object of metadata
    response_info = response.info()

    # returns an int code
    response_code = response.getcode()

    print response_url
    print "------------------------------------------------------"
    print response_info["Date"]
    print "------------------------------------------------------"
    print response_code + 1

test_connection()