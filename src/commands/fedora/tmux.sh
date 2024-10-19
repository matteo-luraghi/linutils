#!/bin/bash

# read user input for session
read -p "Insert the tmux session: " SESH

# check if session exists
tmux has-session -t $SESH 2>/dev/null

# if session doesn't exist create session
if [ $? != 0 ]; then

	# active projects
	projects=$(echo "experiments linutils notifiche-app notifiche-api py-twitch-bot" | tr ' ' '\n')

	# select a project via fzf
	selected=$(printf "$projects" | fzf)
	if [[ -z $selected ]]; then
		exit 0
	fi

	case $selected in
	"experiments")
		dir="Coding/Experiments"
		;;
	"linutils")
		dir="linutils"
		;;
	"notifiche-app")
		dir="Radio/servizi-notifiche-app"
		;;
	"notifiche-api")
		dir="Radio/servizi-notifiche-api"
		;;
	"py-twitch-bot")
		dir="Radio/py-twitch-bot"
		;;
	esac

	tmux new-session -d -s $SESH -n "nvim"

	# nvim window
	tmux send-keys -t $SESH:nvim "cd $dir" C-m
	tmux send-keys -t $SESH:nvim "nvim" C-m

	# terminal window
	tmux new-window -t $SESH -n "zsh"
	tmux send-keys -t $SESH:zsh "cd $dir" C-m

	# git window
	tmux new-window -t $SESH -n "git"
	tmux send-keys -t $SESH:git "cd $dir" C-m
	tmux send-keys -t $SESH:git "lazygit" C-m

	# select nvim window
	tmux select-window -t $SESH:nvim
fi

# attach to session
tmux attach-session -t $SESH
