import assert from 'assert';

import { FieldStatus, HORIZONTAL_LIMIT, LineStatus, Runner, VERTICAL_LIMIT, createNewFields, toDown, toLeft, toRight, toUp } from '../src/main';

describe('building blocks', () => {
  it ('toLeft: normal case', () => {
    const result = toLeft({ x: 1, y: 1});

    assert.strictEqual(result.resultType, 'success');
    assert.strictEqual(result.resultPos.x, 0);
    assert.strictEqual(result.resultPos.y, 1);
  });

  it ('toLeft: error case', () => {
    const result = toLeft({ x: 0, y: 1});

    assert.strictEqual(result.resultType, 'fail');
  });

  it ('toRight: normal case', () => {
    const result = toRight({ x: 15, y: 1});

    assert.strictEqual(result.resultType, 'success');
    assert.strictEqual(result.resultPos.x, 16);
    assert.strictEqual(result.resultPos.y, 1);
  });

  it ('toRight: error case', () => {
    const result = toRight({ x: HORIZONTAL_LIMIT, y: 1 });

    assert.strictEqual(result.resultType, 'fail');
  });

  it('toUp: normal case', () => {
    const result = toUp({ x: 9, y: 9 });

    assert.strictEqual(result.resultType, 'success');
    assert.strictEqual(result.resultPos.x, 9);
    assert.strictEqual(result.resultPos.y, 8);
  });

  it('toUp: error case', () => {
    const result = toUp({ x: 9, y: 0 });

    assert.strictEqual(result.resultType, 'fail');
  });

  it('toDown: normal case', () => {
    const result = toDown({ x: 9, y: 9 });

    assert.strictEqual(result.resultType, 'success');
    assert.strictEqual(result.resultPos.x, 9);
    assert.strictEqual(result.resultPos.y, 10);
  });

  it('toDown: error case', () => {
    const result = toDown({ x: 9, y: VERTICAL_LIMIT });

    assert.strictEqual(result.resultType, 'fail');
  });

});

describe('line status', () => {
  it('horizontal initial: 1 stone', () => {
    
  });
});

describe('ohmock', () => {
  it ('check condition from last position', () => {
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

  it ('getLineStatus] aborts against invalid stone count', () => {
    const fields = createNewFields(19, 19);

    fields[4][9] = 'O';
    fields[5][9] = 'O';
    fields[6][9] = 'O';
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

    const suggestion = runner.getLineStatus(
      fieldStatus, 
      'O',
      toLeft,
      toRight,
    );

    assert.strictEqual(suggestion.status, LineStatus.INVALID_GAME_STATE);
  });

  it ('getLineStatus] normal case for vertical line', () => {
    const fields = createNewFields(19, 19);

    fields[9][7] = 'X';
    fields[9][8] = 'X';
    fields[9][9] = 'X';

    const fieldStatus: FieldStatus = {
      fields,
      lastStonePosition: {
        x: 9,
        y: 9,
      },
    };

    const runner = new Runner();

    const { status, possiblePosition } = runner.getLineStatus(
      fieldStatus, 
      'X',
      toUp,
      toDown,
    );

    assert.strictEqual(status, LineStatus.NEED_BLOCK);
    assert.strictEqual(possiblePosition.x, 9);
    assert.strictEqual(possiblePosition.y, 6);
  });
});