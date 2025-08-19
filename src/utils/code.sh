#!/bin/bash

# read user input for session
read -p "Insert the tmux session: " SESH

# check if session exists
tmux has-session -t $SESH 2>/dev/null

# if session doesn't exist create session
if [ $? != 0 ]; then

  # dynamically get directories from "Radio/IT" and "Coding" omitting directories starting with "_"
  projects=$(find Radio/IT Coding -mindepth 1 -maxdepth 1 -type d ! -name '_*')

  # select a project via fzf
  selected=$(printf "$projects" | fzf)
  if [[ -z $selected ]]; then
    exit 0
  fi

  tmux new-session -d -s $SESH -n "nvim"

  # nvim window
  tmux send-keys -t $SESH:nvim "cd $selected" C-m
  tmux send-keys -t $SESH:nvim "nvim" C-m

  # terminal window
  tmux new-window -t $SESH -n "zsh"
  tmux send-keys -t $SESH:zsh "cd $selected" C-m

  # git window
  tmux new-window -t $SESH -n "git"
  tmux send-keys -t $SESH:git "cd $selected" C-m
  tmux send-keys -t $SESH:git "lazygit" C-m

  # select nvim window
  tmux select-window -t $SESH:nvim
fi

# attach to session
tmux attach-session -t $SESH
