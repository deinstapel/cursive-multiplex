const { setup } = require('shellshot');

setup();

async function cargo_e2e(name) {
    await expect.command(`cargo build --bin end2end_${name}`)
        .forExitCode(exp => exp.toBe(0));
    await expect.command(
        `tmux new-session -x 80 -y 24 -d ../target/debug/end2end_${name} \; set status off && sleep 0.05`
    ).forExitCode(exp => exp.toBe(0));
    await expect.command('tmux capture-pane -J -p -t %0')
        .forStdout(exp => exp.toMatchSnapshot());
    await expect.command('tmux kill-server')
        .forExitCode(exp => exp.toBe(0));
}

it('runs a basic smoke test', async () => await cargo_e2e('smoke'));
it('tests the horizontal splitting', async () => await cargo_e2e('horizontal'));
it('tests the vertical splitting', async () => await cargo_e2e('vertical'));
it('tests the horizontal layout with fixed size child',
   async () => await cargo_e2e('horizontal_fixed_size'));
it('tests the vertical layout with fixed size child',
   async () => await cargo_e2e('vertical_fixed_size'));
it('tests a complex pane setup', async () => await cargo_e2e('complex'));
it('tests switching panes in a horizontal setup',
   async () => await cargo_e2e('horizontal_switch_views'));
it('tests switching panes in a vertical setup',
   async () => await cargo_e2e('vertical_switch_views'));
it('tests switching panes in a complex setup',
   async () => await cargo_e2e('complex_switch_views'));
it('tests removing panes in a horizontal setup',
   async () => await cargo_e2e('horizontal_remove'));
it('tests removing panes in a vertical setup',
   async () => await cargo_e2e('vertical_remove'));
it('tests removing panes in a complex setup',
   async () => await cargo_e2e('complex_remove'));
