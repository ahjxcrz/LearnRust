############
# Jan. 28, 2023
# by Dr CAO
# check all repos for the github org
############

import os
import sys

import requests
from urllib.parse import parse_qs, urlparse

def get_repos(repo_link: str) -> list:
    """
    Returns all repos to a GitHub repository.
    """
    headers = {'Accept': 'application/json'}   # may need Authorization: Bearer OAUTH-TOKEN   
    if "repos" not in repo_link:
        theORGName = repo_link.split("/")[-1]
        repo_link = "https://api.github.com/orgs/"+ str(theORGName) + "/repos"
    r = requests.get(repo_link,headers=headers)
    data = r.json()
    links = []
    #print(data)
    for i in range(0,len(data)):
       if "html_url" not in data[i]: 
          print("skiping .. checking " + data[i])
          continue
       myl = data[i]["html_url"]
       tem = myl.split("/")
       oN = tem[-2]
       oR = tem[-1]
       #print(myl)
       #print(oN)
       #print(oR)
       mylC = get_commits_count(oN, oR)
       links.append((myl,mylC))
    return links


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
    print("Warning! This script may fail because there is a limit from github API, it is not really the problem of this script, you would need to use your token to have higher limit")
    repo_link = "https://github.com/orgs/axelarnetwork"
    if len(sys.argv)>=2:
       # by default, we just use automata network as example
       repo_link = sys.argv[1]
    print("Now checking the repo of the organization: "+repo_link)
    totalRep = get_repos(repo_link)
    print(totalRep)

main()
