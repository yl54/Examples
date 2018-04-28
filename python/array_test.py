#!/usr/bin/env python

import json

# imports of file names with "-" don't work.

# This code will test out array stuffs.
def test_array():
    arr = ["10", "15", 
            "fdafds"]
    print arr
    #arr = [a for a in arr if a is not None]
    print " ".join(arr)

def test_json():
    error = {
        "issue": 12,
        "timestamp_beginning": {
            "thing": 4,
            "other": {
                "thing2": 2
            }
        },
        "timestamp_error": 6
    }
    print json.dumps(error)

def test_boolean():
    success = True
    if success:
        print "good"
    else:
        print "bad"

def test_greater_than():
    retries = None
    attempt = 3
    if attempt <= retries:
        print "good"
    else:
        print "bad"

def test_arr_amt_boolean():
    arr = ["10", "15", "fdafds"]
    if len(arr):
        print "good"
    else:
        print "bad"


def test_string_comparison():
    str_1 = " "
    str_2 = " "
    if str_1 == str_2:      # This is best for value checking
    #if str_1 is str_2:     # This has some weird effects
        print "they are equal"
    else:
        print "they are not equal"

def test_indents():
    st = ' | '.join(['Part 1', 
                     'Part 2'])
    print st

def test_dict_get():
    error = {
        "issue": 12,
        "timestamp_beginning": {
            "thing": 4,
            "other": {
                "thing2": 2
            }
        },
        "timestamp_error": 6
    }
    error["bang"] = "big"
    thing = error.get("bangbang", None)
    if thing is None:
        print "thing is None"
    else:
        print "thing is not None"

test_dict_get()

