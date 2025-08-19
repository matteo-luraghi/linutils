#!/usr/bin/env bash

# list of languages and utils to look for
languages=$(echo "golang lua cpp c typescript rust nodejs javascript python bash php haskell" | tr ' ' '\n')
core_utils=$(echo "xargs find mv sed awk grep kill tar ssh cargo docker docker-compose chmod chown make" | tr ' ' '\n')

# select via fzf
selected=$(printf "$languages\n$core_utils" | fzf)
if [[ -z $selected ]]; then
	exit 0
fi

# read user input for query
read -p "query: " query

if printf $languages | grep -qs $selected; then
	query=$(echo $query | tr ' ' '+')
	tmux neww bash -c "echo \"curl cht.sh/$selected/$query/\" & curl cht.sh/$selected/$query & while [ : ]; do sleep 1; done"
else
	tmux neww bash -c "curl -s cht.sh/$selected~$query | less"
fi
