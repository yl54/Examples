#!/usr/bin/env python

# This file tests out json stuff.

import json
import datetime as dt

dt_fmt = "%m/%d/%Y %H:%M:%S"

# strftime = make a string from datetime
# strptime = make a datetime from string

# Function to make a json object.
def make_dict():
    return {
        "issue": "Bad stuff happenned.",
        "timestamp_error": dt.datetime.now().strftime(dt_fmt)
    }

# Function to dump to a file.
def dump_to_file(file_name, dictd):
    f = open(file_name, 'w+')
    f.write(json.dumps(dictd) + "\n")
    f.write(json.dumps(dictd) + "\n")
    f.close()
    f = open(file_name, 'a')
    f.write(json.dumps(dictd) + "\n")
    f.close()

def test_replace_json():
    al = open("").read()
    print al.replace("\n"," ")

def test_load_bad_json():
    json.load(open(""))

test_load_bad_json()