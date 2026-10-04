#!/bin/bash

# 1. Gather candidate project directories (expand paths relative to $HOME)
SEARCH_DIRS=("$HOME/Radio/IT" "$HOME/Coding" "$HOME/Documents/TESI")

VALID_DIRS=()
for d in "${SEARCH_DIRS[@]}"; do
  [[ -d "$d" ]] && VALID_DIRS+=("$d")
done

if [[ ${#VALID_DIRS[@]} -eq 0 ]]; then
  echo "Error: None of the target project directories exist."
  exit 1
fi

projects=$(find "${VALID_DIRS[@]}" -mindepth 1 -maxdepth 1 -type d ! -name '_*')

# 2. Select directories using fzf (TAB to mark/unmark, Enter to confirm)
selected_dirs=$(printf "%s\n" "$projects" | fzf -m --prompt="Select project(s) [TAB to mark]: ")

if [[ -z "$selected_dirs" ]]; then
  echo "No directories selected. Aborting."
  exit 0
fi

# 3. Helper function to create the custom dev session
create_project_session() {
  local session_name="$1"
  local project_path="$2"

  if tmux has-session -t "$session_name" 2>/dev/null; then
    echo "Session '$session_name' already exists. Skipping creation."
    return
  fi

  # Create session starting directly inside project directory
  tmux new-session -d -s "$session_name" -c "$project_path" -n "nvim"
  tmux send-keys -t "$session_name:nvim" "nvim" C-m

  # Terminal window
  tmux new-window -t "$session_name" -c "$project_path" -n "zsh"

  # Git window
  tmux new-window -t "$session_name" -c "$project_path" -n "git"
  tmux send-keys -t "$session_name:git" "lazygit" C-m

  # Focus back to nvim window
  tmux select-window -t "$session_name:nvim"
}

# 4. Create a session for every marked directory
first_session=""

while IFS= read -r dir; do
  [[ -z "$dir" ]] && continue

  session_name=$(basename "$dir" | tr '.' '_')
  create_project_session "$session_name" "$dir"

  if [[ -z "$first_session" ]]; then
    first_session="$session_name"
  fi
done <<< "$selected_dirs"

# 5. Always create the additional "debug" session
if ! tmux has-session -t "debug" 2>/dev/null; then
  tmux new-session -d -s "debug" -n "term"
fi

# 6. Attach or switch to the first project created
target_session="${first_session:-debug}"

if [[ -n "$TMUX" ]]; then
  tmux switch-client -t "$target_session"
else
  tmux attach-session -t "$target_session"
fi
