const { setup } = require('shellshot');

setup();

it(
    'checks if the basic example produces the same output',
    async () => {
      await expect.command('sh ../scripts/tests/example_basic_snapshot.sh').forStdout(exp => exp.toMatchSnapshot());
    },
);
