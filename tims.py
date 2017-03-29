# Script to fill one system
# Public Domain

#
# run pip install robobrowser requests-ntlm
# create config.json as
# { "user" : "yourusername", "password" : "YourS3cret!Password"}
#

from robobrowser import RoboBrowser
from requests_ntlm import HttpNtlmAuth
import json

# config data
URL = "http://fpg.etn.com/engportfolio/TimeReportHours.aspx"

with open ("config.json", "rt") as fp:
  cfg = json.load (fp)

USER = cfg ["user"]
PASSWORD = cfg ["password"]

br = RoboBrowser (history=True)
br.open (URL, auth=HttpNtlmAuth ("EURO\\%s" % USER, PASSWORD))

# TODO: analyze HTML and figure out how to fill and submit the form
