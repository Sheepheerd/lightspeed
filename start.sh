#!/bin/bash

SESSION="rust"

tmux new-session -d -s $SESSION -n 'main' || {
    echo "Failed to create tmux session."
    exit 1
}

tmux new-window -t $SESSION -n 'ra-multiplex'
tmux send-keys -t $SESSION:'ra-multiplex' 'ra-multiplex server' C-m || {
    echo "Failed to run ra-multiplex in the new window."
    exit 1
}

tmux select-window -t $SESSION:main

tmux attach-session -t $SESSION || {
    echo "Failed to attach to tmux session."
    exit 1
}
