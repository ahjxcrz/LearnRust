############
# Jan. 27, 2023
# by Dr CAO
# check the number of commits for my github
############

import os
import sys

import requests
from urllib.parse import parse_qs, urlparse

def get_commits_count(owner_name: str, repo_name: str) -> int:
    """
    Returns the number of commits to a GitHub repository.
    """
    url = f"https://api.github.com/repos/{owner_name}/{repo_name}/commits?per_page=1"
    r = requests.get(url)
    links = r.links
    rel_last_link_url = urlparse(links["last"]["url"])
    rel_last_link_url_args = parse_qs(rel_last_link_url.query)
    rel_last_link_url_page_arg = rel_last_link_url_args["page"][0]
    commits_count = int(rel_last_link_url_page_arg)
    return commits_count


def main():
    owner_name = "ahjxcrz"
    repo_name = "LearnRust"
    if len(sys.argv)>=3:
       # by default, we just use my own github as example
       owner_name = sys.argv[1]
       repo_name = sys.argv[2] 
    print("now we are checking the owner name: "+owner_name + ", repo name : "+repo_name)
    totalC = get_commits_count(owner_name,repo_name)
    print("by today, the number of commits from the github is "+str(totalC))


main()
