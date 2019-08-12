const { setup } = require('shellshot');

setup();

it(
    'checks if the basic example produces the same output',
    async () => {
        await expect.command('cargo build --example basic --verbose')
            .forExitCode(exp => exp.toBe(0));
        await expect.command('../tmux-2.9a/tmux new-session -x 80 -y 24 -d ../target/debug/examples/basic \; set status off && sleep 1')
            .forExitCode(exp => exp.toBe(0));
        await expect.command('../tmux-2.9a/tmux capture-pane -J -p -t %0')
            .forStdout(exp => exp.toMatchSnapshot());
    },
);
