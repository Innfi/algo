import assert from 'assert';

const quickSort = (input: number[], lhs: number, rhs: number): void => {
    if(rhs - lhs <= 1) return;

    organize(input, lhs, rhs);

    const pivot = toPivot(lhs, rhs);

    quickSort(input, lhs, pivot-1 );
    quickSort(input, pivot+1, rhs);
}

const organize = (input: number[], lhs: number, rhs: number): void => {
    const pivotValue = input[rhs];
    let indexLhs = lhs;
    let indexRhs = rhs-1;

    while(indexLhs < indexRhs) {
        if(input[indexLhs] < pivotValue) {
            indexLhs += 1;
            continue;
        } else if(pivotValue < input[indexRhs]) {
            indexRhs -= 1;
            continue;
        } 

        //FIXME: input[index] === pivotValue

        swapElement(input, indexLhs, indexRhs);
    }

    swapElement(input, toPivot(lhs, rhs), rhs);
}

const swapElement = (input: number[], lhs: number, rhs: number): void => {
    const temp = input[rhs];
    input[rhs] = input[lhs];
    input[lhs] = temp;
}

const toPivot = (lhs: number, rhs: number) => {
    return Math.floor((rhs - lhs) / 2) + lhs;
}

describe('quicksort', () => {
    it('toPivot: whole array', () => {
        const input = [ 5, 3, 7, 6, 2, 1, 4];
        const expectedPivot = 3;

        assert.strictEqual(toPivot(0, input.length-1), expectedPivot);
    });

    it('toPivot: partial array(former)', () => {
        const input = [ 5, 3, 7, 6, 2, 1, 4];
        const expectedPivot = 1;

        assert.strictEqual(toPivot(0, 2), expectedPivot);
    });

    it('toPivot: partial array(latter)', () => {
        const input = [ 5, 3, 7, 6, 2, 1, 4];
        const expectedPivot = 5;

        assert.strictEqual(toPivot(4, 6), expectedPivot);
    });

    it('array.prototype.every', () => {
        const input = [ 1,2,3,4,5 ];
        const result: boolean = input.slice(1).every((value: number, index: number) => {
            return value > input[index];
        });

        assert.strictEqual(result, true);
    });

    it('single organize', () => {
        const input = [ 5, 3, 7, 6, 2, 1, 4];
        organize(input, 0, input.length-1);
        const output = [1, 3, 2, 4, 7, 5, 6];

        assert.deepStrictEqual(input, output);
    });

    it('quicksort: recursive', () => {
        const input = [ 5, 3, 7, 6, 2, 1, 4 ];
        quickSort(input, 0, input.length-1);

        assert.strictEqual(input.slice(1).every((v, i) => v >= input[i]), true);
    });

    it('quicksort: duplicate elements', () => {
        const input = [ 5, 3, 4, 7, 2, 1, 4 ];
        quickSort(input, 0, input.length-1);
// 1 3 4 7 2 5 4
//
        assert.strictEqual(input.slice(1).every((v, i) => v >= input[i]), true);
    });
});