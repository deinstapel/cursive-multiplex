const { setup } = require('shellshot');

setup();

function cargo_e2e(name, custom) {
    return async () => {
        await expect.command(`cargo build --bin end2end_${name}`)
            .forExitCode(exp => exp.toBe(0));
        await expect.command(
            `tmux new-session -x 80 -y 24 -d 'sh -c "TERM=xterm-256color ../target/debug/end2end_${name}"' \; set status off && sleep 0.05`
        ).forExitCode(exp => exp.toBe(0));

        if (!!custom) {
            await custom();
        }

        await expect.command('tmux capture-pane -J -p -t %0')
            .forStdout(exp => exp.toMatchSnapshot());
        await expect.command('tmux kill-server')
            .forExitCode(exp => exp.toBe(0));
    };
}

it('tests a complex pane setup', cargo_e2e('complex'));
it('tests focusing another pane', cargo_e2e('complex_focus'));
it('tests removing panes in a complex setup', cargo_e2e('complex_remove'));
it('tests resizing panes in a complex setup', cargo_e2e('complex_resize', async () => {
    await expect.command('tmux send-keys C-Down && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
    await expect.command('tmux send-keys C-Right && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
    await expect.command('tmux send-keys M-Left M-Down && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
    await expect.command('tmux send-keys C-Up && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
    await expect.command('tmux send-keys C-Left && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
}));
it('tests switching panes in a complex setup', cargo_e2e('complex_switch_views'));

it('tests moving focus down by 1 in a vertical layout', cargo_e2e('down_focus', async () => {
    await expect.command('tmux send-keys -N 1 M-Down && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
}));
it('tests moving focus down by 2 in a vertical layout', cargo_e2e('down_focus', async () => {
    await expect.command('tmux send-keys -N 2 M-Down && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
}));
it('tests moving focus down by 3 in a vertical layout', cargo_e2e('down_focus', async () => {
    await expect.command('tmux send-keys -N 3 M-Down && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
}));

it('tests the horizontal splitting', cargo_e2e('horizontal'));
it('tests the horizontal layout with fixed size child', cargo_e2e('horizontal_fixed_size'));
it('tests removing panes in a horizontal setup', cargo_e2e('horizontal_remove'));
it('tests resizing a pane in a horizontal setup', cargo_e2e('horizontal_resize', async () => {
    await expect.command('tmux send-keys C-Left && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
    await expect.command('tmux send-keys C-Left && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
    await expect.command('tmux send-keys C-Right && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));}));
it('tests switching panes in a horizontal setup', cargo_e2e('horizontal_switch_views'));

it('tests moving focus left by 1 in a horizontal layout', cargo_e2e('left_focus', async () => {
    await expect.command('tmux send-keys -N 1 M-Left && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
}));
it('tests moving focus left by 2 in a horizontal layout', cargo_e2e('left_focus', async () => {
    await expect.command('tmux send-keys -N 2 M-Left && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
}));
it('tests moving focus left by 3 in a horizontal layout', cargo_e2e('left_focus', async () => {
    await expect.command('tmux send-keys -N 3 M-Left && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
}));

it('tests moving focus right by 1 in a horizontal layout', cargo_e2e('right_focus', async () => {
    await expect.command('tmux send-keys -N 1 M-Right && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
}));
it('tests moving focus right by 2 in a horizontal layout', cargo_e2e('right_focus', async () => {
    await expect.command('tmux send-keys -N 2 M-Right && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
}));
it('tests moving focus right by 3 in a horizontal layout', cargo_e2e('right_focus', async () => {
    await expect.command('tmux send-keys -N 3 M-Right && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
}));

it('runs a basic smoke test', cargo_e2e('smoke'));

it('tests moving focus up by 1 in a vertical layout', cargo_e2e('up_focus', async () => {
    await expect.command('tmux send-keys -N 1 M-Up && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
}));
it('tests moving focus up by 2 in a vertical layout', cargo_e2e('up_focus', async () => {
    await expect.command('tmux send-keys -N 2 M-Up && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
}));
it('tests moving focus up by 3 in a vertical layout', cargo_e2e('up_focus', async () => {
    await expect.command('tmux send-keys -N 3 M-Up && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
}));

it('tests the vertical splitting', cargo_e2e('vertical'));
it('tests the vertical layout with fixed size child', cargo_e2e('vertical_fixed_size'));
it('tests removing panes in a vertical setup', cargo_e2e('vertical_remove'));
it('tests resizing a pane in a vertical setup', cargo_e2e('vertical_resize', async () => {
    await expect.command('tmux send-keys C-Up && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
    await expect.command('tmux send-keys C-Up && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
    await expect.command('tmux send-keys C-Down && sleep 0.1')
        .forExitCode(exp => exp.toBe(0));
}));
it('tests switching panes in a vertical setup', cargo_e2e('vertical_switch_views'));
