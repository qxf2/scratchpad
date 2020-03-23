"""
This is a sample script to connect to GitHub search API and get details about repositories
"""
from github import Github
import github_token as access
import csv
import json

def get_repo_stars(repository):
    "Return the number of stars a repo contains"
    github_obj = Github(access.TOKEN)
    repo = github_obj.get_repo(repository)
    stars = repo.stargazers_count

    return stars

def get_repo_forks(repository):
    "Return the number of forks a repo contains"
    github_obj = Github(access.TOKEN)
    repo = github_obj.get_repo(repository)
    forks = repo.forks_count

    return forks 

def get_repo_open_issues(repository):
    "Return the number of watchers a repo contains"
    github_obj = Github(access.TOKEN)
    repo = github_obj.get_repo(repository)
    open_issues = repo.open_issues_count

    return open_issues 

#----START OF SCRIPT
if __name__ == '__main__':
    repositories = ['qxf2/qxf2-page-object-model']
    for repository in repositories:
        stars = get_repo_stars(repository)
        forks = get_repo_forks(repository)
        open_issues = get_repo_open_issues(repository)
        print('{} has {} stars'.format(repository,stars))
        print('{} has {} forks'.format(repository,forks))
        print('{} has {} open_issues'.format(repository,open_issues))
        ## Write API Results to CSV
        with open('repo_count.csv', 'wb') as csvFile:
            gwriter = csv.writer(csvFile, delimiter=',')
            gwriter.writerow(["Repos", "Stars", "Forks","open issues"])
            gwriter.writerow([repositories, stars,forks,open_issues])
            