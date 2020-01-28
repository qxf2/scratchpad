"""
This is a sample script to connect to GitHub search API and get details about repositories
"""
from github import Github
import github_token as access

def get_repo_stars(repository):
    "Return the number of stars a repo contains"
    github_obj = Github(access.TOKEN)
    repo = github_obj.get_repo(repository)
    stars = repo.stargazers_count

    return stars 

#----START OF SCRIPT
if __name__ == '__main__':
    repositories = ['techgaun/github-dorks','UnkL4b/GitMiner','michenriksen/gitrob','PyGithub/PyGithub']
    for repository in repositories:
        stars = get_repo_stars(repository)
        print('{} has {} stars'.format(repository,stars))
