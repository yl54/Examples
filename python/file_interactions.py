#!/usr/bin/env python

# Practice opening/writing/closing files in python.

import os

"""
open options:
'r'  - read mode
'w'  - write mode
'a'  - appending mode
'r+' - special read and write mode
'w+' - read and write, creates file if not created
     - truncated if it exists
"""

# def make_dir():


def file_interact():
    file_name = ""
    file_contents = "This is a test."
    f = open(file_name, 'r')
    f.write(file_contents + "\n")
    f.write(file_contents + "\n")
    f.close()

def file_append():
    file_name = ""
    file_contents = "This is a test."
    f = open(file_name, 'a')
    f.write(file_contents + "\n")
    f.close()

# LESSON: exists is a relative thing.
# Q: is there an exists for like absolute paths?
def check_exists():
    # Check what the absolute path has to say
    check_exists_helper('')
    # Check what the relative path has to say
    check_exists_helper('')
    check_exists_helper('')
    check_exists_helper('') # fold and fold/ are ok. /fold is not

def check_exists_helper(file_name):
    exists = os.path.exists(file_name)
    isdir = os.path.isdir(file_name)
    print 'file name: ' + file_name +  \
          ' | exists: ' + str(exists) +  \
          ' | isdir: ' + str(isdir)

# So abspath checks if the string has a forward slash at the front. If it 
#   does, then it does nothing to the path name.
# if the string doesn't have a forward slash, then it takes the working 
#   directory + the path you provide, and join them together.
# Basically the key is if the backslash is at the front or not
# If you end with a '/' at the end, then it will remove that '/'
def check_abs_path():
    file_name = ''
    abs_file_name = os.path.abspath(file_name)
    print 'touched file name: ' + abs_file_name
    # f = open(abs_file_name, 'w+')
    # f.close()
    #abs_file_name = os.path.abspath(file_name)
    #print 'touched file name: ' + file_name

def check_abs_path_join():
    directory_name = ''
    file_name = ''
    print 'file path join: ' + os.path.join(directory_name, file_name, 'thing')

check_abs_path()

