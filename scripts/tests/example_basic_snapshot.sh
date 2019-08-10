cargo build --example basic 2> /dev/null
tmux new-session -x 80 -y 24 -d ../target/debug/examples/basic && sleep 1
tmux capture-pane -J -p -t %0
