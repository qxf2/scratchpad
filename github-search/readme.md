This session was to show the following:

a) how to approach a problem that needs new tooling
b) how to use PyGithub to find the stars in a project

### SETUP

1. `pip install PyGithub`
2. Create a Github token with only read access at  https://github.com/settings/tokens 
3. Create a github_token.py and set a variable called `TOKEN` to be equal to your access token
4. `python sample_github_search.py`
5. Output should contain the stars for 4 GitHub repos listed

### Meta thinking

_a) Search for how people have already solved your problem_
* GitHub Dork    https://github.com/techgaun/github-dorks
* Surch https://cloudify.co/blog/we-created-an-awesome-search-tool-for-github-repos-in-python/
* Git Miner https://github.com/UnkL4b/GitMiner
* Git Rob: https://github.com/michenriksen/gitrob
* PyGitHub: https://github.com/PyGithub/PyGithub

_b) Which is popular?_

Look at
* Stars
* Contributors
* Forks
* Branches
* Number of commits

_c) Which is easy to start?_

Spend no more than 2-3 minutes per repo

_d) A real problem_

Lets count stars

_e) Breakdown and re-apply the meta_ 

    -> cloning repo
    -> install
    > search for the ??
    -> create a virtualenv
    -> redoing an example of the readme
    -> lookup how to find stars
    -> setup access 
    -> name the file 
    -> list data for input
    -> expected output 

_f) Summarize (1-liner,5-liner,para)_

Spend 15-minutes at least! This is where you learn to make connections and figure out what you need to dive in deeper.

_g) New ideas_

    > credentials getting exposed - periodic audit
    > number of contributors .... targeted analysis
    > how frequently a project is being updated 
    > useful on clients too to see patterns
    > automatic commits and decommits 

_h) Deeper background_

    > Github APIs

_i) Revisit occasionally_
