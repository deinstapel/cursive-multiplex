const { setup } = require('shellshot');

setup();

it(
    'checks if the basic example produces the same output',
    async () => {
        await expect.command('script -qc "cargo run --example basic" out || less out | sha1sum')
          .forStdout(exp => exp.toMatchSnapshot());
    },
);
