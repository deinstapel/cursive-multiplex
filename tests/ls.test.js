const { setup } = require('shellshot');

setup();

it(
    'checks if the files in this directory are the same',
    async () => {
        await expect.command('ls')
            .forStdout(expectation => expectation.toMatchSnapshot());
    },
);
