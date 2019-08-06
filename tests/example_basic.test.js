const { setup } = require('shellshot');

setup();

it(
    'checks if the basic example produces the same output',
    async () => {
        await expect.command('timeout 1 script --quiet --command ../target/debug/examples/basic /dev/null')
            .withEnv({
                TERM: "xterm-256color",
                LINES: 24,
                COLUMNS: 80,
            })
            .forStdout(expectation => expectation.toMatchSnapshot());
    },
);
