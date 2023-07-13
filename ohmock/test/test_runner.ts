import assert from 'assert';

import { FieldStatus, Runner, createNewFields } from '../src/main';

describe('ohmock', () => {
  it('initial', () => {
    assert.strictEqual(1, 1);
  });

  it('check condition from last position', () => {
    const fields = createNewFields(19, 19);

    fields[7][9] = 'O';
    fields[8][9] = 'O';
    fields[9][9] = 'O';

    const fieldStatus: FieldStatus = {
      fields,
      lastStonePosition: {
        x: 9,
        y: 9,
      },
    };

    const runner = new Runner();
    const result = runner.findConditionHorizontal(fieldStatus, 'O');

    assert.strictEqual(result !== undefined, true);
    assert.strictEqual(result!.x, 6);
    assert.strictEqual(result!.y, 9);
  });
});